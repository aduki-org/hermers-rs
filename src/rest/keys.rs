//! API keys resource.

use serde_json::{json, Value};

use crate::rest::crypto::{generate_key, hash_key, prefix_key};
use crate::rest::error::HermesError;
use crate::rest::http::{list_query, Client};
use crate::rest::types::{ApiKey, Keypair, Page, Query};

/// API keys — list and create (no login flows).
pub struct Keys {
    http: Client,
}

impl Keys {
    pub(crate) fn new(http: Client) -> Self {
        Self { http }
    }

    /// List keys for the authenticated user.
    pub async fn list(&self, query: Option<Query>) -> Result<Page<ApiKey>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/keys", opts.as_ref()).await
    }

    /// List tenant keys.
    pub async fn list_tenant(&self, query: Option<Query>) -> Result<Page<ApiKey>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/keys", opts.as_ref()).await
    }

    /// List active tenant keys.
    pub async fn list_active(&self) -> Result<Page<ApiKey>, HermesError> {
        self.http.get("/tenant/keys/active", None).await
    }

    /// Retrieve a key.
    pub async fn retrieve(&self, hex: &str) -> Result<ApiKey, HermesError> {
        self.http.get(&format!("/tenant/keys/{hex}"), None).await
    }

    /// Create a key. Generates `hm_live_…` client-side; sends hash+prefix only.
    pub async fn create(
        &self,
        name: &str,
        scopes: &[String],
        key: Option<&str>,
        meta: Option<Value>,
        expires: Option<&str>,
    ) -> Result<Keypair, HermesError> {
        let raw = key.map(str::to_string).unwrap_or_else(generate_key);
        let mut body = json!({
            "name": name,
            "hash": hash_key(&raw),
            "prefix": prefix_key(&raw),
            "scopes": scopes,
        });
        if let Some(m) = meta {
            body["meta"] = m;
        }
        if let Some(e) = expires {
            body["expires"] = json!(e);
        }
        let res: serde_json::Value = self.http.post("/tenant/keys", &body, None).await?;
        let hex = res
            .get("hex")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        Ok(Keypair { hex, key: raw })
    }

    /// Rename a key.
    pub async fn update_name(&self, hex: &str, name: &str) -> Result<Value, HermesError> {
        self.http
            .patch(
                &format!("/tenant/keys/{hex}/name"),
                &json!({ "name": name }),
                None,
            )
            .await
    }

    /// Update scopes.
    pub async fn update_scopes(&self, hex: &str, scopes: &[String]) -> Result<Value, HermesError> {
        self.http
            .patch(
                &format!("/tenant/keys/{hex}/scopes"),
                &json!({ "scopes": scopes }),
                None,
            )
            .await
    }

    /// Revoke/delete a key.
    pub async fn remove(&self, hex: &str) -> Result<Value, HermesError> {
        self.http.delete(&format!("/tenant/keys/{hex}"), None).await
    }
}
