# Tenant (`hermes.tenant`)

Tenant administration under `/tenant/*`. API keys: [`hermes.keys`](auth.md).

```rust
use hermers::types::Query;

let tenant = hermes.tenant.retrieve().await?;
let members = hermes.tenant.members(Some(Query { limit: Some(50), ..Default::default() })).await?;
```

## Profile

| SDK | HTTP | Returns |
| --- | --- | --- |
| `retrieve()` | `GET /tenant` | `TenantProfile` |
| `update({ name })` | `PATCH /tenant/edit` | full `Tenant` model |
| `view(hex)` | `GET /tenant/view/{hex}` | `TenantProfile` |
| `by_slug(slug)` | `GET /tenant/view/slug/{slug}` | `TenantSummary` |

### `TenantProfile`

| Field | Type | Nullable |
| --- | --- | --- |
| `hex` / `kind` / `name` / `slug` / `plan` / `state` | string | no |
| `domain` / `customer` / `subscription` | string | yes |
| `trial` | datetime | yes |
| `meta` | object | no |
| `created` | datetime | no |
| `users` / `domains` / `storage` | number | no |

`kind`: `personal`\|`team`. `plan`: `free`\|`starter`\|`pro`\|`business`\|`enterprise`. `state`: `active`\|`suspended`\|`pending`\|`deleted`.

## Members — `Page<Members>`

| Field | Type | Nullable |
| --- | --- | --- |
| `hex` / `email` / `name` / `state` | string | no |
| `avatar` | string | yes |
| `owner` | boolean | no |
| `last` | datetime | yes |
| `created` | datetime | no |
| `tenant` | `{ hex, name, slug }` | no |
| `role` | `{ label, kind }` or `{}` | no |
| `total` | number | no |

```json
{
  "hex": "U0X…",
  "email": "ada@example.com",
  "name": "Ada",
  "avatar": null,
  "owner": false,
  "state": "active",
  "last": null,
  "created": "2026-01-01T00:00:00",
  "tenant": { "hex": "T0X…", "name": "Acme", "slug": "acme" },
  "role": { "label": "member", "kind": "permanent" },
  "total": 3
}
```

| SDK | Returns |
| --- | --- |
| `members` / `active_members` / `owners` / `search_members` | `Page<Members>` |
| `invite({ email, role? })` | `{ invite: string, token: string }` |
| `remove_member(user)` | typically `null` / ack |

## Invitations — `Page<Invitations>`

| Field | Type |
| --- | --- |
| `hex` / `email` / `label` / `status` | string |
| `expires` / `created` | datetime |
| `inviter` | `{ hex, name, email }` |
| `total` | number |

`status`: `pending`\|`accepted`\|`rejected`\|`expired`.

## Domains

**Create body:** `{ name, kind?, selector?, meta? }` → `{ hex }`.  
`kind`: `primary`\|`sending`\|`receiving`\|`alias`.

**List row:** `hex`, `name`, `kind`, `status`, `verified?`, `created`, `tenant: { hex, name, slug }`, `total`.

## Quotas — `Page<Quotas>`

| Field | Type |
| --- | --- |
| `tenant` / `metric` | string |
| `ceiling` | number |
| `expires` | datetime? |
| `created` | datetime |
| `total` | number |

Create body: `{ metric, ceiling, reason?, granted?, expires? }` → full quota model.

## Rules — `Page<Rules>`

| Field | Type |
| --- | --- |
| `hex` / `target` / `pattern` / `name` | string |
| `score` | number |
| `active` | boolean |
| `created` | datetime |
| `tenant` | `{ hex, name }` \| null |
| `total` | number |

`target` values include `header.from`, `header.to`, `body.text`, `envelope.from`, … (see API data validation).

Detail (`RuleDetail`) adds `meta` object.

## Webhooks (`hermes.tenant`)

Scopes: `webhooks:read` / `webhooks:write`. Signing: HMAC-SHA256 via `X-Webhook-Signature`.

```rust
use hermers::types::Query;
use serde_json::json;

let created = hermes
    .tenant
    .create_webhook(&json!({
        "url": "https://api.example.com/hooks/hermes",
        "secret": "whsec_xxxxxxxxxxxxxxxx",
        "events": ["message.sent", "message.received"],
        "domains": ["example.com"],
        "active": true,
    }))
    .await?;

let page = hermes
    .tenant
    .webhooks(Some(Query { limit: Some(50), ..Default::default() }))
    .await?;

hermes
    .tenant
    .update_webhook_url(&created.hex, "https://api.example.com/hooks/v2")
    .await?;
hermes.tenant.delete_webhook(&created.hex).await?;
```

