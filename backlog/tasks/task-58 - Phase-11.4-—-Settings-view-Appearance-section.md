---
id: TASK-58
title: 'Phase 11.4 — Settings view: Appearance section'
status: Done
assignee: []
created_date: '2026-03-23 18:47'
updated_date: '2026-03-23 18:50'
labels:
  - frontend
  - theme
  - phase-11
dependencies:
  - TASK-57
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add an Appearance section to `src/views/Settings.svelte` with a Theme dropdown that lets the user choose Dark, Light, or System Default.

## Context

`Settings.svelte` already has sections following the `settings-section` / `setting-row` / `setting-info` / `setting-control` layout. The new Appearance section must use the same layout so it is visually consistent. The `Select` component is already imported in the view.

The theme store (`src/lib/stores/theme.ts`) is created in Phase 11.3 and must be available before this task is worked on.

## Changes to `src/views/Settings.svelte`

**Add to the script imports:**

```typescript
import {
    themeChoice,
    saveTheme,
    type ThemeChoice,
} from '$lib/stores/theme';
```

**Add a new section to the template** (insert before the existing Account section):

```svelte
<!-- Appearance -->
<section class="settings-section">
  <h2 class="section-title">Appearance</h2>
  <div class="setting-row">
    <div class="setting-info">
      <span class="setting-label">Theme</span>
      <span class="setting-desc">
        Controls the colour scheme of the application.
      </span>
    </div>
    <div class="setting-control">
      <Select
        options={[
          { value: 'dark',   label: 'Dark'           },
          { value: 'light',  label: 'Light'          },
          { value: 'system', label: 'System Default' },
        ]}
        value={$themeChoice}
        onchange={(e) =>
          saveTheme((e.target as HTMLSelectElement).value as ThemeChoice)
        }
      />
    </div>
  </div>
</section>
```

No new components are needed. No Rust changes required.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 An Appearance section with a Theme `<Select>` is visible in the Settings view
- [x] #2 The dropdown offers three options: Dark, Light, System Default
- [x] #3 Selecting an option calls `saveTheme()` and the theme changes immediately without a page reload
- [x] #4 The dropdown displays the currently active choice when the Settings view is opened
- [x] #5 The new section uses the same `setting-row` / `setting-info` / `setting-control` CSS structure as existing sections
- [x] #6 `pnpm tsc --noEmit` passes with no type errors
<!-- AC:END -->
