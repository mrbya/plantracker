---
id: TASK-61
title: >-
  Phase 12.1.2–12.1.4 — Wire SearchableSelect into TimeTracking, ManualEntry,
  and Reports
status: Done
assignee: []
created_date: '2026-03-23 20:00'
updated_date: '2026-03-23 20:08'
labels:
  - frontend
  - phase-12
dependencies:
  - TASK-60
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Swap the plan and task `<Select>` dropdowns for `<SearchableSelect>` in three views. **No logic changes** — only the import name and component name change. All props (`id`, `options`, `bind:value`, `placeholder`, `onchange`) remain identical.

`SearchableSelect` is created in TASK-60 and must be available before this task is worked on.

---

## Change pattern (same in all three views)

**Import — remove:**
```typescript
import Select from "$lib/components/ui/Select.svelte";
```

**Import — add:**
```typescript
import SearchableSelect from "$lib/components/ui/SearchableSelect.svelte";
```

In the template, rename `<Select` → `<SearchableSelect` and `</Select>` → `</SearchableSelect>` on the plan and task dropdowns only (the ones bound to `selectedPlanId` and `selectedTaskId`). Also add `disabled={!selectedPlanId}` to each task dropdown, since the `SearchableSelect` component supports a `disabled` prop and it prevents opening an empty panel when no plan is selected.

---

## 1. `src/views/TimeTracking.svelte`

Two dropdowns to replace (both identified by `id="plan-select"` and `id="task-select"`):

**Plan dropdown** — replace `<Select` with `<SearchableSelect`:
```svelte
<SearchableSelect
  id="plan-select"
  options={planOptions}
  bind:value={selectedPlanId}
  placeholder="Select a plan…"
  onchange={onPlanChange}
/>
```

**Task dropdown** — replace `<Select` with `<SearchableSelect`, add `disabled`:
```svelte
<SearchableSelect
  id="task-select"
  options={taskOptions}
  bind:value={selectedTaskId}
  placeholder={selectedPlanId ? undefined : "Select a plan first"}
  disabled={!selectedPlanId}
  onchange={onTaskChange}
/>
```

No other changes to `TimeTracking.svelte`.

---

## 2. `src/views/ManualEntry.svelte`

Same two dropdowns (`id="plan-select"` and `id="task-select"`):

**Plan dropdown:**
```svelte
<SearchableSelect
  id="plan-select"
  options={planOptions}
  bind:value={selectedPlanId}
  placeholder="Select a plan…"
  onchange={onPlanChange}
/>
```

**Task dropdown** (add `disabled`):
```svelte
<SearchableSelect
  id="task-select"
  options={taskOptions}
  bind:value={selectedTaskId}
  placeholder={selectedPlanId ? undefined : "Select a plan first"}
  disabled={!selectedPlanId}
  onchange={onTaskChange}
/>
```

No other changes to `ManualEntry.svelte`.

---

## 3. `src/views/Reports.svelte`

**Important:** `Reports.svelte` has **four** `<Select>` usages. Only two should be replaced — the plan and task filter dropdowns (`id="plan-select"` and `id="task-select"`). The two month-range dropdowns (bound to `MONTH_OPTIONS`) must remain as `<Select>`.

**Plan dropdown** — replace:
```svelte
<SearchableSelect
  id="plan-select"
  options={planOptions}
  bind:value={selectedPlanId}
  onchange={onPlanChange}
/>
```

**Task dropdown** — replace, add `disabled`:
```svelte
<SearchableSelect
  id="task-select"
  options={taskOptions}
  bind:value={selectedTaskId}
  placeholder={selectedPlanId ? undefined : "Select a plan first"}
  disabled={!selectedPlanId}
  onchange={onTaskChange}
/>
```

The two month `<Select>` components (using `MONTH_OPTIONS`) stay unchanged.

After adding the `SearchableSelect` import, if `Select` is still used (for the month dropdowns), keep both imports. Do not remove the `Select` import unless it is no longer referenced.

---

## What must NOT change

- `Settings.svelte` — untouched (sync frequency and theme dropdowns stay as `Select`)
- `Select.svelte` — untouched
- All event handlers (`onPlanChange`, `onTaskChange`), `$effect` blocks, store interactions, and other logic in all three views — no changes to anything except the two import lines and the four component tags
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 In `TimeTracking.svelte`: `Select` import replaced with `SearchableSelect`; plan and task dropdowns use `<SearchableSelect>`; task dropdown has `disabled={!selectedPlanId}`
- [x] #2 In `ManualEntry.svelte`: same changes as above
- [x] #3 In `Reports.svelte`: plan and task dropdowns replaced with `<SearchableSelect>` (with `disabled={!selectedPlanId}` on task); the two month-range `<Select>` components remain unchanged; `Select` import kept if still used
- [x] #4 `Settings.svelte` is not modified
- [x] #5 `Select.svelte` is not modified
- [x] #6 No changes to any event handler, `$effect`, store binding, or other logic in any of the three views
- [ ] #7 Typing a partial plan name in the plan dropdown of TimeTracking filters the list immediately
- [ ] #8 Typing a partial task name in the task dropdown filters within the current plan’s tasks
- [ ] #9 With no plan selected, the task dropdown is disabled (cannot be opened)
- [ ] #10 "No specific task" / "All tasks" entries remain visible even when a query is active
- [x] #11 `pnpm tsc --noEmit` passes; `pnpm svelte-check` passes
<!-- AC:END -->
