//! REST client configuration.

/// Production REST API base (includes `/v1`).
pub const BASE_URL: &str = "https://hermers.aduki.pro/v1";

/// Options for [`crate::Hermes`].
#[derive(Debug, Clone, Default)]
pub struct HermesOptions {
    /// Override the API base URL. Defaults to [`BASE_URL`].
    /// Use only for local/dev/test against a non-production stack.
    pub api_base: Option<String>,
}
