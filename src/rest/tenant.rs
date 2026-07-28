//! Tenant resource.

use serde_json::{json, Value};

use crate::rest::error::HermesError;
use crate::rest::http::{list_query, Client};
use crate::rest::types::{
    Ack, Audit, AuditDetail, Domain, Hex, Invitation, Member, Page, Query, Quota, Rule,
    RuleDetail, Security, Tenant as TenantModel, TenantProfile, TenantSummary, Usage, Webhook,
    WebhookDetail, WebhookModel,
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

    /// List webhooks.
    pub async fn webhooks(&self, query: Option<Query>) -> Result<Page<Webhook>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/webhooks", opts.as_ref()).await
    }

    /// List active webhooks.
    pub async fn active_webhooks(
        &self,
        query: Option<Query>,
    ) -> Result<Page<Webhook>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get("/tenant/webhooks/active", opts.as_ref())
            .await
    }

    /// List webhooks subscribed to an event.
    pub async fn webhook_subscribers(
        &self,
        event: &str,
        query: Option<Query>,
    ) -> Result<Page<Webhook>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/tenant/webhooks/subscribers/{event}"),
                opts.as_ref(),
            )
            .await
    }

    /// Create webhook. Body: `{ url, secret, events?, domains?, active?, meta? }`.
    pub async fn create_webhook(&self, body: &Value) -> Result<Hex, HermesError> {
        self.http.post("/tenant/webhooks", body, None).await
    }

    /// Full webhook model (includes `secret`).
    pub async fn retrieve_webhook(&self, hex: &str) -> Result<WebhookModel, HermesError> {
        self.http
            .get(&format!("/tenant/webhooks/{hex}"), None)
            .await
    }

    /// Detail view (events + nested tenant; no secret).
    pub async fn detail_webhook(&self, hex: &str) -> Result<WebhookDetail, HermesError> {
        self.http
            .get(&format!("/tenant/webhooks/{hex}/detail"), None)
            .await
    }

    /// Toggle active.
    pub async fn update_webhook_active(
        &self,
        hex: &str,
        active: bool,
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/webhooks/{hex}/active"),
                &json!({ "active": active }),
                None,
            )
            .await
    }

    /// Replace domain scope.
    pub async fn update_webhook_domains(
        &self,
        hex: &str,
        domains: &[String],
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/webhooks/{hex}/domains"),
                &json!({ "domains": domains }),
                None,
            )
            .await
    }

    /// Replace event subscriptions.
    pub async fn update_webhook_events(
        &self,
        hex: &str,
        events: &[String],
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/webhooks/{hex}/events"),
                &json!({ "events": events }),
                None,
            )
            .await
    }

    /// Rotate signing secret.
    pub async fn update_webhook_secret(
        &self,
        hex: &str,
        secret: &str,
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/webhooks/{hex}/secret"),
                &json!({ "secret": secret }),
                None,
            )
            .await
    }

    /// Change callback URL.
    pub async fn update_webhook_url(&self, hex: &str, url: &str) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/webhooks/{hex}/url"),
                &json!({ "url": url }),
                None,
            )
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
    /// Promote tenant plan (owner-only).
    pub async fn promote(&self) -> Result<Ack, HermesError> {
        self.http.post("/tenant/promote", &json!({}), None).await
    }

    /// Transfer ownership.
    pub async fn transfer(
        &self,
        tenant: &str,
        from: &str,
        to: &str,
    ) -> Result<Value, HermesError> {
        self.http
            .post(
                &format!("/tenant/transfer/{tenant}/{from}/{to}"),
                &json!({}),
                None,
            )
            .await
    }

    /// Audit detail by hex.
    pub async fn view_audit(&self, hex: &str) -> Result<AuditDetail, HermesError> {
        self.http
            .get(&format!("/tenant/view/audit/{hex}"), None)
            .await
    }

    /// Active domains.
    pub async fn active_domains(
        &self,
        query: Option<Query>,
    ) -> Result<Page<Domain>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/domains/active", opts.as_ref()).await
    }

    /// Pending domains.
    pub async fn pending_domains(
        &self,
        query: Option<Query>,
    ) -> Result<Page<Domain>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get("/tenant/domains/pending", opts.as_ref())
            .await
    }

    /// Domains by status (GET with JSON body).
    pub async fn domains_by_status(
        &self,
        status: &str,
        query: Option<Query>,
    ) -> Result<Page<Domain>, HermesError> {
        let default = crate::rest::http::RequestOptions::default();
        let opts = query.as_ref().map(list_query);
        self.http
            .request(
                "GET",
                "/tenant/domains/status",
                Some(&json!({ "status": status })),
                opts.as_ref().unwrap_or(&default),
            )
            .await
    }

    /// Domain by name path.
    pub async fn domain_by_name(&self, name: &str) -> Result<Domain, HermesError> {
        self.http
            .get(&format!("/tenant/domains/name/{name}"), None)
            .await
    }

    /// Lookup domain by name (GET with JSON body).
    pub async fn lookup_domain_by_name(&self, name: &str) -> Result<Domain, HermesError> {
        let opts = crate::rest::http::RequestOptions::default();
        self.http
            .request(
                "GET",
                "/tenant/domains/lookup/name",
                Some(&json!({ "name": name })),
                &opts,
            )
            .await
    }

    /// Patch domain kind.
    pub async fn update_domain_kind(&self, hex: &str, kind: &str) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/domains/{hex}/kind"),
                &json!({ "kind": kind }),
                None,
            )
            .await
    }

    /// Patch domain name.
    pub async fn update_domain_name(&self, hex: &str, name: &str) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/domains/{hex}/name"),
                &json!({ "name": name }),
                None,
            )
            .await
    }

    /// Patch domain status.
    pub async fn update_domain_status(
        &self,
        hex: &str,
        body: &Value,
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(&format!("/tenant/domains/{hex}/status"), body, None)
            .await
    }

    /// Patch DKIM.
    pub async fn update_domain_dkim(&self, hex: &str, dkim: &str) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/domains/{hex}/dkim"),
                &json!({ "dkim": dkim }),
                None,
            )
            .await
    }

    /// Patch selector.
    pub async fn update_domain_selector(
        &self,
        hex: &str,
        selector: &str,
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/domains/{hex}/selector"),
                &json!({ "selector": selector }),
                None,
            )
            .await
    }

    /// Patch auth records.
    pub async fn update_domain_auth(&self, hex: &str, body: &Value) -> Result<Ack, HermesError> {
        self.http
            .patch(&format!("/tenant/domains/{hex}/auth"), body, None)
            .await
    }

    /// Patch meta.
    pub async fn update_domain_meta(&self, hex: &str, meta: &Value) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/domains/{hex}/meta"),
                &json!({ "meta": meta }),
                None,
            )
            .await
    }

    /// Tenant audits.
    pub async fn audits(&self, query: Option<Query>) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/audits", opts.as_ref()).await
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
                &format!("/tenant/audits/action/{action}"),
                opts.as_ref(),
            )
            .await
    }

    /// Failed audits.
    pub async fn failed_audits(&self, query: Option<Query>) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/audits/failed", opts.as_ref()).await
    }

    /// Successful audits.
    pub async fn successful_audits(
        &self,
        query: Option<Query>,
    ) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get("/tenant/audits/successful", opts.as_ref())
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
            .get(&format!("/tenant/audits/ip/{ip}"), opts.as_ref())
            .await
    }

    /// Audits for a user.
    pub async fn audits_by_user(
        &self,
        user: &str,
        query: Option<Query>,
    ) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(&format!("/tenant/audits/user/{user}"), opts.as_ref())
            .await
    }

    /// Audits for user+action.
    pub async fn audits_by_user_action(
        &self,
        user: &str,
        action: &str,
        query: Option<Query>,
    ) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/tenant/audits/user/{user}/action/{action}"),
                opts.as_ref(),
            )
            .await
    }

    /// Failed audits for a user.
    pub async fn failed_audits_by_user(
        &self,
        user: &str,
        query: Option<Query>,
    ) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/tenant/audits/user/{user}/failed"),
                opts.as_ref(),
            )
            .await
    }

    /// Successful audits for a user.
    pub async fn successful_audits_by_user(
        &self,
        user: &str,
        query: Option<Query>,
    ) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/tenant/audits/user/{user}/successful"),
                opts.as_ref(),
            )
            .await
    }

    /// Audits for user+IP.
    pub async fn audits_by_user_ip(
        &self,
        user: &str,
        ip: &str,
        query: Option<Query>,
    ) -> Result<Page<Audit>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/tenant/audits/user/{user}/ip/{ip}"),
                opts.as_ref(),
            )
            .await
    }

    /// Pending invitations.
    pub async fn pending_invitations(
        &self,
        query: Option<Query>,
    ) -> Result<Page<Invitation>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get("/tenant/invitations/pending", opts.as_ref())
            .await
    }

    /// Expired invitations.
    pub async fn expired_invitations(
        &self,
        query: Option<Query>,
    ) -> Result<Page<Invitation>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get("/tenant/invitations/expired", opts.as_ref())
            .await
    }

    /// Invitations by status.
    pub async fn invitations_by_status(
        &self,
        status: &str,
        query: Option<Query>,
    ) -> Result<Page<Invitation>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/tenant/invitations/status/{status}"),
                opts.as_ref(),
            )
            .await
    }

    /// Retrieve quota by metric.
    pub async fn retrieve_quota(&self, metric: &str) -> Result<Quota, HermesError> {
        self.http
            .get(&format!("/tenant/quotas/{metric}"), None)
            .await
    }

    /// Patch quota ceiling.
    pub async fn update_quota_ceiling(
        &self,
        metric: &str,
        ceiling: i64,
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/quotas/{metric}/ceiling"),
                &json!({ "ceiling": ceiling }),
                None,
            )
            .await
    }

    /// Patch quota expires.
    pub async fn update_quota_expires(
        &self,
        metric: &str,
        expires: &str,
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/quotas/{metric}/expires"),
                &json!({ "expires": expires }),
                None,
            )
            .await
    }

    /// Patch quota reason.
    pub async fn update_quota_reason(
        &self,
        metric: &str,
        reason: &str,
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/quotas/{metric}/reason"),
                &json!({ "reason": reason }),
                None,
            )
            .await
    }

    /// Delete quota.
    pub async fn delete_quota(&self, metric: &str) -> Result<Ack, HermesError> {
        self.http
            .delete(&format!("/tenant/quotas/{metric}"), None)
            .await
    }

    /// Active rules.
    pub async fn active_rules(&self, query: Option<Query>) -> Result<Page<Rule>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http.get("/tenant/rules/active", opts.as_ref()).await
    }

    /// Rules by target.
    pub async fn rules_by_target(
        &self,
        target: &str,
        query: Option<Query>,
    ) -> Result<Page<Rule>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/tenant/rules/target/{target}"),
                opts.as_ref(),
            )
            .await
    }

    /// Get rule model.
    pub async fn get_rule(&self, hex: &str) -> Result<Rule, HermesError> {
        self.http.get(&format!("/tenant/rules/{hex}"), None).await
    }

    /// Rule by name.
    pub async fn rule_by_name(&self, name: &str) -> Result<Rule, HermesError> {
        self.http
            .get(&format!("/tenant/rules/name/{name}"), None)
            .await
    }

    /// Patch rule active.
    pub async fn update_rule_active(&self, hex: &str, active: bool) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/rules/{hex}/active"),
                &json!({ "active": active }),
                None,
            )
            .await
    }

    /// Patch rule name.
    pub async fn update_rule_name(&self, hex: &str, name: &str) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/rules/{hex}/name"),
                &json!({ "name": name }),
                None,
            )
            .await
    }

    /// Patch rule pattern.
    pub async fn update_rule_pattern(
        &self,
        hex: &str,
        pattern: &str,
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/rules/{hex}/pattern"),
                &json!({ "pattern": pattern }),
                None,
            )
            .await
    }

    /// Patch rule score.
    pub async fn update_rule_score(&self, hex: &str, score: f32) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/rules/{hex}/score"),
                &json!({ "score": score }),
                None,
            )
            .await
    }

    /// Patch rule target.
    pub async fn update_rule_target(
        &self,
        hex: &str,
        target: &str,
    ) -> Result<Ack, HermesError> {
        self.http
            .patch(
                &format!("/tenant/rules/{hex}/target"),
                &json!({ "target": target }),
                None,
            )
            .await
    }

    /// Delete rule.
    pub async fn delete_rule(&self, hex: &str) -> Result<Ack, HermesError> {
        self.http
            .delete(&format!("/tenant/rules/{hex}"), None)
            .await
    }

    /// Usage by metric.
    pub async fn usage_by_metric(
        &self,
        metric: &str,
        query: Option<Query>,
    ) -> Result<Page<Usage>, HermesError> {
        let opts = query.as_ref().map(list_query);
        self.http
            .get(
                &format!("/tenant/usage/metric/{metric}"),
                opts.as_ref(),
            )
            .await
    }

}
