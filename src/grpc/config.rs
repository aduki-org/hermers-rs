//! gRPC client configuration.

/// Production gRPC endpoint (host:port, TLS).
pub const BASE_ENDPOINT: &str = "grpc.aduki.pro:443";

/// Options for [`crate::HermesGrpc`].
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Override endpoint (`host:port`). Defaults to [`BASE_ENDPOINT`].
    pub endpoint: Option<String>,
    /// Use plaintext (h2c) — tests / local only.
    pub insecure: bool,
}
