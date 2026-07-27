//! Tenant resource.

use serde_json::{json, Value};

use crate::rest::error::HermesError;
use crate::rest::http::{list_query, Client};
use crate::rest::types::{
    Ack, Domain, Hex, Invitation, Member, Page, Query, Quota, Rule, RuleDetail, Security,
    Tenant as TenantModel, TenantProfile, TenantSummary, Usage, Webhook, WebhookDetail,
};

/// Tenant profile, members, domains, quotas, rules, webhooks, usage.
pub struct Tenant {
    http: Client,
}

impl Tenant {
    pub(crate) fn new(http: Client) -> Self {
        Self { http }
    }

    /// Current tenant profile.
    pub async fn retrieve(&self) -> Result<TenantProfile, HermesError> {
        self.http.get("/tenant", None).await
    }

    /// Rename tenant.
    pub async fn update(&self, name: &str) -> Result<TenantModel, HermesError> {
        self.http
            .patch("/tenant/edit", &json!({ "name": name }), None)
            .await
    }

    /// View tenant by hex.
    pub async fn view(&self, hex: &str) -> Result<TenantProfile, HermesError> {
        self.http.get(&format!("/tenant/view/{hex}"), None).await
    }

    /// View by slug.
    pub async fn by_slug(&self, slug: &str) -> Result<TenantSummary, HermesError> {
        self.http
            .get(&format!("/tenant/view/slug/{slug}"), None)
            .await
    }

    /// Members.
    pub async fn members(&self, query: Option<Query>) -> Result<Page<Member>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/members", opts.as_ref()).await
    }

    /// Active members.
    pub async fn active_members(&self, query: Option<Query>) -> Result<Page<Member>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/members/active", opts.as_ref()).await
    }

    /// Owners.
    pub async fn owners(&self, query: Option<Query>) -> Result<Page<Member>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/members/owners", opts.as_ref()).await
    }

    /// Search members.
    pub async fn search_members(&self, q: &str) -> Result<Page<Member>, HermesError> {
        self.http
            .get(&format!("/tenant/members/search/{q}"), None)
            .await
    }

    /// Invite a member.
    pub async fn invite(&self, email: &str, role: Option<&str>) -> Result<Value, HermesError> {
        let mut body = json!({ "email": email });
        if let Some(r) = role {
            body["role"] = json!(r);
        }
        self.http.post("/tenant/invite", &body, None).await
    }

    /// Remove a member.
    pub async fn remove_member(&self, user: &str) -> Result<Value, HermesError> {
        self.http
            .delete(&format!("/tenant/members/{user}"), None)
            .await
    }

    /// Create domain.
    pub async fn create_domain(
        &self,
        name: &str,
        kind: &str,
        selector: Option<&str>,
        meta: Option<Value>,
    ) -> Result<Hex, HermesError> {
        let mut body = json!({ "name": name, "kind": kind });
        if let Some(s) = selector {
            body["selector"] = json!(s);
        }
        if let Some(m) = meta {
            body["meta"] = m;
        }
        self.http.post("/tenant/domains", &body, None).await
    }

    /// List domains.
    pub async fn domains(&self, query: Option<Query>) -> Result<Page<Domain>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/domains", opts.as_ref()).await
    }

    /// Retrieve domain.
    pub async fn retrieve_domain(&self, hex: &str) -> Result<Domain, HermesError> {
        self.http.get(&format!("/tenant/domains/{hex}"), None).await
    }

    /// Delete domain.
    pub async fn delete_domain(&self, hex: &str) -> Result<Ack, HermesError> {
        self.http
            .delete(&format!("/tenant/domains/{hex}"), None)
            .await
    }

    /// Invitations.
    pub async fn invitations(&self, query: Option<Query>) -> Result<Page<Invitation>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/invitations", opts.as_ref()).await
    }

    /// Quotas.
    pub async fn quotas(&self, query: Option<Query>) -> Result<Page<Quota>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/quotas", opts.as_ref()).await
    }

    /// Create quota.
    pub async fn create_quota(&self, body: &Value) -> Result<Quota, HermesError> {
        self.http.post("/tenant/quotas", body, None).await
    }

    /// Rules.
    pub async fn rules(&self, query: Option<Query>) -> Result<Page<Rule>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/rules", opts.as_ref()).await
    }

    /// Create rule.
    pub async fn create_rule(&self, body: &Value) -> Result<Rule, HermesError> {
        self.http.post("/tenant/rules", body, None).await
    }

    /// Rule detail.
    pub async fn retrieve_rule(&self, hex: &str) -> Result<RuleDetail, HermesError> {
        self.http
            .get(&format!("/tenant/rules/{hex}/detail"), None)
            .await
    }

    /// Webhooks.
    pub async fn webhooks(&self, query: Option<Query>) -> Result<Page<Webhook>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/webhooks", opts.as_ref()).await
    }

    /// Create webhook.
    pub async fn create_webhook(&self, body: &Value) -> Result<Hex, HermesError> {
        self.http.post("/tenant/webhooks", body, None).await
    }

    /// Retrieve webhook.
    pub async fn retrieve_webhook(&self, hex: &str) -> Result<WebhookDetail, HermesError> {
        self.http
            .get(&format!("/tenant/webhooks/{hex}"), None)
            .await
    }

    /// Delete webhook.
    pub async fn delete_webhook(&self, hex: &str) -> Result<Ack, HermesError> {
        self.http
            .delete(&format!("/tenant/webhooks/{hex}"), None)
            .await
    }

    /// Security snapshot.
    pub async fn security(&self) -> Result<Security, HermesError> {
        self.http.get("/tenant/security", None).await
    }

    /// Usage page.
    pub async fn usage(&self, query: Option<Query>) -> Result<Page<Usage>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/usage", opts.as_ref()).await
    }

    /// Usage summary.
    pub async fn usage_summary(&self) -> Result<Vec<Usage>, HermesError> {
        self.http.get("/tenant/usage/summary", None).await
    }
}
