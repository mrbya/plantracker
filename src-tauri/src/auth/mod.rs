/// OS keychain access for persisting token sets.
pub mod keychain;
/// `AuthManager` — manages token lifecycle and authentication state.
pub mod manager;
/// OAuth 2.0 Authorization Code + PKCE flow and token management.
pub mod oauth;
/// PKCE verifier/challenge and CSRF state generation.
pub mod pkce;
