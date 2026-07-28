# Hosts & testing

## Hosts

| Host | Role |
| --- | --- |
| `hermers.aduki.pro` | REST `/v1` |
| `grpc.aduki.pro:443` | Native gRPC over TLS (DNS-only — not Cloudflare HTTP proxy) |

Default SDK endpoints:

| Client | Feature | Default |
| --- | --- | --- |
| `Hermes` | `rest` | `https://hermers.aduki.pro/v1` |
| `HermesGrpc` | `grpc` | `grpc.aduki.pro:443` |

Override endpoints only for local/dev (`RestConfig::api_base`, `GrpcConfig::{endpoint, insecure}`).

## Authentication

Every REST request:

```http
Authorization: Key hm_live_…
```

gRPC metadata: `authorization: Key hm_live_…`.

## Testing

```bash
cargo test
export HERMERS_API_KEY=hm_live_…   # never commit secrets
cargo test
```

Without `HERMERS_API_KEY`, unit/mocks still run; live suites no-op. Do not use a misspelled env var name — the prefix is **Hermers** (`HERMERS_API_KEY`).
