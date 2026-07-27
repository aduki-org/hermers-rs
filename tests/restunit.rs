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
