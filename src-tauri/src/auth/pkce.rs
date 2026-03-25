use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};

/// Generates a PKCE code verifier suitable for use with the Microsoft identity platform.
///
/// The verifier is derived from 64 cryptographically random bytes produced by
/// [`rand::thread_rng`], which internally uses the operating system's CSPRNG. Those
/// 64 bytes (512 bits of entropy) are base64url-encoded without padding, yielding an
/// 86-character string.
///
/// RFC 7636 requires the verifier to be between 43 and 128 characters. At 86 characters
/// this implementation sits comfortably within that range while maximising entropy.
/// Using smaller integer types such as `rand::random::<u64>()` would produce insufficient
/// entropy and may be rejected by stricter Microsoft tenants.
///
/// # Returns
///
/// A 86-character base64url string (characters `A–Z`, `a–z`, `0–9`, `-`, `_`).
#[must_use]
pub fn generate_code_verifier() -> String {
    let mut bytes: [u8; 64] = [0; 64];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Derives the PKCE code challenge from a code verifier.
///
/// The challenge is computed as `BASE64URL(SHA-256(verifier))` as specified by
/// RFC 7636 §4.2. Microsoft requires `code_challenge_method=S256`; the plain method
/// is not accepted.
///
/// This function is pure — the same verifier always produces the same challenge.
/// The verifier itself is never sent over the network; only the challenge is included
/// in the authorisation URL.
///
/// # Arguments
///
/// - `verifier`: The PKCE code verifier string, typically produced by
///   [`generate_code_verifier`].
///
/// # Returns
///
/// A base64url-encoded string that is the SHA-256 hash of the UTF-8 representation of
/// `verifier`.
#[must_use]
pub fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}

/// Generates a random OAuth `state` parameter for CSRF protection.
///
/// The state value is derived from 16 cryptographically random bytes produced by
/// [`rand::thread_rng`], hex-encoded to a 32-character lowercase string. This provides
/// 128 bits of entropy, which is sufficient to make state guessing attacks infeasible.
///
/// The caller must store the returned state and verify that the value returned in the
/// OAuth callback URL matches exactly. If they differ, the callback must be rejected as
/// a potential CSRF attack.
///
/// # Returns
///
/// A 32-character lowercase hexadecimal string.
#[must_use]
pub fn generate_state() -> String {
    let mut bytes: [u8; 16] = [0; 16];
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
