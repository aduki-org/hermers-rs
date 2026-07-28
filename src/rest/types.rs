//! REST DTO shapes matching Hermers HTTP JSON.
//!
//! Timestamps are naive datetime strings (`"2026-07-28T12:00:00"`).
//! Freeform jsonb stays [`Json`].

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Freeform JSON.
pub type Json = Value;

/// Paginated list envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    /// Page items.
    pub items: Vec<T>,
    /// Total matching rows.
    pub total: i64,
    /// Cursor for the next page.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// 1-based page index when used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Total pages when used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<u32>,
}

/// Common list query parameters.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Query {
    /// Cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Group filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Search string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

/// Nested hex + name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexName {
    /// Resource hex.
    pub hex: String,
    /// Display name.
    pub name: String,
}

/// Nested hex + name + email.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexNameEmail {
    /// Resource hex.
    pub hex: String,
    /// Display name.
    pub name: String,
    /// Email.
    pub email: String,
}

/// Nested hex + name + slug.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexNameSlug {
    /// Resource hex.
    pub hex: String,
    /// Display name.
    pub name: String,
    /// Slug.
    pub slug: String,
}

/// Full user model (`password` omitted).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Numeric id.
    pub id: i64,
    /// User hex.
    pub hex: String,
    /// Tenant hex.
    pub tenant: String,
    /// Email.
    pub email: String,
    /// Phone.
    #[serde(default)]
    pub phone: Option<String>,
    /// Display name.
    pub name: String,
    /// Bio.
    #[serde(default)]
    pub bio: Option<String>,
    /// Avatar URL.
    #[serde(default)]
    pub avatar: Option<String>,
    /// Owner flag.
    pub owner: bool,
    /// Account state.
    pub state: String,
    /// TOTP secret (rarely present).
    #[serde(default)]
    pub totp: Option<String>,
    /// Timezone.
    pub timezone: String,
    /// Locale.
    pub locale: String,
    /// Contacts blob.
    #[serde(default)]
    pub contacts: Option<Json>,
    /// Meta jsonb.
    pub meta: Json,
    /// Last activity.
    #[serde(default)]
    pub last: Option<String>,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Profile lookup view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User hex.
    pub hex: String,
    /// Email.
    pub email: String,
    /// Phone.
    #[serde(default)]
    pub phone: Option<String>,
    /// Name.
    pub name: String,
    /// Bio.
    #[serde(default)]
    pub bio: Option<String>,
    /// Avatar.
    #[serde(default)]
    pub avatar: Option<String>,
    /// Owner.
    pub owner: bool,
    /// State.
    pub state: String,
    /// Timezone.
    pub timezone: String,
    /// Locale.
    pub locale: String,
    /// Last seen.
    #[serde(default)]
    pub last: Option<String>,
    /// Created.
    pub created: String,
    /// Nested tenant summary.
    pub tenant: TenantBrief,
    /// Role object (may be empty `{}`).
    pub role: Json,
}

/// Brief tenant on profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantBrief {
    /// Hex.
    pub hex: String,
    /// Name.
    pub name: String,
    /// Slug.
    pub slug: String,
    /// Plan.
    pub plan: String,
    /// Kind.
    pub kind: String,
}

/// Preferences row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preference {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// User hex.
    pub user: String,
    /// Language.
    pub language: String,
    /// Timezone.
    pub timezone: String,
    /// Currency.
    pub currency: String,
    /// Theme.
    pub theme: String,
    /// Notifications jsonb.
    pub notifications: Json,
    /// Communication jsonb.
    pub communication: Json,
    /// Privacy jsonb.
    pub privacy: Json,
    /// Display jsonb.
    pub display: Json,
    /// Regional jsonb.
    pub regional: Json,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Preference `info` section body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenceInfoBody {
    /// Language.
    pub language: String,
    /// Timezone.
    pub timezone: String,
    /// Currency.
    pub currency: String,
    /// Theme: `light` | `dark` | `auto`.
    pub theme: String,
}

