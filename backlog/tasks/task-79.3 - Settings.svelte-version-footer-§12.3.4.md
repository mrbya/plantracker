---
id: TASK-79.3
title: Settings.svelte version footer (§12.3.4)
status: Done
assignee: []
created_date: '2026-03-26 12:55'
updated_date: '2026-03-26 13:23'
labels:
  - frontend
  - phase-12
dependencies:
  - TASK-79.2
references:
  - docs/phases/phase-12-quality-of-life.md
parent_task_id: TASK-79
priority: medium
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Load the app version in `Settings.svelte` and render it as a subtle footer below the Account section.

## Script changes

```typescript
import { getAppVersion } from '$lib/api';

let appVersion = $state<string | null>(null);

// Inside onMount, alongside the existing getDataDir() call:
onMount(async () => {
    try { dataDir = await getDataDir(); } catch (e) { addError('Could not get data directory: ' + String(e)); }
    try { appVersion = await getAppVersion(); } catch { /* non-critical — footer simply won't render */ }
});
```

## Template

Append after the closing `</section>` of the Account section, inside the `.view` wrapper:

```svelte
{#if appVersion}
    <footer class="version-footer">
        v{appVersion} · PlanTracker
    </footer>
{/if}
```

## CSS

```css
.version-footer {
    margin-top: 1.5rem;
    padding-top: 1rem;
    border-top: 1px solid var(--border);
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    text-align: center;
    user-select: text;
}
```

`user-select: text` allows the version string to be copied for bug reports.

The `{#if appVersion}` guard prevents rendering if the command fails.

## i18n note

`v{appVersion} · PlanTracker` is an identifier, not a translatable sentence. No message key needed.

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.3.4
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 appVersion state variable is initialised to null and populated in onMount
- [x] #2 Footer renders as 'v{version} · PlanTracker' when appVersion is non-null
- [x] #3 Footer is absent from the DOM when appVersion is null (guard confirmed)
- [x] #4 Version string is selectable with the cursor (user-select: text)
- [x] #5 Footer uses --text-muted colour and --font-size-sm
- [x] #6 Footer has a top border using --border
- [x] #7 Renders correctly in both dark (Mocha) and light (Latte) themes
- [x] #8 Storybook Settings/Default story shows v0.0.0-storybook in the footer
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added `appVersion = $state<string | null>(null)` to Settings.svelte. Imported `getAppVersion` from `$lib/api` and called it in `onMount` after `getDataDir`, silently swallowing errors so the footer simply does not render on failure. Added `{#if appVersion}` footer template rendering `v{appVersion} · PlanTracker` after the Account section. Added `.version-footer` CSS with `--text-muted`, `--font-size-sm`, `--border` top border, and `user-select: text`. `pnpm tsc --noEmit` and `pnpm svelte-check` both pass clean.
<!-- SECTION:FINAL_SUMMARY:END -->
