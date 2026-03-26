---
id: TASK-78.5
title: Add Language selector section to Settings view (§12.2.7)
status: Done
assignee: []
created_date: '2026-03-26 07:16'
updated_date: '2026-03-26 07:54'
labels:
  - frontend
  - i18n
  - phase-12
dependencies: []
parent_task_id: TASK-78
priority: medium
ordinal: 5000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a **Language** settings section to `src/views/Settings.svelte`, positioned after the Appearance section. Use the existing `setting-row` layout pattern.

```typescript
import { saveLocale, LOCALE_LABELS, type AppLocale } from '$lib/stores/locale';
import { getLocale } from '$lib/paraglide/runtime';

const localeOptions = Object.entries(LOCALE_LABELS).map(([value, label]) => ({
    value,
    label,
}));

// Reactive: re-reads on locale change so the dropdown reflects the active locale.
const currentLocale = $derived(getLocale() as AppLocale);
```

Template to add after the Appearance section:

```svelte
<section class="settings-section">
  <h2 class="section-title">{m.settings_section_language()}</h2>
  <div class="setting-row">
    <div class="setting-info">
      <span class="setting-label">{m.settings_language_label()}</span>
      <span class="setting-desc">{m.settings_language_desc()}</span>
    </div>
    <div class="setting-control">
      <Select
        options={localeOptions}
        value={currentLocale}
        onchange={(e) =>
          saveLocale((e.target as HTMLSelectElement).value as AppLocale)
        }
      />
    </div>
  </div>
</section>
```

Locale labels displayed in the dropdown use their **native name** (`LOCALE_LABELS`): English, Slovenčina, Deutsch.

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.2.7
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 A 'Language' section appears in Settings after the Appearance section
- [ ] #2 The Select dropdown shows English, Slovenčina, Deutsch as options
- [ ] #3 Selecting a different language immediately changes all visible strings (reactive via $derived)
- [ ] #4 The dropdown reflects the currently active locale on render
<!-- AC:END -->
