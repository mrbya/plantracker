---
id: TASK-44
title: 'Reports: Make "All plans" and "All tasks" re-selectable in plan/task filters'
status: To Do
assignee: []
created_date: '2026-03-23 01:55'
labels:
  - frontend
  - reports
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
In the Reports view, after selecting a specific plan or task the user cannot return to the "All plans" / "All tasks" state — those options disappear from the dropdown. This is because the `Select` component renders its placeholder as `<option value="" disabled hidden>`, which is hidden once a real value is selected.

**Root cause**

`src/lib/components/ui/Select.svelte` line 19:
```svelte
<option value="" disabled hidden>{placeholder}</option>
```
The `disabled hidden` attributes prevent re-selection. This is intentional in other views (TimeTracking, ManualEntry) where a specific task must always be chosen, so **the `Select` component itself must not be changed**.

**Fix — local to `Reports.svelte` only**

Instead of relying on the placeholder prop, prepend selectable "All …" options directly into the options arrays:

- Plan select options: `[{ value: "", label: "All plans" }, ...planOptions]` — remove the `placeholder` prop so there is no disabled hidden option competing with the selectable one.
- Task select options (when a plan is selected): `[{ value: "", label: "All tasks" }, ...taskOptions]`
- Task select (when no plan is selected): keep showing only the placeholder "Select a plan first" (the existing disabled hidden option), since listing tasks without a plan isn't meaningful.

`onPlanChange` already resets `selectedTaskId` to `""` when the plan changes — this is correct and should remain.

No changes to `Select.svelte`, `TimeTracking.svelte`, `ManualEntry.svelte`, or the Rust backend.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 After selecting a plan, the plan dropdown still shows 'All plans' as a selectable option and choosing it resets selectedPlanId to '' and clears the task selection.
- [ ] #2 After selecting a task, the task dropdown still shows 'All tasks' as a selectable option and choosing it resets selectedTaskId to ''.
- [ ] #3 When no plan is selected, the task dropdown shows the non-selectable 'Select a plan first' hint (unchanged behaviour).
- [ ] #4 TimeTracking.svelte and ManualEntry.svelte are unmodified.
- [ ] #5 Select.svelte is unmodified.
- [ ] #6 The app builds without errors (cargo + svelte-check).
<!-- AC:END -->
