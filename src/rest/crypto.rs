//! Client-side API key helpers.

use sha2::{Digest, Sha256};

/// Generate a raw API key: `hm_live_` + 64 hex chars.
pub fn generate_key() -> String {
    let mut buf = [0u8; 32];
    getrandom::fill(&mut buf).expect("getrandom");
    let mut out = String::with_capacity(8 + 64);
    out.push_str("hm_live_");
    for b in buf {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

/// SHA-256 hex digest of the raw key (what the server stores).
pub fn hash_key(raw: &str) -> String {
    let digest = Sha256::digest(raw.as_bytes());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

/// First 16 characters of the raw key (server-side index only).
pub fn prefix_key(raw: &str) -> String {
    raw.chars().take(16).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_shape() {
        let key = generate_key();
        assert!(key.starts_with("hm_live_"));
        assert_eq!(key.len(), 8 + 64);
        assert_eq!(prefix_key(&key), &key[..16]);
        assert_eq!(hash_key(&key).len(), 64);
    }
}
