//! HTTP transport and whoami cache.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::rest::config::{Config, BASE_URL};
use crate::rest::error::HermesError;
use crate::rest::types::Query;

/// Per-request options.
#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    /// Extra headers.
    pub headers: HashMap<String, String>,
    /// Query string parameters.
    pub query: HashMap<String, String>,
    /// `Idempotency-Key` header.
    pub idempotency: Option<String>,
    /// `If-Match` header.
    pub match_etag: Option<String>,
}

/// Cached identity from REST `GET /auth/whoami`.
#[derive(Debug, Clone)]
pub struct Identity {
    /// Session / JTI hex.
    pub hex: Option<String>,
    /// Authenticated user hex.
    pub user: String,
    /// Authenticated tenant hex.
    pub tenant: String,
    /// Whether the key belongs to a tenant owner.
    pub owner: Option<bool>,
    /// Granted scopes.
    pub scopes: Option<Vec<String>>,
    /// Denied scopes.
    pub deny: Option<Vec<String>>,
    /// Plan / tier label.
    pub tier: Option<String>,
    /// Present on wire; currently often `""`.
    pub ip: Option<String>,
    /// Present on wire; currently often `""`.
    pub agent: Option<String>,
    /// Raw whoami JSON.
    pub raw: Option<Value>,
}

/// Outbound request for a custom [`Backend`].
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method.
    pub method: String,
    /// Full URL.
    pub url: String,
    /// Headers.
    pub headers: HashMap<String, String>,
    /// Optional JSON body bytes.
    pub body: Option<Vec<u8>>,
}

/// Inbound response for a custom [`Backend`].
#[derive(Debug, Clone)]
pub struct Response {
    /// Status code.
    pub status: u16,
    /// Status text.
    pub status_text: String,
    /// Body bytes.
    pub body: Vec<u8>,
}

type BoxFut<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Pluggable HTTP executor (production = reqwest; tests = mock).
pub trait Backend: Send + Sync {
    /// Execute one HTTP request.
    fn execute<'a>(&'a self, req: Request) -> BoxFut<'a, Result<Response, HermesError>>;
}

/// Default backend using `reqwest` + rustls.
pub struct Reqwest {
    client: reqwest::Client,
}

impl Reqwest {
    /// Build a default client.
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl Default for Reqwest {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for Reqwest {
    fn execute<'a>(&'a self, req: Request) -> BoxFut<'a, Result<Response, HermesError>> {
        Box::pin(async move {
            let method = req
                .method
                .parse()
                .map_err(|_| HermesError::new("invalid HTTP method", 0, "network_error"))?;
            let mut builder = self.client.request(method, &req.url);
            for (k, v) in &req.headers {
                builder = builder.header(k, v);
            }
            if let Some(body) = req.body {
                builder = builder.body(body);
            }
            let res = builder
                .send()
                .await
                .map_err(|e| HermesError::new(e.to_string(), 0, "network_error"))?;
            let status = res.status().as_u16();
            let status_text = res.status().canonical_reason().unwrap_or("").to_string();
            let body = res
                .bytes()
                .await
                .map_err(|e| HermesError::new(e.to_string(), 0, "network_error"))?;
            Ok(Response {
                status,
                status_text,
                body: body.to_vec(),
            })
        })
    }
}

#[derive(Clone, Deserialize, Serialize)]
struct WhoamiResponse {
    hex: Option<String>,
    user: Option<String>,
    tenant: Option<String>,
    owner: Option<bool>,
    scopes: Option<Vec<String>>,
    deny: Option<Vec<String>>,
    tier: Option<String>,
    ip: Option<String>,
    agent: Option<String>,
}

/// Low-level HTTP client. Auth is API key only: `Authorization: Key <key>`.
///
/// When constructed inside a Tokio runtime, starts `GET /auth/whoami` eagerly
/// (same UX as the TypeScript SDK). Otherwise whoami runs on first `ready()` /
/// resource call.
#[derive(Clone)]
pub struct Client {
    /// API base URL (no trailing slash).
    pub api_base: String,
    /// Raw API key.
    pub api_key: String,
    backend: Arc<dyn Backend>,
    identity_cache: Arc<Mutex<Option<Identity>>>,
    whoami_lock: Arc<tokio::sync::Mutex<()>>,
}

