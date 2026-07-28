//! REST unit tests with a mock HTTP backend.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use hermers::rest::config::{Config, BASE_URL};
use hermers::rest::crypto::{generate_key, hash_key, prefix_key};
use hermers::rest::error::HermesError;
use hermers::rest::http::{Backend, Request, Response};
use hermers::{Hermes, HermesError as RootErr};

fn whoami_json() -> Vec<u8> {
    br#"{
        "hex": "A0S",
        "user": "U0X",
        "tenant": "T0X",
        "owner": true,
        "scopes": [],
        "deny": [],
        "tier": "free",
        "ip": "",
        "agent": ""
    }"#
    .to_vec()
}

struct MockBackend {
    /// Recorded request bodies (JSON text).
    bodies: Arc<Mutex<Vec<String>>>,
    /// Optional status override for non-whoami paths.
    fail: Arc<Mutex<Option<(u16, Vec<u8>)>>>,
}

impl MockBackend {
    fn new() -> Self {
        Self {
            bodies: Arc::new(Mutex::new(Vec::new())),
            fail: Arc::new(Mutex::new(None)),
        }
    }
}

impl Backend for MockBackend {
    fn execute<'a>(
        &'a self,
        req: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response, HermesError>> + Send + 'a>> {
        Box::pin(async move {
            if req.url.contains("/auth/whoami") {
                return Ok(Response {
                    status: 200,
                    status_text: "OK".into(),
                    body: whoami_json(),
                });
            }
            if let Some(body) = req.body {
                self.bodies
                    .lock()
                    .unwrap()
                    .push(String::from_utf8_lossy(&body).into_owned());
            }
            if let Some((status, body)) = self.fail.lock().unwrap().clone() {
                return Ok(Response {
                    status,
                    status_text: "Error".into(),
                    body,
                });
            }
            // Default OK empty page / hex ack
            let body = if req.url.contains("/tenant/keys") && req.method == "POST" {
                br#"{"hex":"k1"}"#.to_vec()
            } else if req.url.contains("/tenant/webhooks") && req.method == "POST" {
                br#"{"hex":"W0X"}"#.to_vec()
            } else if (req.method == "PATCH" || req.method == "DELETE")
                && req.url.contains("/tenant/webhooks")
            {
                br#"{"ok":true}"#.to_vec()
            } else if req.method == "GET" {
                br#"{"items":[],"total":0}"#.to_vec()
            } else {
                b"null".to_vec()
            };
            Ok(Response {
                status: 200,
                status_text: "OK".into(),
                body,
            })
        })
    }
}

#[tokio::test]
async fn defaults_to_production_base() {
    let backend = Arc::new(MockBackend::new());
    let hermes = Hermes::with_backend("hm_live_testkey", Config::default(), backend).unwrap();
    assert_eq!(hermes.http.api_base, BASE_URL);
    assert_eq!(BASE_URL, "https://hermers.aduki.pro/v1");
    assert!(matches!(Hermes::new(""), Err(e) if e.code == "invalid_api_key"));
    let _ = RootErr::new("x", 0, "y"); // type re-export sanity
}

#[tokio::test]
async fn hashes_api_keys_client_side() {
    let key = generate_key();
    assert!(key.starts_with("hm_live_"));
    assert_eq!(key.len(), 72);
    assert_eq!(prefix_key(&key), &key[..16]);
    assert_eq!(hash_key(&key).len(), 64);
}

#[tokio::test]
async fn throws_hermes_error_from_flat_envelope() {
    let backend = Arc::new(MockBackend::new());
    *backend.fail.lock().unwrap() = Some((
        404,
        br#"{"error":"not_found","message":"missing"}"#.to_vec(),
    ));
    let hermes = Hermes::with_backend(
        "hm_live_abc",
        Config {
            api_base: Some("https://example.test/v1".into()),
        },
        backend,
    )
    .unwrap();
    let err = hermes.contacts.list(None).await.unwrap_err();
    assert_eq!(err.code, "not_found");
    assert_eq!(err.message, "missing");
    assert_eq!(err.status, 404);
}

#[tokio::test]
async fn create_key_posts_hash_and_prefix_only() {
    let backend = Arc::new(MockBackend::new());
    let bodies = Arc::clone(&backend.bodies);
    let hermes = Hermes::with_backend(
        "hm_live_admin",
        Config {
            api_base: Some("https://example.test/v1".into()),
        },
        backend,
    )
    .unwrap();
    let fixed = generate_key();
    let created = hermes
        .keys
        .create("ci", &["contacts:read".into()], Some(&fixed), None, None)
        .await
        .unwrap();
    assert_eq!(created.key, fixed);
    let body: HashMap<String, serde_json::Value> =
        serde_json::from_str(&bodies.lock().unwrap()[0]).unwrap();
    assert_eq!(body["hash"], hash_key(&fixed));
    assert_eq!(body["prefix"], prefix_key(&fixed));
    assert!(!body.contains_key("key"));
}

#[tokio::test]
async fn caches_whoami_fields() {
    let backend = Arc::new(MockBackend::new());
    let hermes = Hermes::with_backend(
        "hm_live_abc",
        Config {
            api_base: Some("https://example.test/v1".into()),
        },
        backend,
    )
    .unwrap();
    let id = hermes.ready().await.unwrap();
    assert_eq!(id.ip.as_deref(), Some(""));
    assert_eq!(id.agent.as_deref(), Some(""));
    assert_eq!(id.owner, Some(true));
    assert_eq!(id.user, "U0X");
    assert_eq!(id.tenant, "T0X");
    assert!(hermes.me().is_some());
}

