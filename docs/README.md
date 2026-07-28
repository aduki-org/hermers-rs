# Hermers Rust SDK

Open-source Rust client for Hermers (**`hermers` crate**). The Hermers **server is private / proprietary** — this package and docs describe the public client APIs only.

| Module | Transport | Default endpoint |
| --- | --- | --- |
| [`rest`](rest/index.md) (`Hermes`) | REST / JSON | `https://hermers.aduki.pro/v1` |
| [`grpc`](grpc/index.md) (`HermesGrpc`) | Native gRPC (TLS) | `grpc.aduki.pro:443` |

## Install

```toml
[dependencies]
hermers = "0.2"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

REST only:

```toml
hermers = { version = "0.2", default-features = false, features = ["rest"] }
```

## Quickstart

```rust
use hermers::Hermes;

#[tokio::main]
async fn main() -> Result<(), hermers::HermesError> {
    let hermes = Hermes::new("hm_live_xxxxxxxxxxxxxxxxxxxxxxxx")?;
    let me = hermes.ready().await?; // GET /auth/whoami — caches user + tenant
    println!("{} / {}", me.tenant, me.user);
    let contacts = hermes.contacts.list(None).await?; // never pass tenant/user hex
    println!("{} contacts", contacts.total);
    Ok(())
}
```

```rust
use hermers::HermesGrpc;

#[tokio::main]
async fn main() -> Result<(), hermers::HermesGrpcError> {
    let client = HermesGrpc::connect("hm_live_xxxxxxxxxxxxxxxxxxxxxxxx").await?;
    client.ready().await?; // SessionService.Whoami
    let list = client.contacts.list(None, Some(50)).await?;
    println!("{} items", list.items.len());
    Ok(())
}
```

## Authentication

**API key only.** Every REST request sends:

```http
Authorization: Key hm_live_…
```

gRPC metadata: `authorization: Key hm_live_…`.

There are **no** login, password, or JWT refresh helpers. Prefer `await client.ready()` before the first resource call.

## Hosts

| Host | Role |
| --- | --- |
| `hermers.aduki.pro` | REST `/v1` |
| `grpc.aduki.pro:443` | Native gRPC over TLS |

Override endpoints only for local/dev (`RestConfig::api_base`, `GrpcConfig::{endpoint, insecure}`).

## Testing

```bash
cargo test
export HERMERS_API_KEY=hm_live_…   # never commit secrets
cargo test
```

Without `HERMERS_API_KEY`, unit/mocks still run; live suites no-op.

## Documentation

- [REST client (`rest`)](rest/index.md)
- [gRPC client (`grpc`)](grpc/index.md)
- [Types & enums](types/index.md)
- [REST cheatsheet](http.md)
- [gRPC cheatsheet](grpc.md)

### REST resources

- [Contacts](rest/services/contacts.md) · [Mail](rest/services/mail.md) · [User](rest/services/user.md) · [Tenant](rest/services/tenant.md)
- [Keys](rest/services/auth.md) · [Calendar](rest/services/calendar.md) · [Events](rest/services/events.md) · [Feeds](rest/services/feeds.md) · [Scheduling](rest/services/scheduling.md)

### gRPC resources

- [Session](grpc/services/session.md) · [Contact](grpc/services/contact.md) · [Mail](grpc/services/mail.md) · [Feed](grpc/services/feed.md)
- [Storage](grpc/services/storage.md) · [Sync](grpc/services/sync.md) · [Security](grpc/services/security.md) · [Spam](grpc/services/spam.md)
- [Tier](grpc/services/tier.md) · [Usage](grpc/services/usage.md)
