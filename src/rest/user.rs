//! User resource.

use serde_json::{json, Value};

use crate::rest::error::HermesError;
use crate::rest::http::{list_query, Client};
use crate::rest::types::{
    Audit, Page, Preference, PreferenceInfoBody, Query, Session, User as UserModel, UserProfile,
};

/// Profile, sessions, audits, preferences.
pub struct User {
    http: Client,
}

impl User {
    pub(crate) fn new(http: Client) -> Self {
        Self { http }
    }

    /// Current user.
    pub async fn retrieve(&self) -> Result<UserModel, HermesError> {
        self.http.get("/user", None).await
    }

    /// Lookup by email.
    pub async fn lookup_by_email(&self, email: &str) -> Result<UserModel, HermesError> {
        self.http
            .post("/user/lookup/email", &json!({ "email": email }), None)
            .await
    }

    /// Lookup profile by hex.
    pub async fn lookup_profile(&self, hex: &str) -> Result<UserProfile, HermesError> {
        self.http
            .post("/user/lookup/profile", &json!({ "hex": hex }), None)
            .await
    }

    /// Update name/bio.
    pub async fn update_info(&self, name: &str, bio: &str) -> Result<UserModel, HermesError> {
        self.http
            .patch("/user/info", &json!({ "name": name, "bio": bio }), None)
            .await
    }

    /// Update email (raw JSON string body).
    pub async fn update_email(&self, email: &str) -> Result<UserModel, HermesError> {
        self.http.patch("/user/email", &json!(email), None).await
    }

    /// Update phone (raw JSON string body).
    pub async fn update_phone(&self, phone: &str) -> Result<UserModel, HermesError> {
        self.http.patch("/user/phone", &json!(phone), None).await
    }

    /// Update meta (unwrapped object body).
    pub async fn update_meta(&self, meta: &Value) -> Result<UserModel, HermesError> {
        self.http.patch("/user/meta", meta, None).await
    }

    /// Update avatar (raw JSON string body).
    pub async fn update_avatar(&self, avatar: &str) -> Result<UserModel, HermesError> {
        self.http.patch("/user/avatar", &json!(avatar), None).await
    }

    /// Active sessions.
    pub async fn active_sessions(&self, query: Option<Query>) -> Result<Page<Session>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/sessions", opts.as_ref()).await
    }

    /// Sessions filtered by auth method.
    pub async fn sessions_by_method(
        &self,
        method: &str,
        query: Option<Query>,
    ) -> Result<Page<Session>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/user/sessions/method/{method}"),
                opts.as_ref(),
            )
            .await
    }

    /// Audits.
    pub async fn audits(&self, query: Option<Query>) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/audits", opts.as_ref()).await
    }

    /// Update preferences section.
    pub async fn update_preferences_info(
        &self,
        body: &PreferenceInfoBody,
    ) -> Result<Preference, HermesError> {
        self.http
            .patch("/user/preferences/info", body, None)
            .await
    }

    /// Update a freeform preference section.
    pub async fn update_preferences(
        &self,
        section: &str,
        body: &Value,
    ) -> Result<Preference, HermesError> {
        self.http
            .patch(&format!("/user/preferences/{section}"), body, None)
            .await
    }
    /// Audits by action.
    pub async fn audits_by_action(
        &self,
        action: &str,
        query: Option<Query>,
    ) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/user/audits/action/{action}"),
                opts.as_ref(),
            )
            .await
    }

    /// Failed audits.
    pub async fn failed_audits(&self, query: Option<Query>) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/user/audits/failed", opts.as_ref()).await
    }

    /// Successful audits.
    pub async fn successful_audits(
        &self,
        query: Option<Query>,
    ) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get("/user/audits/successful", opts.as_ref())
            .await
    }

    /// Audits by IP.
    pub async fn audits_by_ip(
        &self,
        ip: &str,
        query: Option<Query>,
    ) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(&format!("/user/audits/ip/{ip}"), opts.as_ref())
            .await
    }

}
