//! Calendar resource.

use serde_json::json;

use crate::rest::error::HermesError;
use crate::rest::http::{list_query, Client};
use crate::rest::types::{
    Calendar as CalendarRow, CalendarCreated, Event, Page, Query,
};

/// Calendars.
pub struct Calendar {
    http: Client,
}

impl Calendar {
    pub(crate) fn new(http: Client) -> Self {
        Self { http }
    }

    /// List calendars.
    pub async fn list(&self, query: Option<Query>) -> Result<Page<CalendarRow>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/calendars", opts.as_ref()).await
    }

    /// Search calendars.
    pub async fn search(&self, q: &str) -> Result<Page<CalendarRow>, HermesError> {
        self.http
            .get(&format!("/user/calendars/search/{q}"), None)
            .await
    }

    /// Create a calendar.
    pub async fn create(
        &self,
        name: &str,
        description: Option<&str>,
        color: Option<&str>,
        timezone: Option<&str>,
    ) -> Result<CalendarCreated, HermesError> {
        let mut body = json!({ "name": name });
        if let Some(d) = description {
            body["description"] = json!(d);
        }
        if let Some(c) = color {
            body["color"] = json!(c);
        }
        if let Some(t) = timezone {
            body["timezone"] = json!(t);
        }
        self.http.post("/user/calendars", &body, None).await
    }

    /// Events across calendars.
    pub async fn events(&self, query: Option<Query>) -> Result<Page<Event>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/calendars/events", opts.as_ref()).await
    }
}
