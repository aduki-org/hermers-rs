//! Structured REST API errors.

use serde_json::Value;

/// Structured API error matching Hermers flat `{ error, message }` envelopes.
#[derive(Debug, Clone)]
pub struct HermesError {
    /// Human-readable message.
    pub message: String,
    /// HTTP status (0 for client-side / network errors).
    pub status: u16,
    /// Machine-readable code (e.g. `forbidden`, `network_error`).
    pub code: String,
    /// Raw response body when available.
    pub body: Option<Value>,
}

impl HermesError {
    /// Build an error.
    pub fn new(message: impl Into<String>, status: u16, code: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status,
            code: code.into(),
            body: None,
        }
    }

    /// Attach a parsed JSON body.
    pub fn with_body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// Parse API errors: `{ "error": "forbidden", "message": "…" }`.
    pub fn from_response(status: u16, status_text: &str, body: Option<Value>) -> Self {
        let fallback = if status_text.is_empty() {
            format!("HTTP {status}")
        } else {
            status_text.to_string()
        };

        let Some(Value::Object(map)) = body.as_ref() else {
            let msg = match &body {
                Some(Value::String(s)) if !s.is_empty() => s.clone(),
                _ => fallback,
            };
            return Self::new(msg, status, "http_error").with_body_opt(body);
        };

        let top_message = map
            .get("message")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(str::to_string);

        match map.get("error") {
            Some(Value::String(code)) => Self {
                message: top_message.unwrap_or(fallback),
                status,
                code: code.clone(),
                body,
            },
            Some(Value::Object(err)) => {
                let code = err
                    .get("code")
                    .and_then(|v| v.as_str())
                    .unwrap_or("http_error")
                    .to_string();
                let message = err
                    .get("message")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .map(str::to_string)
                    .or(top_message)
                    .unwrap_or(fallback);
                Self {
                    message,
                    status,
                    code,
                    body,
                }
            }
            _ => Self::new(fallback, status, "http_error").with_body_opt(body),
        }
    }

    fn with_body_opt(mut self, body: Option<Value>) -> Self {
        self.body = body;
        self
    }
}

impl std::fmt::Display for HermesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl std::error::Error for HermesError {}
