/**
 * Single invoke boundary — all Tauri command calls live here.
 * Views and stores must import from this file, never call invoke() directly.
 */
import { invoke } from '@tauri-apps/api/core';

import type { AuthStatus } from '$lib/types';

// ---------------------------------------------------------------------------
// Auth
// ---------------------------------------------------------------------------

export async function login(): Promise<AuthStatus> {
  return invoke<AuthStatus>('login');
}

export async function logout(): Promise<void> {
  return invoke('logout');
}

export async function getAuthStatus(): Promise<AuthStatus> {
  return invoke<AuthStatus>('get_auth_status');
}
