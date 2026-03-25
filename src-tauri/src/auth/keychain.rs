use keyring::Entry;

use crate::auth::oauth::TokenSet;

/// Keychain service name used for all `PlanTracker` credential entries.
///
/// This string appears as the "service" or "application" label in the platform's
/// credential manager UI (Secret Service/libsecret on Linux, Keychain Access on macOS,
/// Windows Credential Manager on Windows).
const SERVICE: &str = "PlanTracker";

/// Keychain account name under which the serialised [`TokenSet`] is stored.
///
/// A single account name is sufficient because `PlanTracker` supports only one
/// signed-in user at a time. The stored credential is a JSON-serialised [`TokenSet`].
const ACCOUNT: &str = "token_set";

/// Persists a [`TokenSet`] to the operating-system keychain.
///
/// The token set is serialised to JSON and stored under the service name
/// `"PlanTracker"` and account name `"token_set"`. Platform backends:
/// - **Linux**: Secret Service via `libsecret` (e.g. GNOME Keyring, `KWallet`).
/// - **macOS**: macOS Keychain.
/// - **Windows**: Windows Credential Manager.
///
/// This function is called after every successful login and after every silent token
/// refresh so that the persisted tokens are always up to date. Tokens are never written
/// to any file, environment variable, or `SQLite` database.
///
/// # Arguments
///
/// - `tokens`: Reference to the [`TokenSet`] to persist.
///
/// # Errors
///
/// Returns an error if:
/// - the keychain entry cannot be created (e.g. the keyring service is unavailable), or
/// - [`keyring::Entry::set_password`] fails (e.g. permission denied).
pub fn save_tokens(tokens: &TokenSet) -> anyhow::Result<()> {
    let entry = Entry::new(SERVICE, ACCOUNT)?;
    entry.set_password(&serde_json::to_string(tokens)?)?;
    Ok(())
}

/// Loads a [`TokenSet`] from the operating-system keychain.
///
/// Looks up the credential stored under service `"PlanTracker"` / account
/// `"token_set"`. This is called once at application startup by
/// [`super::manager::AuthManager::new`] to restore a previous session without
/// requiring the user to log in again.
///
/// # Returns
///
/// - `Ok(Some(tokens))` if a valid entry was found and deserialised successfully.
/// - `Ok(None)` if no entry exists in the keychain for this service/account pair
///   (i.e. the user has never logged in or has previously logged out).
///
/// # Errors
///
/// Returns an error if:
/// - the keychain entry exists but cannot be read (e.g. access denied), or
/// - the stored JSON is malformed and cannot be deserialised as a [`TokenSet`].
pub fn load_tokens() -> anyhow::Result<Option<TokenSet>> {
    let entry = Entry::new(SERVICE, ACCOUNT)?;
    match entry.get_password() {
        Ok(json) => Ok(Some(serde_json::from_str(&json)?)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

/// Removes the stored [`TokenSet`] from the operating-system keychain.
///
/// This is called during logout to ensure that no credentials remain in the keychain
/// after the user signs out. The function is **idempotent**: if no entry exists it
/// returns `Ok(())` without raising an error, so it is safe to call multiple times.
///
/// # Errors
///
/// Returns an error if:
/// - the keychain entry exists but cannot be deleted (e.g. permission denied).
pub fn clear_tokens() -> anyhow::Result<()> {
    let entry = Entry::new(SERVICE, ACCOUNT)?;
    match entry.delete_password() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}
