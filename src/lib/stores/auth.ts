import { writable, derived } from 'svelte/store';
import type { AuthStatus } from '$lib/types';

export const authStatus = writable<AuthStatus | null>(null);

export const isAuthenticated = derived(authStatus, (s) => s?.isAuthenticated ?? false);

export const userDisplayName = derived(authStatus, (s) => s?.userDisplayName ?? null);

// Stubs — implemented in Phase 3
export async function login(): Promise<void> {}
export async function logout(): Promise<void> {}
