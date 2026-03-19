---
id: TASK-21
title: Phase 3.7 — Login screen and app entry point
status: Done
assignee: []
created_date: '2026-03-19 11:05'
updated_date: '2026-03-19 11:21'
labels:
  - frontend
  - auth
  - phase-3
dependencies:
  - TASK-20
priority: high
ordinal: 7000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/views/Login.svelte` and wire up the app entry point so auth state gates access to the main layout.

**`src/views/Login.svelte`:**
- Centered layout on `--bg` background
- App title ("PlanTracker") and a short tagline
- "Sign in with Microsoft" primary `<Button>` — calls `login()` from the auth store
- Disabled and shows a `<Spinner>` while login is in progress
- Error is surfaced via the `notifications` store (toast) — no inline error display needed
- No hardcoded colors — Catppuccin CSS variables only

**`src/routes/+page.svelte`** — update to call `initAuth()` on mount and gate layout rendering:
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { isAuthenticated, initAuth } from '$lib/stores/auth';
  import Layout from '$lib/components/Layout.svelte';
  import Login from '../views/Login.svelte';
  import ToastContainer from '$lib/components/ToastContainer.svelte';

  onMount(() => initAuth());
</script>

<ToastContainer />
{#if $isAuthenticated}
  <Layout />
{:else}
  <Login />
{/if}
```

**`src/lib/components/ToastContainer.svelte`** — renders toast notifications:
- Fixed position, bottom-right corner
- Reads from `notifications` store
- Each toast: color-coded dot + message, auto-dismissed after 4 s (handled by the store)
- Colors: `--success` for success, `--danger` for error, `--warning` for warning

After implementing, run `pnpm tsc --noEmit` to confirm no TypeScript errors.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Clicking 'Sign in with Microsoft' opens the system browser to Microsoft login
- [x] #2 Button is disabled and shows a spinner while login is pending
- [x] #3 After successful login, the app transitions to the main Layout without a page reload
- [x] #4 Refreshing the app re-uses stored tokens and shows Layout directly (no re-login)
- [x] #5 'Sign out' in the sidebar clears tokens and returns to Login screen
- [x] #6 ToastContainer renders at bottom-right and dismisses automatically after 4 seconds
- [x] #7 pnpm tsc --noEmit passes with no type errors
<!-- AC:END -->
