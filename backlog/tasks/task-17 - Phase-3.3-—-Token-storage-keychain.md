---
id: TASK-17
title: Phase 3.3 — Token storage (keychain)
status: Done
assignee: []
created_date: '2026-03-19 11:04'
updated_date: '2026-03-19 11:15'
labels:
  - backend
  - auth
  - phase-3
dependencies:
  - TASK-16
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/auth/keychain.rs` to persist tokens in the OS keychain using the `keyring` crate. Service name: `"PlanTracker"`, account key: `"token_set"`.

```rust
use keyring::Entry;
use crate::auth::oauth::TokenSet;

pub fn save_tokens(tokens: &TokenSet) -> anyhow::Result<()> {
    let entry = Entry::new("PlanTracker", "token_set")?;
    entry.set_password(&serde_json::to_string(tokens)?)?;
    Ok(())
}

pub fn load_tokens() -> anyhow::Result<Option<TokenSet>> {
    let entry = Entry::new("PlanTracker", "token_set")?;
    match entry.get_password() {
        Ok(json) => Ok(Some(serde_json::from_str(&json)?)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn clear_tokens() -> anyhow::Result<()> {
    let entry = Entry::new("PlanTracker", "token_set")?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // already gone
        Err(e) => Err(e.into()),
    }
}
```

**Never store tokens in:**
- `tauri-plugin-store` (plaintext JSON file on disk)
- Environment variables
- Any file in the data directory
- In-memory state that persists across app exits

Expose from `auth/mod.rs`: `pub mod keychain;`
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 save_tokens serializes TokenSet to JSON and stores it in the OS keychain under service 'PlanTracker'
- [x] #2 load_tokens returns None (not an error) when no entry exists in the keychain
- [x] #3 clear_tokens returns Ok(()) if no entry exists (idempotent)
- [x] #4 Tokens are never written to disk files or environment variables
- [x] #5 cargo clippy -- -D warnings passes
<!-- AC:END -->
