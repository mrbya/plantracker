import { derived, writable } from 'svelte/store';

import { getAuthStatus, login as apiLogin, logout as apiLogout } from '$lib/api';
import { addError } from '$lib/stores/notifications';
import { syncAndLoad } from '$lib/stores/planner';
import type { AuthStatus } from '$lib/types';

export const authStatus = writable<AuthStatus | null>(null);

export const isAuthenticated = derived(authStatus, (s) => s?.isAuthenticated ?? false);
export const userDisplayName = derived(authStatus, (s) => s?.userDisplayName ?? null);

/** Fetches auth state from the backend and hydrates the store. Call once on app startup. */
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

export async function login(): Promise<void> {
  try {
    const status = await apiLogin();
    authStatus.set(status);
    if (status.isAuthenticated) {
      syncAndLoad();
    }
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
