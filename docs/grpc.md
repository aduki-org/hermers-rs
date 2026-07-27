# gRPC cheatsheet

```rust
use hermers::{HermesGrpc, BASE_ENDPOINT, HermesGrpcError};

#[tokio::main]
async fn main() -> Result<(), HermesGrpcError> {
    let client = HermesGrpc::connect(std::env::var("HERMERS_API_KEY").unwrap()).await?;
    client.ready().await?;

    let contacts = client.contacts.list(None, Some(50)).await?;
    let mailboxes = client.mail.list_mailboxes().await?;
    let tier = client.tier.resolve().await?;
    let _ = (contacts.items.len(), mailboxes.items.len(), tier, BASE_ENDPOINT);
    Ok(())
}
```

Endpoint: `grpc.aduki.pro:443` (TLS) · Auth metadata: `authorization: Key <apiKey>`

| Service | Package | Resource |
| --- | --- | --- |
| Session | `hermes.session` | `sessions` |
| Contact | `hermes.contact` | `contacts` |
| Mail | `hermes.mail` | `mail` |
| Feed | `hermes.feeds` | `feeds` |
| Storage | `hermes.storage` | `storage` |
| Sync | `hermes.sync` | `sync` |
| Security | `hermes.security` | `security` |
| Spam | `hermes.spam` | `spam` |
| Tier | `hermes.tier` | `tier` |
| Usage | `hermes.usage` (`Usageervice`) | `usage` |

Guides: [grpc/index.md](grpc/index.md) · [Types](types/index.md).
