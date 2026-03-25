/**
 * Authentication store.
 *
 * Holds the single source of truth for whether the user is signed in and who
 * they are.  The private `authStatus` writable is kept internal; consumers
 * read the two derived exports (`isAuthenticated`, `userDisplayName`) so they
 * only re-render when the specific slice they care about changes.
 *
 * Startup sequence:
 *   1. `+page.svelte` calls `initAuth()` inside `onMount`.
 *   2. `initAuth` asks the backend whether a valid token already exists in the
 *      OS keychain.
 *   3. If authenticated, `syncAndLoad()` is triggered immediately so the
 *      planner store is populated before the user sees any view.
 */
import { derived, writable } from "svelte/store";

import {
  getAuthStatus,
  login as apiLogin,
  logout as apiLogout,
} from "$lib/api";
import { addError } from "$lib/stores/notifications";
import { syncAndLoad } from "$lib/stores/planner";
import type { AuthStatus } from "$lib/types";

/** Internal store — not exported; use the derived stores below. */
const authStatus = writable<AuthStatus | null>(null);

/**
 * `true` while a valid (or silently refreshable) token exists in the keychain.
 * Drives the conditional rendering in `+page.svelte` that shows either the
 * main `Layout` or the `Login` view.
 */
export const isAuthenticated = derived(
  authStatus,
  (s) => s?.isAuthenticated ?? false,
);

/**
 * The Microsoft Graph display name of the signed-in user, or `null` when not
 * authenticated or before the first sync has populated the field.  Displayed
 * in the sidebar avatar tooltip and the Settings view.
 */
export const userDisplayName = derived(
  authStatus,
  (s) => s?.userDisplayName ?? null,
);

/**
 * Fetches auth state from the backend and hydrates the store.
 *
 * Must be called exactly once, during app startup (inside `onMount` in
 * `+page.svelte`).  If the backend reports an authenticated state it also
 * fires `syncAndLoad()` so that plans and tasks are ready without requiring
 * a manual sync.
 *
 * Errors reading keychain state are silently swallowed (store resets to
 * `null`) because the user can always sign in again.
 */
export async function initAuth(): Promise<void> {
  try {
    const status = await getAuthStatus();
    authStatus.set(status);
    if (status.isAuthenticated) {
      syncAndLoad();
    }
  } catch {
    authStatus.set(null);
  }
}

/**
 * Opens the Microsoft OAuth 2.0 PKCE login flow in the system browser.
 *
 * On success the store is updated with the new `AuthStatus` (including the
 * user's display name) and `syncAndLoad()` is triggered.  On failure an error
 * toast is shown and the store is left unchanged.
 */
export async function login(): Promise<void> {
  try {
    const status = await apiLogin();
    authStatus.set(status);
    if (status.isAuthenticated) {
      syncAndLoad();
    }
  } catch (e) {
    addError("Sign in failed: " + String(e));
  }
}

/**
 * Signs the current user out and resets the store to an unauthenticated state.
 *
 * Calls the backend to clear tokens from the OS keychain, then overwrites the
 * store so the app immediately navigates to the `Login` view.  On failure an
 * error toast is shown.
 */
export async function logout(): Promise<void> {
  try {
    await apiLogout();
    authStatus.set({ isAuthenticated: false, userDisplayName: null });
  } catch (e) {
    addError("Sign out failed: " + String(e));
  }
}
