---
id: TASK-15
title: Phase 3.1 — PKCE helper functions
status: Done
assignee: []
created_date: '2026-03-19 11:03'
updated_date: '2026-03-19 11:09'
labels:
  - backend
  - auth
  - phase-3
dependencies: []
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/auth/pkce.rs` with the three cryptographic helpers needed for the OAuth 2.0 PKCE flow.

```rust
use rand::RngCore;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use sha2::{Digest, Sha256};

/// 64 random bytes → base64url string (86 chars, within RFC 7636's 43–128 range).
pub fn generate_code_verifier() -> String {
    let mut bytes = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// SHA-256 of the verifier, base64url-encoded.
pub fn generate_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hash)
}

/// 16 random bytes, hex-encoded — used as the OAuth `state` CSRF token.
pub fn generate_state() -> String {
    let mut bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}
```

**Do NOT use `rand::random::<u64>()`** — insufficient entropy rejected by some Microsoft tenants (see `pitfalls.md`).

Expose the module from `src-tauri/src/auth/mod.rs`:
```rust
pub mod pkce;
```

Declare `pub mod auth;` in `lib.rs`.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 generate_code_verifier() produces an 86-character base64url string
- [x] #2 generate_code_challenge() returns the SHA-256 / base64url of the verifier
- [x] #3 generate_state() returns a 32-character hex string (16 bytes)
- [x] #4 No rand::random::<u64>() or any fixed-size integer RNG source used
- [x] #5 cargo clippy -- -D warnings passes
<!-- AC:END -->