| SDK | HTTP | Returns |
| --- | --- | --- |
| `webhooks` | `GET /tenant/webhooks` | `Page<Webhook>` |
| `active_webhooks` | `GET /tenant/webhooks/active` | `Page<Webhook>` |
| `webhook_subscribers(event)` | `GET /tenant/webhooks/subscribers/{event}` | `Page<Webhook>` |
| `create_webhook` | `POST /tenant/webhooks` | `Hex` (`{ hex }`) |
| `retrieve_webhook` | `GET /tenant/webhooks/{hex}` | `WebhookModel` (includes `secret`) |
| `detail_webhook` | `GET /tenant/webhooks/{hex}/detail` | `WebhookDetail` (no secret) |
| `update_webhook_active` | `PATCH …/active` | `Ack` |
| `update_webhook_domains` | `PATCH …/domains` | `Ack` |
| `update_webhook_events` | `PATCH …/events` | `Ack` |
| `update_webhook_secret` | `PATCH …/secret` | `Ack` |
| `update_webhook_url` | `PATCH …/url` | `Ack` |
| `delete_webhook` | `DELETE /tenant/webhooks/{hex}` | `Ack` |

### Create body

| Field | Type | Required |
| --- | --- | --- |
| `url` | string | yes |
| `secret` | string | yes (16–256 chars) |
| `events` | string[] | no |
| `domains` | string[] | no |
| `active` | boolean | no |
| `meta` | object | no |

### List row — `Webhook`

| Field | Type | Nullable |
| --- | --- | --- |
| `hex` / `url` | string | no |
| `active` | boolean | no |
| `created` | datetime | no |
| `tenant` | `{ hex, name }` | no |
| `total` | number | no |

### Model — `WebhookModel` (`retrieve_webhook`)

| Field | Type |
| --- | --- |
| `id` | i64 |
| `hex` / `tenant` / `url` / `secret` | string (`tenant` is hex, not nested) |
| `events` / `domains` | `Vec<Option<String>>` |
| `active` | bool |
| `meta` | jsonb |
| `created` / `updated` | datetime |

### Detail — `WebhookDetail` (`detail_webhook`)

`hex`, `url`, `events` (jsonb), `active`, `created`, `tenant: { hex, name }` — no `secret`.

## Usage — `Page<Usages>` / summary array

| Field | Type |
| --- | --- |
| `tenant` / `metric` | string |
| `window` | date (`YYYY-MM-DD`) |
| `value` / `ceiling` / `total` | number |

## Security — `GET /tenant/security`

```json
{
  "mtasts": [{ "domain": "…", "policy": {}, "expires": "…" }],
  "tlsa": [{ "host": "…", "port": 25, "records": {}, "expires": "…" }],
  "bimi": [{ "domain": "…", "location": null, "vmc": null, "expires": "…" }],
  "reports": [{ "hex": "…", "kind": "…", "domain": "…", "period": "2026-07-01", "received": "…" }]
}
```

Note REST uses `policy` / `records` objects (not `policyJson` / `recordsJson` — those names are gRPC-generated).


## Admin actions

| SDK | HTTP | Returns |
| --- | --- | --- |
| `promote()` | `POST /tenant/promote` | `Ack { ok: true }` (owner-only) |
| `transfer(tenant, from, to)` | `POST /tenant/transfer/{tenant}/{from}/{to}` | `null` / empty |
| `view_audit(hex)` | `GET /tenant/view/audit/{hex}` | `AuditDetail` |

### `AuditDetail`

| Field | Type | Nullable |
| --- | --- | --- |
| `hex` / `action` | string | no |
| `success` | boolean | no |
| `reason` / `ip` / `agent` | string | yes |
| `device` / `meta` / `actor` | object | yes |
| `created` | datetime | no |

## Tenant audits — `Page<Audit>`

| SDK | HTTP |
| --- | --- |
| `audits(query?)` | `GET /tenant/audits` |
| `audits_by_action(action, query?)` | `GET /tenant/audits/action/{action}` |
| `failed_audits` / `successful_audits` | `GET /tenant/audits/failed` / `…/successful` |
| `audits_by_ip(ip, query?)` | `GET /tenant/audits/ip/{ip}` |
| `audits_by_user(user, query?)` | `GET /tenant/audits/user/{user}` |
| `audits_by_user_action(user, action, query?)` | `GET /tenant/audits/user/{user}/action/{action}` |
| `failed_audits_by_user` / `successful_audits_by_user` | `GET …/user/{user}/failed` / `…/successful` |
| `audits_by_user_ip(user, ip, query?)` | `GET /tenant/audits/user/{user}/ip/{ip}` |