/// Tenant profile view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantProfile {
    /// Hex.
    pub hex: String,
    /// Kind.
    pub kind: String,
    /// Name.
    pub name: String,
    /// Slug.
    pub slug: String,
    /// Plan.
    pub plan: String,
    /// State.
    pub state: String,
    /// Primary domain.
    #[serde(default)]
    pub domain: Option<String>,
    /// Billing customer id.
    #[serde(default)]
    pub customer: Option<String>,
    /// Subscription id.
    #[serde(default)]
    pub subscription: Option<String>,
    /// Trial end.
    #[serde(default)]
    pub trial: Option<String>,
    /// Meta.
    pub meta: Json,
    /// Created.
    pub created: String,
    /// User count.
    pub users: i64,
    /// Domain count.
    pub domains: i64,
    /// Storage usage.
    pub storage: i64,
}

/// Full tenant model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Kind.
    pub kind: String,
    /// Name.
    pub name: String,
    /// Slug.
    pub slug: String,
    /// Plan.
    pub plan: String,
    /// State.
    pub state: String,
    /// Domain.
    #[serde(default)]
    pub domain: Option<String>,
    /// Customer.
    #[serde(default)]
    pub customer: Option<String>,
    /// Subscription.
    #[serde(default)]
    pub subscription: Option<String>,
    /// Billed at.
    #[serde(default)]
    pub billed: Option<String>,
    /// Renews at.
    #[serde(default)]
    pub renews: Option<String>,
    /// Trial.
    #[serde(default)]
    pub trial: Option<String>,
    /// Meta.
    pub meta: Json,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Tenant summary by slug.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantSummary {
    /// Hex.
    pub hex: String,
    /// Name.
    pub name: String,
    /// Slug.
    pub slug: String,
    /// Kind.
    pub kind: String,
    /// State.
    pub state: String,
    /// Created.
    pub created: String,
}

/// Member list row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    /// User hex.
    pub hex: String,
    /// Email.
    pub email: String,
    /// Name.
    pub name: String,
    /// Avatar.
    #[serde(default)]
    pub avatar: Option<String>,
    /// Owner.
    pub owner: bool,
    /// State.
    pub state: String,
    /// Last seen.
    #[serde(default)]
    pub last: Option<String>,
    /// Created.
    pub created: String,
    /// Tenant.
    pub tenant: HexNameSlug,
    /// Role summary.
    pub role: Json,
    /// Total (list).
    #[serde(default)]
    pub total: Option<i64>,
}

/// Domain list/detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Domain {
    /// Hex.
    pub hex: String,
    /// Domain name.
    pub name: String,
    /// Kind.
    pub kind: String,
    /// Status.
    pub status: String,
    /// DKIM selector.
    #[serde(default)]
    pub selector: Option<String>,
    /// DKIM.
    #[serde(default)]
    pub dkim: Option<String>,
    /// SPF.
    #[serde(default)]
    pub spf: Option<Json>,
    /// DMARC.
    #[serde(default)]
    pub dmarc: Option<Json>,
    /// Verified at.
    #[serde(default)]
    pub verified: Option<String>,
    /// Created.
    pub created: String,
    /// Tenant.
    pub tenant: HexNameSlug,
    /// Mailbox count.
    #[serde(default)]
    pub mailboxes: Option<i64>,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Invitation list row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invitation {
    /// Hex.
    pub hex: String,
    /// Email.
    pub email: String,
    /// Label.
    pub label: String,
    /// Status.
    pub status: String,
    /// Expires.
    pub expires: String,
    /// Created.
    pub created: String,
    /// Inviter.
    pub inviter: HexNameEmail,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Invitation detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitationDetail {
    /// Hex.
    pub hex: String,
    /// Email.
    pub email: String,
    /// Label.
    pub label: String,
    /// Status.
    pub status: String,
    /// Expires.
    pub expires: String,
    /// Created.
    pub created: String,
    /// Inviter.
    pub inviter: HexNameEmail,
    /// Privileges.
    pub privileges: Json,
    /// Message.
    #[serde(default)]
    pub message: Option<String>,
    /// Tenant.
    pub tenant: HexNameSlug,
}

/// Quota row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota {
    /// Tenant hex.
    pub tenant: String,
    /// Metric.
    pub metric: String,
    /// Ceiling.
    pub ceiling: i64,
    /// Expires.
    #[serde(default)]
    pub expires: Option<String>,
    /// Created.
    pub created: String,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
    /// Reason.
    #[serde(default)]
    pub reason: Option<String>,
    /// Granted by.
    #[serde(default)]
    pub granted: Option<String>,
    /// Updated.
    #[serde(default)]
    pub updated: Option<String>,
}

