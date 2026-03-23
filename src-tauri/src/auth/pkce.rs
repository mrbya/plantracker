use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};

/// Generates a PKCE code verifier: 64 random bytes base64url-encoded → 86-char string.
/// Entropy: 512 bits — well within RFC 7636's 43–128 character range.
pub fn generate_code_verifier() -> String {
    let mut bytes = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Derives the PKCE code challenge: SHA-256 of the verifier, base64url-encoded.
pub fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}

/// Generates a random OAuth state token for CSRF protection: 16 bytes hex-encoded → 32 chars.
pub fn generate_state() -> String {
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use sha2::{Digest, Sha256};

    #[test]
    fn verifier_length_within_rfc7636_range() {
        let verifier = generate_code_verifier();
        assert!(
            verifier.len() >= 43 && verifier.len() <= 128,
            "verifier length {} is outside RFC 7636 range 43–128",
            verifier.len()
        );
    }

    #[test]
    fn verifier_is_base64url() {
        let verifier = generate_code_verifier();
        assert!(
            verifier
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'),
            "verifier contains non-base64url characters: {verifier}"
        );
    }

    #[test]
    fn challenge_is_sha256_of_verifier() {
        let verifier = generate_code_verifier();
        let expected = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
        assert_eq!(generate_code_challenge(&verifier), expected);
    }

    #[test]
    fn state_is_32_hex_chars() {
        let state = generate_state();
        assert_eq!(
            state.len(),
            32,
            "state length should be 32, got {}",
            state.len()
        );
        assert!(
            state
                .chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()),
            "state contains non-lowercase-hex characters: {state}"
        );
    }

    #[test]
    fn successive_verifiers_differ() {
        let v1 = generate_code_verifier();
        let v2 = generate_code_verifier();
        assert_ne!(v1, v2, "consecutive verifiers must differ");
    }
}
