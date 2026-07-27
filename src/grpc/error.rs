//! gRPC errors.

/// Structured gRPC error.
#[derive(Debug, Clone)]
pub struct HermesGrpcError {
    /// Human-readable message.
    pub message: String,
    /// Status name (e.g. `NOT_FOUND`, `UNAVAILABLE`).
    pub code: String,
    /// Numeric gRPC status code when known.
    pub grpc_code: Option<i32>,
}

impl HermesGrpcError {
    /// Build an error.
    pub fn new(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: code.into(),
            grpc_code: None,
        }
    }

    /// From a tonic status.
    pub fn from_status(status: tonic::Status) -> Self {
        Self {
            message: status.message().to_string(),
            code: format!("{:?}", status.code()),
            grpc_code: Some(status.code() as i32),
        }
    }
}

impl std::fmt::Display for HermesGrpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl std::error::Error for HermesGrpcError {}

impl From<tonic::Status> for HermesGrpcError {
    fn from(s: tonic::Status) -> Self {
        Self::from_status(s)
    }
}
