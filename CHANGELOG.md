# Changelog

## 0.2.0 — 2026-07-28

### Added

- Full API-key REST coverage (~78 previously missing routes): tenant audits/domains/quotas/rules/usage/invitations filters, key expired/user/prefix/hash/last, calendar update/remove, feeds update/sync, mail folder + mailbox field patches, user audit/session filters, scheduling active/guests/status.
- Types: `AuditDetail`, `FeedSync`.
- Webhooks: full surface (`active_webhooks`, `webhook_subscribers`, `detail_webhook`, field updates).

### Fixed

- `active_sessions` → `GET /user/sessions` (not `/user/sessions/active`).

### Changed

- `retrieve_webhook` returns **`WebhookModel`** (includes `secret`). Use `detail_webhook` for the nested detail view without secret.

## 0.1.0 — 2026-07-28

Initial open-source Rust SDK release.

- REST client (`Hermes`) — API-key auth, whoami cache, base `https://hermers.aduki.pro/v1`
- gRPC client (`HermesGrpc`) — TLS to `grpc.aduki.pro:443`, Session whoami cache
- Feature flags: `rest` + `grpc` (both default); gRPC TLS uses native roots (`tls-native-roots`) for current CAs
- Unit tests with injectable HTTP backend; live suites gated on `HERMERS_API_KEY`
- Edition 2024; modern `reqwest`/`tonic`/`serde`/`tokio` stack