List row fields match user audits (`hex`, `action`, `success`, `reason?`, `ip?`, `agent?`, `device?`, `created`, `actor?`, `total`).

## Domain filters & patches

| SDK | HTTP | Returns |
| --- | --- | --- |
| `active_domains(query?)` | `GET /tenant/domains/active` | `Page<Domain>` |
| `pending_domains(query?)` | `GET /tenant/domains/pending` | `Page<Domain>` |
| `domains_by_status(status, query?)` | `GET /tenant/domains/status` with JSON body `{ status }` | `Page<Domain>` |
| `domain_by_name(name)` | `GET /tenant/domains/name/{name}` | `Domain` |
| `lookup_domain_by_name(name)` | `GET /tenant/domains/lookup/name` with JSON body `{ name }` | `Domain` |
| `update_domain_kind(hex, kind)` | `PATCH …/{hex}/kind` `{ kind }` | `Ack { ok: true }` |
| `update_domain_name(hex, name)` | `PATCH …/{hex}/name` `{ name }` | `Ack { ok: true }` |
| `update_domain_status(hex, body)` | `PATCH …/{hex}/status` | `Ack { ok: true }` |
| `update_domain_dkim(hex, dkim)` | `PATCH …/{hex}/dkim` `{ dkim }` | `Ack { ok: true }` |
| `update_domain_selector(hex, selector)` | `PATCH …/{hex}/selector` `{ selector }` | `Ack { ok: true }` |
| `update_domain_auth(hex, body)` | `PATCH …/{hex}/auth` | `Ack { ok: true }` |
| `update_domain_meta(hex, meta)` | `PATCH …/{hex}/meta` `{ meta }` | `Ack { ok: true }` |

`status` values: `pending`\|`verified`\|`active`\|`suspended`\|`failed`.

## Invitation filters

| SDK | HTTP | Returns |
| --- | --- | --- |
| `pending_invitations(query?)` | `GET /tenant/invitations/pending` | `Page<Invitation>` |
| `expired_invitations(query?)` | `GET /tenant/invitations/expired` | `Page<Invitation>` |
| `invitations_by_status(status, query?)` | `GET /tenant/invitations/status/{status}` | `Page<Invitation>` |

## Quota by metric

| SDK | HTTP | Returns |
| --- | --- | --- |
| `retrieve_quota(metric)` | `GET /tenant/quotas/{metric}` | `Quota` |
| `update_quota_ceiling(metric, ceiling)` | `PATCH …/{metric}/ceiling` `{ ceiling }` | `Ack { ok: true }` |
| `update_quota_expires(metric, expires)` | `PATCH …/{metric}/expires` `{ expires }` | `Ack { ok: true }` |
| `update_quota_reason(metric, reason)` | `PATCH …/{metric}/reason` `{ reason }` | `Ack { ok: true }` |
| `delete_quota(metric)` | `DELETE /tenant/quotas/{metric}` | `Ack { ok: true }` |

## Rule filters & patches

| SDK | HTTP | Returns |
| --- | --- | --- |
| `active_rules(query?)` | `GET /tenant/rules/active` | `Page<Rule>` |
| `rules_by_target(target, query?)` | `GET /tenant/rules/target/{target}` | `Page<Rule>` |
| `get_rule(hex)` | `GET /tenant/rules/{hex}` | `Rule` |
| `rule_by_name(name)` | `GET /tenant/rules/name/{name}` | `Rule` |
| `retrieve_rule(hex)` | `GET /tenant/rules/{hex}/detail` | `RuleDetail` |
| `update_rule_active(hex, active)` | `PATCH …/{hex}/active` `{ active }` | `Ack { ok: true }` |
| `update_rule_name(hex, name)` | `PATCH …/{hex}/name` `{ name }` | `Ack { ok: true }` |
| `update_rule_pattern(hex, pattern)` | `PATCH …/{hex}/pattern` `{ pattern }` | `Ack { ok: true }` |
| `update_rule_score(hex, score)` | `PATCH …/{hex}/score` `{ score }` | `Ack { ok: true }` |
| `update_rule_target(hex, target)` | `PATCH …/{hex}/target` `{ target }` | `Ack { ok: true }` |
| `delete_rule(hex)` | `DELETE /tenant/rules/{hex}` | `Ack { ok: true }` |

## Usage by metric

| SDK | HTTP | Returns |
| --- | --- | --- |
| `usage_by_metric(metric, query?)` | `GET /tenant/usage/metric/{metric}` | `Page<Usage>` |

## Errors

`{ "error": "…", "message": "…" }` — see [Types](../../types/index.md).
