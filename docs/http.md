# REST cheatsheet

```rust
use hermers::{Hermes, BASE_URL, HermesError};
use hermers::types::Query;

#[tokio::main]
async fn main() -> Result<(), HermesError> {
    let hermes = Hermes::new(std::env::var("HERMERS_API_KEY").unwrap())?;
    hermes.ready().await?;

    // Contacts
    let contacts = hermes.contacts.list(Some(Query { limit: Some(50), ..Default::default() })).await?;

    // Mail
    let inbox = hermes.mail.inbox(Some(Query { limit: Some(50), ..Default::default() })).await?;

    // User
    let user = hermes.user.retrieve().await?;

    // Keys — raw secret returned once
    let created = hermes.keys.create("ci", &["contacts:read".into()], None, None, None).await?;
    // created.hex, created.key

    let hook = hermes.tenant.create_webhook(&serde_json::json!({
        "url": "https://api.example.com/hooks",
        "secret": "whsec_xxxxxxxxxxxxxxxx",
        "events": ["message.sent"],
    })).await?;
    let _ = (contacts.total, inbox.total, user.hex, created.hex, hook.hex, BASE_URL);
    Ok(())
}
```

Base: `https://hermers.aduki.pro/v1` · Auth: `Authorization: Key <apiKey>`

| Area | Prefix |
| --- | --- |
| Whoami | `GET /auth/whoami` |
| Contacts | `/user/contacts` |
| Mail / mailbox | `/user/mail`, `/user/mailbox` |
| User | `/user` |
| Tenant | `/tenant` |
| Keys | `/user/keys`, `/tenant/keys` |
| Calendar / events / feeds | `/user/calendars`, `/user/events`, `/user/feeds` |
| Scheduling | `/book/…`, `/user/appointments`, `/user/services` |

Guides: [rest/index.md](rest/index.md) · [Types](types/index.md) · [Auth](rest/services/auth.md).
