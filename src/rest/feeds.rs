//! Feeds resource.

use serde_json::{json, Value};

use crate::rest::error::HermesError;
use crate::rest::http::Client;
use crate::rest::types::{Feed, FeedSync};

/// External calendar feeds.
pub struct Feeds {
    http: Client,
}

impl Feeds {
    pub(crate) fn new(http: Client) -> Self {
        Self { http }
    }

    /// Create a feed.
    pub async fn create(
        &self,
        connection: &str,
        remote: &str,
        name: &str,
        color: Option<&str>,
        block: Option<bool>,
    ) -> Result<Feed, HermesError> {
        let mut body = json!({
            "connection": connection,
            "remote": remote,
            "name": name,
        });
        if let Some(c) = color {
            body["color"] = json!(c);
        }
        if let Some(b) = block {
            body["block"] = json!(b);
        }
        self.http.post("/user/feeds", &body, None).await
    }

    /// List feeds.
    pub async fn list(&self) -> Result<Vec<Feed>, HermesError> {
        self.http.get("/user/feeds", None).await
    }

    /// Retrieve a feed.
    pub async fn retrieve(&self, hex: &str) -> Result<Feed, HermesError> {
        self.http.get(&format!("/user/feeds/{hex}"), None).await
    }

    /// Update a feed.
    pub async fn update(&self, hex: &str, body: &Value) -> Result<Feed, HermesError> {
        self.http
            .patch(&format!("/user/feeds/{hex}"), body, None)
            .await
    }

    /// Trigger an immediate sync.
    pub async fn sync(&self, hex: &str) -> Result<FeedSync, HermesError> {
        self.http
            .post(&format!("/user/feeds/{hex}/sync"), &json!({}), None)
            .await
    }

    /// Delete a feed.
    pub async fn remove(&self, hex: &str) -> Result<Value, HermesError> {
        self.http.delete(&format!("/user/feeds/{hex}"), None).await
    }
}
