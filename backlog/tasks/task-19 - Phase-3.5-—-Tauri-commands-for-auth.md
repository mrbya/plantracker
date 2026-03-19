---
id: TASK-19
title: Phase 3.5 — Tauri commands for auth
status: Done
assignee: []
created_date: '2026-03-19 11:05'
updated_date: '2026-03-19 11:18'
labels:
  - backend
  - frontend
  - auth
  - phase-3
dependencies:
  - TASK-18
priority: high
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src-tauri/src/commands/auth.rs` with the three auth commands. Create `src-tauri/src/commands/mod.rs` to expose the module. Register all commands in `lib.rs`.

```rust
use tauri::State;
use std::sync::Arc;
use crate::auth::manager::AuthManager;
use crate::auth::oauth::start_login;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthStatus {
    pub is_authenticated: bool,
    pub user_display_name: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub display_name: String,
    pub user_principal_name: String,
}

#[tauri::command]
pub async fn login(
    app: tauri::AppHandle,
    auth: State<'_, Arc<AuthManager>>,
) -> Result<AuthStatus, String> {
    let client_id = env!("VITE_AZURE_CLIENT_ID");
    let tenant_id = env!("VITE_AZURE_TENANT_ID");
    let tokens = start_login(&app, client_id, tenant_id).await.map_err(|e| e.to_string())?;
    auth.set_tokens(tokens).await.map_err(|e| e.to_string())?;
    Ok(AuthStatus { is_authenticated: true, user_display_name: None })
}

#[tauri::command]
pub async fn logout(auth: State<'_, Arc<AuthManager>>) -> Result<(), String> {
    auth.clear().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_auth_status(auth: State<'_, Arc<AuthManager>>) -> Result<AuthStatus, String> {
    Ok(AuthStatus {
        is_authenticated: auth.is_authenticated().await,
        user_display_name: auth.user_display_name().await,
    })
}
```

Register in `lib.rs`:
```rust
pub mod commands;
// ...
.invoke_handler(tauri::generate_handler![
    commands::auth::login,
    commands::auth::logout,
    commands::auth::get_auth_status,
])
```

Also add `src/lib/api/index.ts` (if not already present) with typed invoke wrappers:
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { AuthStatus } from '$lib/types';

export async function login(): Promise<AuthStatus> {
    return invoke<AuthStatus>('login');
}
export async function logout(): Promise<void> {
    return invoke('logout');
}
export async function getAuthStatus(): Promise<AuthStatus> {
    return invoke<AuthStatus>('get_auth_status');
}
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 login, logout, get_auth_status commands are registered in tauri::generate_handler!
- [x] #2 All three commands return errors as String (not panic)
- [x] #3 AuthStatus and UserInfo structs use serde(rename_all = camelCase)
- [x] #4 src/lib/api/index.ts exports typed wrappers for all three commands
- [x] #5 invoke() is called only inside src/lib/api/index.ts — not in views or stores directly
- [x] #6 cargo clippy -- -D warnings passes
<!-- AC:END -->