/// Rule list row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// Hex.
    pub hex: String,
    /// Target.
    pub target: String,
    /// Pattern.
    pub pattern: String,
    /// Score.
    pub score: f64,
    /// Active.
    pub active: bool,
    /// Name.
    pub name: String,
    /// Created.
    pub created: String,
    /// Tenant.
    #[serde(default)]
    pub tenant: Option<HexName>,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Rule detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleDetail {
    /// Hex.
    pub hex: String,
    /// Target.
    pub target: String,
    /// Pattern.
    pub pattern: String,
    /// Score.
    pub score: f64,
    /// Active.
    pub active: bool,
    /// Name.
    pub name: String,
    /// Created.
    pub created: String,
    /// Tenant.
    #[serde(default)]
    pub tenant: Option<HexName>,
    /// Meta.
    pub meta: Json,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// API key list/detail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Hex.
    pub hex: String,
    /// Name.
    pub name: String,
    /// Prefix.
    pub prefix: String,
    /// Active.
    pub active: bool,
    /// Expires.
    #[serde(default)]
    pub expires: Option<String>,
    /// Last used.
    #[serde(default)]
    pub last: Option<String>,
    /// Created.
    pub created: String,
    /// Tenant.
    pub tenant: HexName,
    /// User.
    #[serde(default)]
    pub user: Option<HexNameEmail>,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
    /// Scopes.
    #[serde(default)]
    pub scopes: Option<Json>,
}

