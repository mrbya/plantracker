use keyring::Entry;

use crate::auth::oauth::TokenSet;

/// Keychain service name used for all `PlanTracker` credential entries.
const SERVICE: &str = "PlanTracker";
/// Keychain account name under which the serialised `TokenSet` is stored.
const ACCOUNT: &str = "token_set";

/// Persists tokens in the OS keychain (secret-service on Linux, Keychain on macOS, Credential Manager on Windows).
///
/// # Errors
/// Returns an error if the keychain entry cannot be created or the password cannot be set.
pub fn save_tokens(tokens: &TokenSet) -> anyhow::Result<()> {
    let entry = Entry::new(SERVICE, ACCOUNT)?;
    entry.set_password(&serde_json::to_string(tokens)?)?;
    Ok(())
}

/// Loads tokens from the OS keychain. Returns `None` if no entry exists.
///
/// # Errors
/// Returns an error if the keychain entry cannot be read or the JSON is malformed.
pub fn load_tokens() -> anyhow::Result<Option<TokenSet>> {
    let entry = Entry::new(SERVICE, ACCOUNT)?;
    match entry.get_password() {
        Ok(json) => Ok(Some(serde_json::from_str(&json)?)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Removes tokens from the OS keychain. Idempotent — returns `Ok(())` if no entry exists.
///
/// # Errors
/// Returns an error if the keychain entry exists but cannot be deleted.
pub fn clear_tokens() -> anyhow::Result<()> {
    let entry = Entry::new(SERVICE, ACCOUNT)?;
    match entry.delete_password() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}
