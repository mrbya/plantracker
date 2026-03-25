/// OS keychain access for persisting [`oauth::TokenSet`] values.
///
/// Provides [`keychain::save_tokens`], [`keychain::load_tokens`], and
/// [`keychain::clear_tokens`]. Uses the `keyring` crate, which delegates to
/// `libsecret`/Secret Service on Linux, macOS Keychain on macOS, and Windows
/// Credential Manager on Windows.
pub mod keychain;

/// [`manager::AuthManager`] — single authority for token lifecycle and authentication state.
///
/// `AuthManager` is the only component that directly reads from or writes to the
/// keychain and that holds raw token strings. All other code obtains a valid access
/// token by calling [`manager::AuthManager::get_valid_token`].
pub mod manager;

/// OAuth 2.0 Authorization Code + PKCE flow and token structures.
///
/// Exposes [`oauth::start_login`] (the full browser-based login flow),
/// [`oauth::exchange_code_for_tokens`], [`oauth::refresh_access_token`], and the
/// [`oauth::TokenSet`] struct. The `Debug` implementation on `TokenSet` redacts
/// the raw token strings to prevent accidental logging.
pub mod oauth;

/// PKCE code verifier/challenge generation and CSRF state generation.
///
/// Exposes [`pkce::generate_code_verifier`] (64-byte CSPRNG → 86-char base64url),
/// [`pkce::generate_code_challenge`] (SHA-256 of verifier), and
/// [`pkce::generate_state`] (16-byte CSPRNG → 32-char hex) as specified by RFC 7636.
pub mod pkce;
