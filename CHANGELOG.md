# Changelog

## 0.1.0 — 2026-07-28

Initial open-source Rust SDK release.

- REST client (`Hermes`) — API-key auth, whoami cache, base `https://hermers.aduki.pro/v1`
- gRPC client (`HermesGrpc`) — TLS to `grpc.aduki.pro:443`, Session whoami cache
- Feature flags: `rest` + `grpc` (both default); gRPC TLS uses native roots (`tls-native-roots`) for current CAs
- Unit tests with injectable HTTP backend; live suites gated on `HERMERS_API_KEY`
- Edition 2024; modern `reqwest`/`tonic`/`serde`/`tokio` stack
