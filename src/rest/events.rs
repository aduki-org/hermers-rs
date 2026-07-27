//! Events resource.

use serde_json::Value;

use crate::rest::error::HermesError;
use crate::rest::http::{list_query, Client};
use crate::rest::types::{Event, Page, Query};

/// Calendar events.
pub struct Events {
    http: Client,
}

impl Events {
    pub(crate) fn new(http: Client) -> Self {
        Self { http }
    }

    /// List events.
    pub async fn list(&self, query: Option<Query>) -> Result<Page<Event>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/events", opts.as_ref()).await
    }

    /// Events in range.
    pub async fn range(&self, start: &str, end: &str) -> Result<Page<Event>, HermesError> {
        self.http
            .get(&format!("/user/events/range/{start}/{end}"), None)
            .await
    }

    /// Recurring events.
    pub async fn recurring(&self) -> Result<Page<Event>, HermesError> {
        self.http.get("/user/events/recurring", None).await
    }

    /// Search events.
    pub async fn search(&self, q: &str) -> Result<Page<Event>, HermesError> {
        self.http
            .get(&format!("/user/events/search/{q}"), None)
            .await
    }

    /// Upcoming.
    pub async fn upcoming(&self) -> Result<Page<Event>, HermesError> {
        self.http.get("/user/events/upcoming", None).await
    }

    /// Past.
    pub async fn past(&self) -> Result<Page<Event>, HermesError> {
        self.http.get("/user/events/past", None).await
    }

    /// Create event.
    pub async fn create(&self, body: &Value) -> Result<Value, HermesError> {
        self.http.post("/user/events", body, None).await
    }

    /// Update event.
    pub async fn update(&self, hex: &str, body: &Value) -> Result<Value, HermesError> {
        self.http
            .patch(&format!("/user/events/{hex}"), body, None)
            .await
    }

    /// Delete event.
    pub async fn remove(&self, hex: &str) -> Result<Value, HermesError> {
        self.http.delete(&format!("/user/events/{hex}"), None).await
    }
}