/// Webhook list row (`GET /tenant/webhooks`, `/active`, `/subscribers/{event}`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    /// Hex.
    pub hex: String,
    /// URL.
    pub url: String,
    /// Active.
    pub active: bool,
    /// Created.
    pub created: String,
    /// Tenant.
    pub tenant: HexName,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Full webhook model (`GET /tenant/webhooks/{hex}`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookModel {
    /// Numeric id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Tenant hex string (not nested).
    pub tenant: String,
    /// Callback URL.
    pub url: String,
    /// Signing secret.
    pub secret: String,
    /// Subscribed events (`null` holes allowed on wire).
    pub events: Vec<Option<String>>,
    /// Domain scope (`null` holes allowed on wire).
    pub domains: Vec<Option<String>>,
    /// Active.
    pub active: bool,
    /// Meta jsonb.
    pub meta: Json,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Webhook detail view (`GET /tenant/webhooks/{hex}/detail` — no secret).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDetail {
    /// Hex.
    pub hex: String,
    /// URL.
    pub url: String,
    /// Events jsonb.
    pub events: Json,
    /// Active.
    pub active: bool,
    /// Created.
    pub created: String,
    /// Nested tenant `{ hex, name }`.
    pub tenant: HexName,
}

/// Tenant audit detail (`GET /tenant/view/audit/{hex}`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditDetail {
    /// Hex.
    pub hex: String,
    /// Action.
    pub action: String,
    /// Success.
    pub success: bool,
    /// Reason.
    #[serde(default)]
    pub reason: Option<String>,
    /// IP.
    #[serde(default)]
    pub ip: Option<String>,
    /// Agent.
    #[serde(default)]
    pub agent: Option<String>,
    /// Device.
    #[serde(default)]
    pub device: Option<Json>,
    /// Meta.
    #[serde(default)]
    pub meta: Option<Json>,
    /// Created.
    pub created: String,
    /// Actor.
    #[serde(default)]
    pub actor: Option<Json>,
}

/// Audit row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audit {
    /// Hex.
    pub hex: String,
    /// Action.
    pub action: String,
    /// Success.
    pub success: bool,
    /// Reason.
    #[serde(default)]
    pub reason: Option<String>,
    /// IP.
    #[serde(default)]
    pub ip: Option<String>,
    /// Agent.
    #[serde(default)]
    pub agent: Option<String>,
    /// Device.
    #[serde(default)]
    pub device: Option<Json>,
    /// Created.
    pub created: String,
    /// Actor.
    #[serde(default)]
    pub actor: Option<HexNameEmail>,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Usage row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Tenant.
    pub tenant: String,
    /// Metric.
    pub metric: String,
    /// Window date `YYYY-MM-DD`.
    pub window: String,
    /// Value.
    pub value: i64,
    /// Ceiling.
    pub ceiling: i64,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Tenant security snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Security {
    /// MTA-STS.
    pub mtasts: Vec<Json>,
    /// TLSA.
    pub tlsa: Vec<Json>,
    /// BIMI.
    pub bimi: Vec<Json>,
    /// Reports.
    pub reports: Vec<Json>,
}

/// Mail message list row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Hex.
    pub hex: String,
    /// UID.
    pub uid: i64,
    /// Subject.
    #[serde(default)]
    pub subject: Option<String>,
    /// Sender.
    #[serde(default)]
    pub sender: Option<String>,
    /// Size.
    pub size: i64,
    /// Flags.
    pub flags: Vec<Option<String>>,
    /// Thread.
    #[serde(default)]
    pub thread: Option<String>,
    /// Spam score.
    #[serde(default)]
    pub spam: Option<f64>,
    /// Internal date.
    pub internaldate: String,
    /// Mailbox.
    pub mailbox: HexName,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Thread list row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    /// Thread id.
    pub thread: String,
    /// Subject.
    #[serde(default)]
    pub subject: Option<String>,
    /// Count.
    pub count: i64,
    /// Unread.
    pub unread: i64,
    /// Latest.
    pub latest: String,
    /// Mailbox.
    pub mailbox: HexName,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Mailbox list row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mailbox {
    /// Hex.
    pub hex: String,
    /// Name.
    pub name: String,
    /// Delimiter.
    pub delimiter: String,
    /// Flags.
    pub flags: Vec<Option<String>>,
    /// UIDVALIDITY.
    pub uidvalidity: i64,
    /// UIDNEXT.
    pub uidnext: i64,
    /// Messages.
    pub messages: i64,
    /// Unread.
    pub unread: i64,
    /// Created.
    pub created: String,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Full mailbox model (create/update).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailboxModel {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Tenant.
    pub tenant: String,
    /// User.
    pub user: String,
    /// Name.
    pub name: String,
    /// Delimiter.
    pub delimiter: String,
    /// Flags.
    pub flags: Vec<Option<String>>,
    /// UIDVALIDITY.
    pub uidvalidity: i64,
    /// UIDNEXT.
    pub uidnext: i64,
    /// Modseq.
    pub modseq: i64,
    /// Meta.
    pub meta: Json,
    /// Role.
    #[serde(default)]
    pub role: Option<String>,
    /// Subscribed.
    pub subscribed: bool,
    /// Parent.
    #[serde(default)]
    pub parent: Option<String>,
    /// ACL.
    pub acl: Json,
    /// Quota.
    #[serde(default)]
    pub quota: Option<i64>,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Create mailbox body.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailboxData {
    /// Name.
    pub name: String,
    /// Role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Child.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child: Option<String>,
    /// Unread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread: Option<i64>,
    /// Empty.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    /// Messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<i64>,
    /// Search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<bool>,
    /// Uidnext.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uidnext: Option<i64>,
    /// Flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    /// Subscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed: Option<bool>,
    /// Parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Quota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    /// ACL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<Json>,
    /// Meta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Json>,
}

/// Contact list row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// Hex.
    pub hex: String,
    /// ETag.
    pub etag: String,
    /// Name.
    #[serde(default)]
    pub name: Option<String>,
    /// Emails.
    pub emails: Vec<Option<String>>,
    /// Phones.
    pub phones: Vec<Option<String>>,
    /// Groups.
    pub groups: Vec<Option<String>>,
    /// Created.
    pub created: String,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Full contact model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactModel {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Tenant.
    pub tenant: String,
    /// User.
    pub user: String,
    /// ETag.
    pub etag: String,
    /// vCard.
    pub vcard: String,
    /// Name.
    #[serde(default)]
    pub name: Option<String>,
    /// Emails.
    pub emails: Vec<Option<String>>,
    /// Phones.
    pub phones: Vec<Option<String>>,
    /// Groups.
    pub groups: Vec<Option<String>>,
    /// Meta.
    pub meta: Json,
    /// Book.
    #[serde(default)]
    pub book: Option<String>,
    /// Href.
    #[serde(default)]
    pub href: Option<String>,
    /// UID.
    #[serde(default)]
    pub uid: Option<String>,
    /// Version.
    #[serde(default)]
    pub version: Option<String>,
    /// Size.
    #[serde(default)]
    pub size: Option<i64>,
    /// Deleted.
    #[serde(default)]
    pub deleted: Option<String>,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Create contact body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactData {
    /// vCard body.
    pub vcard: String,
    /// Display name (required by API).
    pub name: String,
    /// Emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// Phones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<String>>,
    /// Groups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// Meta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Json>,
}

/// Calendar list row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar {
    /// Hex.
    pub hex: String,
    /// Name.
    pub name: String,
    /// Description.
    #[serde(default)]
    pub description: Option<String>,
    /// Color.
    #[serde(default)]
    pub color: Option<String>,
    /// Timezone.
    pub timezone: String,
    /// Created.
    pub created: String,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Calendar create result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarCreated {
    /// Hex.
    pub hex: String,
    /// ETag.
    pub etag: String,
    /// Sync token.
    #[serde(rename = "sync_token")]
    pub sync_token: String,
}

/// Event list row.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Hex.
    pub hex: String,
    /// UID.
    pub uid: String,
    /// Start.
    #[serde(default)]
    pub start: Option<String>,
    /// End.
    #[serde(default)]
    pub end: Option<String>,
    /// Created.
    pub created: String,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Manual feed sync result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedSync {
    /// Feed hex.
    pub hex: String,
    /// Whether sync succeeded.
    pub ok: bool,
}

/// Feed model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Tenant.
    pub tenant: String,
    /// User.
    pub user: String,
    /// Connection.
    pub connection: String,
    /// Remote URL.
    pub remote: String,
    /// Name.
    pub name: String,
    /// Color.
    #[serde(default)]
    pub color: Option<String>,
    /// Block.
    pub block: bool,
    /// Sync token.
    #[serde(default)]
    pub sync: Option<String>,
    /// Active.
    pub active: bool,
    /// Meta.
    pub meta: Json,
    /// Last sync.
    #[serde(default)]
    pub last: Option<String>,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Booking service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Tenant.
    pub tenant: String,
    /// User.
    pub user: String,
    /// Name.
    pub name: String,
    /// Slug.
    pub slug: String,
    /// Description.
    #[serde(default)]
    pub description: Option<String>,
    /// Duration minutes.
    pub duration: i64,
    /// Buffer.
    pub buffer: i64,
    /// Notice.
    pub notice: i64,
    /// Horizon.
    pub horizon: i64,
    /// Increment.
    pub increment: i64,
    /// Max.
    #[serde(default)]
    pub max: Option<i64>,
    /// Location.
    pub location: Json,
    /// Questions.
    pub questions: Json,
    /// Active.
    pub active: bool,
    /// Meta.
    pub meta: Json,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Appointment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appointment {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Tenant.
    pub tenant: String,
    /// Service.
    pub service: String,
    /// Host.
    pub host: String,
    /// Start.
    pub start: String,
    /// End.
    pub end: String,
    /// Timezone.
    pub timezone: String,
    /// Status.
    pub status: String,
    /// UID.
    pub uid: String,
    /// Sequence.
    pub sequence: i64,
    /// Method.
    pub method: String,
    /// Event.
    #[serde(default)]
    pub event: Option<String>,
    /// Location.
    #[serde(default)]
    pub location: Option<Json>,
    /// Notes.
    #[serde(default)]
    pub notes: Option<String>,
    /// Cancelled.
    #[serde(default)]
    pub cancelled: Option<String>,
    /// Rescheduled.
    #[serde(default)]
    pub rescheduled: Option<String>,
    /// Meta.
    pub meta: Json,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Guest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Guest {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Tenant.
    pub tenant: String,
    /// Appointment.
    pub appointment: String,
    /// User.
    #[serde(default)]
    pub user: Option<String>,
    /// Name.
    pub name: String,
    /// Email.
    pub email: String,
    /// Phone.
    #[serde(default)]
    pub phone: Option<String>,
    /// Status.
    pub status: String,
    /// Answers.
    pub answers: Json,
    /// Token.
    pub token: String,
    /// Notified.
    #[serde(default)]
    pub notified: Option<String>,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Availability window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Tenant.
    pub tenant: String,
    /// User.
    pub user: String,
    /// Name.
    pub name: String,
    /// Timezone.
    pub timezone: String,
    /// Priority.
    pub priority: i64,
    /// Start.
    #[serde(default)]
    pub start: Option<String>,
    /// End.
    #[serde(default)]
    pub end: Option<String>,
    /// Busy type.
    pub busytype: String,
    /// RRULE.
    #[serde(default)]
    pub rrule: Option<String>,
    /// Slots.
    pub slots: Json,
    /// Active.
    pub active: bool,
    /// Meta.
    pub meta: Json,
    /// Created.
    pub created: String,
    /// Updated.
    pub updated: String,
}

/// Availability override.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Override {
    /// Id.
    pub id: i64,
    /// Hex.
    pub hex: String,
    /// Tenant.
    pub tenant: String,
    /// User.
    pub user: String,
    /// Window.
    #[serde(default)]
    pub window: Option<String>,
    /// Start.
    pub start: String,
    /// End.
    pub end: String,
    /// Available.
    pub available: bool,
    /// Reason.
    #[serde(default)]
    pub reason: Option<String>,
    /// Created.
    pub created: String,
}

/// Availability response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Availability {
    /// Free slots.
    pub slots: Vec<Json>,
    /// Busy blocks.
    pub busy: Vec<Json>,
}

/// Create appointment body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppointmentData {
    /// Host user hex.
    pub host: String,
    /// Service hex.
    pub service: String,
    /// Start.
    pub start: String,
    /// End.
    pub end: String,
    /// Timezone.
    pub timezone: String,
    /// UID.
    pub uid: String,
    /// Method.
    pub method: String,
    /// Event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Json>,
    /// Notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Rescheduled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rescheduled: Option<String>,
    /// Meta.
    pub meta: Json,
}

/// Session list row (REST).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Hex.
    pub hex: String,
    /// IP.
    #[serde(default)]
    pub ip: Option<String>,
    /// Agent.
    #[serde(default)]
    pub agent: Option<String>,
    /// Device.
    #[serde(default)]
    pub device: Option<Json>,
    /// Location.
    #[serde(default)]
    pub location: Option<Json>,
    /// Seen.
    pub seen: String,
    /// Expires.
    pub expires: String,
    /// Created.
    pub created: String,
    /// User.
    pub user: HexNameEmail,
    /// Total.
    #[serde(default)]
    pub total: Option<i64>,
}

/// Hex-only ack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hex {
    /// Hex.
    pub hex: String,
}

/// Key create response (includes raw key once).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keypair {
    /// Key hex id.
    pub hex: String,
    /// Raw `hm_live_…` secret (only returned once).
    pub key: String,
}

/// `{ ok: true }` ack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ack {
    /// Always true on success.
    pub ok: bool,
}
