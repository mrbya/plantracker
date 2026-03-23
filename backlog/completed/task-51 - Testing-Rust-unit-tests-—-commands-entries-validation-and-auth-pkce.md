---
id: TASK-51
title: 'Testing: Rust unit tests — commands::entries validation and auth::pkce'
status: Done
assignee: []
created_date: '2026-03-23 07:52'
updated_date: '2026-03-23 10:50'
labels:
  - testing
  - backend
dependencies:
  - TASK-48
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Pure-function unit tests requiring no DB or Tauri state. Covers sections 10.5 and 10.6 of Phase 10.

**10.5 — `commands::entries` — `parse_and_validate_times`**

Add inline `#[cfg(test)]` module to `src-tauri/src/commands/entries.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::parse_and_validate_times;

    #[test]
    fn rejects_end_before_start() {
        let result = parse_and_validate_times(
            "2024-03-15T11:00:00Z",
            "2024-03-15T10:00:00Z",
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("after start_time"));
    }

    #[test]
    fn rejects_equal_times() {
        let result = parse_and_validate_times(
            "2024-03-15T10:00:00Z",
            "2024-03-15T10:00:00Z",
        );
        assert!(result.is_err());
    }

    #[test]
    fn accepts_valid_range() {
        let result = parse_and_validate_times(
            "2024-03-15T10:00:00Z",
            "2024-03-15T11:00:00Z",
        );
        assert!(result.is_ok());
        let (start, end) = result.unwrap();
        assert!(end > start);
    }

    #[test]
    fn rejects_unparseable_start() {
        let result = parse_and_validate_times("not-a-date", "2024-03-15T11:00:00Z");
        assert!(result.is_err());
    }
}
```

**10.6 — `auth::pkce`**

Add inline `#[cfg(test)]` module to `src-tauri/src/auth/pkce.rs`:

| Test | What it asserts |
|---|---|
| `verifier_length_within_rfc7636_range` | `generate_code_verifier()` length is 43–128 chars |
| `verifier_is_base64url` | All chars are `[A-Za-z0-9\-_]` — no padding or `+/` |
| `challenge_is_sha256_of_verifier` | `generate_code_challenge(v)` equals `BASE64URL(SHA256(v))` |
| `state_is_32_hex_chars` | `generate_state()` is exactly 32 lowercase hex chars |
| `successive_verifiers_differ` | Two consecutive calls return different values (RNG is live) |
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 4 parse_and_validate_times tests implemented in commands/entries.rs
- [x] #2 5 PKCE tests implemented in auth/pkce.rs
- [x] #3 All tests are synchronous (#[test], not #[tokio::test]) — no async or DB needed
- [x] #4 SQLX_OFFLINE=true cargo test passes with all tests green
- [x] #5 cargo clippy passes with no warnings
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added 4 synchronous tests for `parse_and_validate_times` in `commands/entries.rs` (rejects_end_before_start, rejects_equal_times, accepts_valid_range, rejects_unparseable_start) and 5 synchronous tests for PKCE helpers in `auth/pkce.rs` (verifier_length_within_rfc7636_range, verifier_is_base64url, challenge_is_sha256_of_verifier, state_is_32_hex_chars, successive_verifiers_differ). All 30 tests pass under SQLX_OFFLINE=true; clippy is clean.
<!-- SECTION:FINAL_SUMMARY:END -->