#[tokio::test]
async fn authorization_header_is_key_scheme() {
    struct CaptureAuth {
        auth: Arc<Mutex<Option<String>>>,
    }
    impl Backend for CaptureAuth {
        fn execute<'a>(
            &'a self,
            req: Request,
        ) -> Pin<Box<dyn Future<Output = Result<Response, HermesError>> + Send + 'a>> {
            Box::pin(async move {
                *self.auth.lock().unwrap() = req.headers.get("Authorization").cloned();
                Ok(Response {
                    status: 200,
                    status_text: "OK".into(),
                    body: whoami_json(),
                })
            })
        }
    }
    let auth = Arc::new(Mutex::new(None));
    let backend = Arc::new(CaptureAuth {
        auth: Arc::clone(&auth),
    });
    let hermes = Hermes::with_backend(
        "hm_live_secret",
        Config {
            api_base: Some("https://example.test/v1".into()),
        },
        backend,
    )
    .unwrap();
    let _ = hermes.ready().await.unwrap();
    assert_eq!(
        auth.lock().unwrap().as_deref(),
        Some("Key hm_live_secret")
    );
}

#[tokio::test]
async fn create_webhook_posts_and_patches() {
    let backend = Arc::new(MockBackend::new());
    let bodies = Arc::clone(&backend.bodies);
    let hermes = Hermes::with_backend(
        "hm_live_admin",
        Config {
            api_base: Some("https://example.test/v1".into()),
        },
        backend,
    )
    .unwrap();
    let created = hermes
        .tenant
        .create_webhook(&serde_json::json!({
            "url": "https://hooks.example/h",
            "secret": "whsec_0123456789abcdef",
            "events": ["message.sent"],
        }))
        .await
        .unwrap();
    assert_eq!(created.hex, "W0X");
    let body: serde_json::Value = serde_json::from_str(&bodies.lock().unwrap()[0]).unwrap();
    assert_eq!(body["url"], "https://hooks.example/h");
    hermes.tenant.active_webhooks(None).await.unwrap();
    hermes
        .tenant
        .webhook_subscribers("message.sent", None)
        .await
        .unwrap();
    hermes
        .tenant
        .update_webhook_active("W0X", false)
        .await
        .unwrap();
    hermes
        .tenant
        .update_webhook_url("W0X", "https://hooks.example/v2")
        .await
        .unwrap();
    hermes.tenant.delete_webhook("W0X").await.unwrap();
}

#[tokio::test]
async fn covers_new_rest_surfaces() {
    struct Capture {
        urls: Arc<Mutex<Vec<(String, String)>>>,
    }
    impl Backend for Capture {
        fn execute<'a>(
            &'a self,
            req: Request,
        ) -> Pin<Box<dyn Future<Output = Result<Response, HermesError>> + Send + 'a>> {
            Box::pin(async move {
                if req.url.contains("/auth/whoami") {
                    return Ok(Response {
                        status: 200,
                        status_text: "OK".into(),
                        body: whoami_json(),
                    });
                }
                self.urls
                    .lock()
                    .unwrap()
                    .push((req.method.clone(), req.url.clone()));
                let body = if req.url.contains("/feeds/") && req.url.ends_with("/sync") {
                    br#"{"hex":"F0X","ok":true}"#.to_vec()
                } else if req.url.contains("/tenant/keys/lookup/prefix") {
                    br#"{"hex":"K0X","name":"ci","prefix":"hm_live_abc","active":true,"created":"2026-01-01T00:00:00","tenant":{"hex":"T0X","name":"Acme"}}"#.to_vec()
                } else if req.method == "GET" {
                    br#"{"items":[],"total":0}"#.to_vec()
                } else {
                    br#"{"ok":true}"#.to_vec()
                };
                Ok(Response {
                    status: 200,
                    status_text: "OK".into(),
                    body,
                })
            })
        }
    }
    let urls = Arc::new(Mutex::new(Vec::new()));
    let hermes = Hermes::with_backend(
        "hm_live_admin",
        Config {
            api_base: Some("https://example.test/v1".into()),
        },
        Arc::new(Capture {
            urls: Arc::clone(&urls),
        }),
    )
    .unwrap();
    hermes.user.active_sessions(None).await.unwrap();
    hermes.user.sessions_by_method("key", None).await.unwrap();
    hermes.tenant.audits(None).await.unwrap();
    hermes.tenant.promote().await.unwrap();
    hermes.tenant.active_domains(None).await.unwrap();
    hermes
        .feeds
        .sync("F0X")
        .await
        .unwrap();
    hermes
        .mail
        .folder_unread("INBOX", None)
        .await
        .unwrap();
    hermes.keys.list_expired().await.unwrap();
    hermes.keys.lookup_prefix("hm_live_abc").await.unwrap();
    hermes
        .scheduling
        .active_appointments(None)
        .await
        .unwrap();

    let got = urls.lock().unwrap().clone();
    assert!(got.iter().any(|(m, u)| m == "GET" && u.ends_with("/user/sessions") && !u.contains("/active")));
    assert!(got.iter().any(|(_, u)| u.ends_with("/user/sessions/method/key")));
    assert!(got.iter().any(|(_, u)| u.ends_with("/tenant/audits")));
    assert!(got.iter().any(|(m, u)| m == "POST" && u.ends_with("/tenant/promote")));
    assert!(got.iter().any(|(_, u)| u.ends_with("/user/feeds/F0X/sync")));
    assert!(got.iter().any(|(_, u)| u.ends_with("/user/mail/folder/INBOX/unread")));
    assert!(got.iter().any(|(_, u)| u.ends_with("/tenant/keys/expired")));
    assert!(got.iter().any(|(m, u)| m == "POST" && u.ends_with("/tenant/keys/lookup/prefix")));
}