impl Client {
    /// Construct with the default reqwest backend.
    pub fn new(api_key: impl Into<String>, options: Config) -> Result<Self, HermesError> {
        Self::with_backend(api_key, options, Arc::new(Reqwest::new()))
    }

    /// Construct with a custom HTTP backend (unit tests).
    pub fn with_backend(
        api_key: impl Into<String>,
        options: Config,
        backend: Arc<dyn Backend>,
    ) -> Result<Self, HermesError> {
        let api_key = api_key.into();
        if api_key.is_empty() {
            return Err(HermesError::new(
                "API key is required (e.g. hm_live_...)",
                0,
                "invalid_api_key",
            ));
        }
        let api_base = options
            .api_base
            .unwrap_or_else(|| BASE_URL.to_string())
            .trim_end_matches('/')
            .to_string();

        let client = Self {
            api_base,
            api_key,
            backend,
            identity_cache: Arc::new(Mutex::new(None)),
            whoami_lock: Arc::new(tokio::sync::Mutex::new(())),
        };
        client.maybe_spawn_eager_whoami();
        Ok(client)
    }

    fn maybe_spawn_eager_whoami(&self) {
        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            return;
        };
        let this = Self {
            api_base: self.api_base.clone(),
            api_key: self.api_key.clone(),
            backend: Arc::clone(&self.backend),
            identity_cache: Arc::clone(&self.identity_cache),
            whoami_lock: Arc::clone(&self.whoami_lock),
        };
        handle.spawn(async move {
            let _ = this.whoami().await;
        });
    }

    /// Synced snapshot after `ready()` / `whoami()`; otherwise `None`.
    pub fn me(&self) -> Option<Identity> {
        self.identity_cache.lock().unwrap().clone()
    }

    /// Await until identity is cached.
    pub async fn ready(&self) -> Result<Identity, HermesError> {
        self.whoami().await
    }

    /// Resolve (and cache) the authenticated identity.
    pub async fn whoami(&self) -> Result<Identity, HermesError> {
        if let Some(id) = self.me() {
            return Ok(id);
        }
        let _guard = self.whoami_lock.lock().await;
        if let Some(id) = self.me() {
            return Ok(id);
        }
        let id = self.fetch_whoami().await?;
        *self.identity_cache.lock().unwrap() = Some(id.clone());
        Ok(id)
    }

    async fn fetch_whoami(&self) -> Result<Identity, HermesError> {
        let profile: WhoamiResponse = self.get_raw("/auth/whoami").await?;
        let user = profile.user.filter(|s| !s.is_empty()).ok_or_else(|| {
            HermesError::new(
                "whoami response missing user or tenant",
                0,
                "invalid_identity",
            )
        })?;
        let tenant = profile.tenant.filter(|s| !s.is_empty()).ok_or_else(|| {
            HermesError::new(
                "whoami response missing user or tenant",
                0,
                "invalid_identity",
            )
        })?;
        let raw = serde_json::to_value(&WhoamiResponse {
            hex: profile.hex.clone(),
            user: Some(user.clone()),
            tenant: Some(tenant.clone()),
            owner: profile.owner,
            scopes: profile.scopes.clone(),
            deny: profile.deny.clone(),
            tier: profile.tier.clone(),
            ip: profile.ip.clone(),
            agent: profile.agent.clone(),
        })
        .ok();
        Ok(Identity {
            hex: profile.hex,
            user,
            tenant,
            owner: profile.owner,
            scopes: profile.scopes,
            deny: profile.deny,
            tier: profile.tier,
            ip: profile.ip,
            agent: profile.agent,
            raw,
        })
    }

    async fn get_raw<T: DeserializeOwned>(&self, path: &str) -> Result<T, HermesError> {
        self.request_internal("GET", path, Option::<&()>::None, &RequestOptions::default())
            .await
    }

    /// Authenticated request (awaits whoami first).
    pub async fn request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: &str,
        path: &str,
        body: Option<&B>,
        options: &RequestOptions,
    ) -> Result<T, HermesError> {
        let _ = self.whoami().await?;
        self.request_internal(method, path, body, options).await
    }

    async fn request_internal<T: DeserializeOwned, B: Serialize>(
        &self,
        method: &str,
        path: &str,
        body: Option<&B>,
        options: &RequestOptions,
    ) -> Result<T, HermesError> {
        let mut url = if path.starts_with("http") {
            path.to_string()
        } else if path.starts_with('/') {
            format!("{}{}", self.api_base, path)
        } else {
            format!("{}/{}", self.api_base, path)
        };

        if !options.query.is_empty() {
            let mut first = !url.contains('?');
            for (k, v) in &options.query {
                url.push(if first { '?' } else { '&' });
                first = false;
                url.push_str(&encode(k));
                url.push('=');
                url.push_str(&encode(v));
            }
        }

        let mut headers = HashMap::new();
        headers.insert("Accept".into(), "application/json".into());
        headers.insert("Authorization".into(), format!("Key {}", self.api_key));
        for (k, v) in &options.headers {
            headers.insert(k.clone(), v.clone());
        }
        let bodybytes = if let Some(b) = body {
            headers.insert("Content-Type".into(), "application/json".into());
            Some(
                serde_json::to_vec(b)
                    .map_err(|e| HermesError::new(e.to_string(), 0, "serialize_error"))?,
            )
        } else {
            None
        };
        if let Some(ref key) = options.idempotency {
            headers.insert("Idempotency-Key".into(), key.clone());
        }
        if let Some(ref etag) = options.match_etag {
            headers.insert("If-Match".into(), etag.clone());
        }

        let res = self
            .backend
            .execute(Request {
                method: method.to_string(),
                url,
                headers,
                body: bodybytes,
            })
            .await?;

        if res.status == 204 {
            return serde_json::from_value(Value::Null)
                .or_else(|_| serde_json::from_value(Value::Object(Default::default())))
                .map_err(|e| HermesError::new(e.to_string(), 204, "decode_error"));
        }

        let parsed: Option<Value> = if res.body.is_empty() {
            None
        } else {
            match serde_json::from_slice(&res.body) {
                Ok(v) => Some(v),
                Err(_) => Some(Value::String(
                    String::from_utf8_lossy(&res.body).into_owned(),
                )),
            }
        };

        if !(200..300).contains(&res.status) {
            return Err(HermesError::from_response(
                res.status,
                &res.status_text,
                parsed,
            ));
        }

        let value = parsed.unwrap_or(Value::Object(Default::default()));
        if value.is_null() {
            return serde_json::from_value(Value::Null).or_else(|_| {
                serde_json::from_value(Value::Object(Default::default()))
                    .map_err(|e| HermesError::new(e.to_string(), res.status, "decode_error"))
            });
        }
        serde_json::from_value(value)
            .map_err(|e| HermesError::new(e.to_string(), res.status, "decode_error"))
    }

    /// GET helper.
    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        options: Option<&RequestOptions>,
    ) -> Result<T, HermesError> {
        let default = RequestOptions::default();
        self.request::<T, ()>("GET", path, None, options.unwrap_or(&default))
            .await
    }

    /// POST helper.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
        options: Option<&RequestOptions>,
    ) -> Result<T, HermesError> {
        let default = RequestOptions::default();
        self.request("POST", path, Some(body), options.unwrap_or(&default))
            .await
    }

    /// PATCH helper.
    pub async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
        options: Option<&RequestOptions>,
    ) -> Result<T, HermesError> {
        let default = RequestOptions::default();
        self.request("PATCH", path, Some(body), options.unwrap_or(&default))
            .await
    }

    /// DELETE helper.
    pub async fn delete<T: DeserializeOwned>(
        &self,
        path: &str,
        options: Option<&RequestOptions>,
    ) -> Result<T, HermesError> {
        let default = RequestOptions::default();
        self.request::<T, ()>("DELETE", path, None, options.unwrap_or(&default))
            .await
    }
}

fn encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Build query options from optional list params.
pub fn list_query(params: &Query) -> RequestOptions {
    let mut q = HashMap::new();
    if let Some(ref v) = params.after {
        q.insert("after".into(), v.clone());
    }
    if let Some(v) = params.limit {
        q.insert("limit".into(), v.to_string());
    }
    if let Some(v) = params.page {
        q.insert("page".into(), v.to_string());
    }
    if let Some(ref v) = params.group {
        q.insert("group".into(), v.clone());
    }
    if let Some(ref v) = params.search {
        q.insert("search".into(), v.clone());
    }
    RequestOptions {
        query: q,
        ..Default::default()
    }
}
