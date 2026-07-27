# `hermers::rest` — REST client

Stripe/Square-style root client (`Hermes`). Pass an API key; call resource methods. Tenant and user come from whoami — never pass hex IDs.

Field tables and JSON examples describe the **HTTP wire shapes** the API returns.

## Install

```toml
[dependencies]
hermers = { version = "0.1", default-features = false, features = ["rest"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

```rust
use hermers::{Hermes, HermesError, BASE_URL};

#[tokio::main]
async fn main() -> Result<(), HermesError> {
    let hermes = Hermes::new("hm_live_xxxxxxxxxxxxxxxxxxxxxxxx")?;
    hermes.ready().await?;
    Ok(())
}
```

## Defaults

| Setting | Value |
| --- | --- |
| Base URL | `https://hermers.aduki.pro/v1` (`BASE_URL`) |
| Auth | `Authorization: Key <apiKey>` |

Override for local/dev:

```rust
use hermers::{Hermes, RestConfig};

let hermes = Hermes::with_options(
    std::env::var("HERMERS_API_KEY").unwrap(),
    RestConfig {
        api_base: Some("http://127.0.0.1:8443/v1".into()),
    },
)?;
```

## Identity

| API | Behavior |
| --- | --- |
| `ready()` / `whoami()` | `GET /auth/whoami` → cache |
| `me()` | Cached `Identity` or `None` |

Whoami fields: `hex`, `user`, `tenant`, `owner`, `scopes`, `deny`, `tier`, `ip`, `agent` — see [Auth](services/auth.md).

```rust
let me = hermes.ready().await?;
assert!(!me.user.is_empty() && !me.tenant.is_empty());
```

## Resources

| Property | Docs |
| --- | --- |
| `contacts` | [Contacts](services/contacts.md) |
| `mail` | [Mail](services/mail.md) |
| `keys` | [Authentication & keys](services/auth.md) |
| `user` | [User](services/user.md) |
| `tenant` | [Tenant](services/tenant.md) |
| `calendar` | [Calendar](services/calendar.md) |
| `events` | [Events](services/events.md) |
| `feeds` | [Feeds](services/feeds.md) |
| `scheduling` | [Scheduling](services/scheduling.md) |

## Common envelopes

**Page:** `{ items, total, next? }` or `{ items, total, page?, pages? }` — Rust type `Page<T>` with query `Query`.

**Empty ack:** many PATCH/DELETE endpoints return JSON `null` (deserialized as `serde_json::Value::Null`).

**Errors:**

```json
{ "error": "forbidden", "message": "…" }
```

Returned as `HermesError { status, code, message, body }`. See [Types](../types/index.md).

## See also

Crate README: [`../../README.md`](../../README.md)
