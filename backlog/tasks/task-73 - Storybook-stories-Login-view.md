---
id: TASK-73
title: 'Storybook stories: Login view'
status: To Do
assignee: []
created_date: '2026-03-25 14:50'
labels:
  - storybook
  - docs
dependencies:
  - TASK-62
priority: medium
ordinal: 11100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Create `src/stories/views/Login.stories.svelte` covering `src/views/Login.svelte`.

`Login` is the simplest view to story — it has no store dependencies beyond `auth` and its only interactive element is the sign-in button.

## Approach
- Seed the `auth` store so `isAuthenticated` is `false` and no loading state is active.
- Mock `invoke('login')` to be a no-op (or a slow promise to simulate the loading state).

## Stories to write

| Story name | State |
|---|---|
| `Idle` | Default — sign-in button enabled, no spinner |
| `SigningIn` | `loading` state active — button disabled with spinner |

## Notes
- Story title: `PlanTracker/Views/Login`.
- Depends on TASK-62.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Idle story shows enabled sign-in button
- [ ] #2 SigningIn story shows disabled button with Spinner
- [ ] #3 No invoke() errors in console
<!-- AC:END -->
