# Changelog

## 0.1.0 — 2026-07-28

Initial open-source Rust SDK release.

- REST client (`Hermes`) mirroring `@hermers/sdk` v3 — API-key auth, whoami cache, production base `https://hermers.aduki.pro/v1`
- gRPC client (`HermesGrpc`) mirroring `@hermers/grpc` v3 — TLS to `grpc.aduki.pro:443`, Session whoami cache
- Feature flags: `rest` (default), `grpc` (default); disable either to shrink the dependency tree
- Unit tests with injectable HTTP backend; live suites gated on `HERMERS_API_KEY`
