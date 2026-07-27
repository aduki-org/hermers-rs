# `hermers::grpc` — native gRPC client

Native gRPC over TLS via `tonic`. Types/stubs are generated from Hermers `proto/*.proto` at build time.

## Install

```toml
[dependencies]
hermers = { version = "0.1", features = ["grpc"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

```rust
use hermers::{HermesGrpc, HermesGrpcError, BASE_ENDPOINT};

#[tokio::main]
async fn main() -> Result<(), HermesGrpcError> {
    let client = HermesGrpc::connect("hm_live_xxxxxxxxxxxxxxxxxxxxxxxx").await?;
    client.ready().await?;
    Ok(())
}
```

## Defaults

| Setting | Value |
| --- | --- |
| Endpoint | `grpc.aduki.pro:443` (`BASE_ENDPOINT`) |
| Transport | TLS / HTTP2 (not grpc-web, not plaintext in production) |
| Auth | metadata `authorization: Key <apiKey>` |

Local override:

```rust
use hermers::{GrpcConfig, HermesGrpc};

let client = HermesGrpc::connect_with(
    std::env::var("HERMERS_API_KEY").unwrap(),
    GrpcConfig {
        endpoint: Some("127.0.0.1:8444".into()),
        insecure: true, // plaintext h2c for local only
    },
)
.await?;
```

Drop the client when finished (no explicit `close` required).

## Identity

| API | Signature | Behavior |
| --- | --- | --- |
| `ready()` | `async → Result<Identity>` | Awaits `SessionService.Whoami` |
| `whoami()` | `async → Result<Identity>` | Cached after first success |
| `me()` | `→ Option<Identity>` | Sync snapshot |

```rust
// Identity fields (logical)
// hex: Option<String>
// user: String
// tenant: String
// owner: Option<bool>
// scopes / deny: Option<Vec<String>>
// tier: Option<String>
// raw: Option<Session>  // proto message
```

Whoami fills `tenant` / `user` for resource RPCs — callers never pass them.

## Resources

| Property | Docs |
| --- | --- |
| `contacts` | [Contact](services/contact.md) |
| `mail` | [Mail](services/mail.md) |
| `feeds` | [Feed](services/feed.md) |
| `storage` | [Storage](services/storage.md) |
| `sync` | [Sync](services/sync.md) |
| `security` | [Security](services/security.md) |
| `spam` | [Spam](services/spam.md) |
| `tier` | [Tier](services/tier.md) |
| `usage` | [Usage](services/usage.md) |
| `sessions` | [Session](services/session.md) |

## Errors

Failed RPCs return `HermesGrpcError` with `code` (status name such as `NOT_FOUND`), `message`, and optional `grpc_code`.

## See also

Crate README: [`../../README.md`](../../README.md) · [Types](../types/index.md) — REST JSON shapes differ from proto messages.
