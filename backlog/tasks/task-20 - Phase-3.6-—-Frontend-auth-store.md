---
id: TASK-20
title: Phase 3.6 — Frontend auth store
status: Done
assignee: []
created_date: '2026-03-19 11:05'
updated_date: '2026-03-19 11:20'
labels:
  - frontend
  - auth
  - phase-3
dependencies:
  - TASK-19
priority: high
ordinal: 6000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/lib/stores/auth.ts` with reactive auth state and actions. Stores are the only layer that calls `src/lib/api/index.ts` — views bind to stores only.

```typescript
import { writable, derived, get } from 'svelte/store';
import { getAuthStatus, login as apiLogin, logout as apiLogout } from '$lib/api';
import { addError } from '$lib/stores/notifications';
import type { AuthStatus } from '$lib/types';

export const authStatus = writable<AuthStatus | null>(null);

export const isAuthenticated = derived(authStatus, s => s?.isAuthenticated ?? false);
export const userDisplayName = derived(authStatus, s => s?.userDisplayName ?? null);

/** Call once on app startup to hydrate auth state from the backend. */
export async function initAuth(): Promise<void> {
    try {
        const status = await getAuthStatus();
        authStatus.set(status);
    } catch (e) {
        authStatus.set(null);
    }
}

export async function login(): Promise<void> {
    try {
        const status = await apiLogin();
        authStatus.set(status);
    } catch (e) {
        addError('Sign in failed: ' + String(e));
    }
}

export async function logout(): Promise<void> {
    try {
        await apiLogout();
        authStatus.set({ isAuthenticated: false, userDisplayName: null });
    } catch (e) {
        addError('Sign out failed: ' + String(e));
    }
}
```

Also ensure `src/lib/types.ts` contains:
```typescript
export interface AuthStatus {
    isAuthenticated: boolean;
    userDisplayName: string | null;
}
```

And create `src/lib/stores/notifications.ts` if not already present:
```typescript
import { writable } from 'svelte/store';

export type NotificationType = 'success' | 'error' | 'warning';
export interface Notification { id: number; type: NotificationType; message: string; }

export const notifications = writable<Notification[]>([]);

let nextId = 0;
function add(type: NotificationType, message: string) {
    const id = ++nextId;
    notifications.update(n => [...n, { id, type, message }]);
    setTimeout(() => notifications.update(n => n.filter(x => x.id !== id)), 4000);
}
export const addSuccess = (msg: string) => add('success', msg);
export const addError   = (msg: string) => add('error', msg);
export const addWarning = (msg: string) => add('warning', msg);
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 authStatus, isAuthenticated, userDisplayName stores are exported from auth.ts
- [x] #2 initAuth() fetches status from backend and hydrates the store without throwing
- [x] #3 login() calls apiLogin(), updates authStatus, and shows an error toast on failure
- [x] #4 logout() calls apiLogout(), resets authStatus to unauthenticated state
- [x] #5 notifications.ts exports addSuccess, addError, addWarning with 4-second auto-dismiss
- [x] #6 AuthStatus interface in types.ts uses camelCase matching the Rust serde output
- [x] #7 No invoke() calls anywhere outside src/lib/api/index.ts
<!-- AC:END -->
