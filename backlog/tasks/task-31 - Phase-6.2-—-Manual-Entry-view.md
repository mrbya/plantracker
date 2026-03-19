---
id: TASK-31
title: Phase 6.2 — Manual Entry view
status: Done
assignee: []
created_date: '2026-03-19 13:44'
updated_date: '2026-03-19 13:50'
labels:
  - frontend
  - typescript
  - svelte
milestone: Phase 6
dependencies:
  - TASK-30
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement `src/views/ManualEntry.svelte` — a form for creating and editing time entries, plus a recent entries table shared with the Time Tracking view. Also complete the delete stub left in `TimeTracking.svelte`.

## API wrappers (add to `src/lib/api/index.ts`)

```typescript
export async function createManualEntry(params: {
  taskId: string;
  startTime: string;
  endTime: string;
  notes?: string;
}): Promise<TimeEntry>

export async function updateEntry(params: {
  id: string;
  startTime: string;
  endTime: string;
  notes?: string;
}): Promise<TimeEntry>

export async function deleteEntry(id: string): Promise<void>
```

## ManualEntry.svelte layout

**Top section — form**
- Plan `<Select>` → Task `<Select>` (same plan/task store logic as TimeTracking)
- Start `<Input type="datetime-local">` bound to `startTime`
- End `<Input type="datetime-local">` bound to `endTime` — must be after start
- Notes `<textarea>` (optional, styled like `<Input>`)
- Submit `<Button variant="primary">` — label "Save Entry" when creating, "Update Entry" when editing
- Inline validation messages for: missing task, end before start, unparseable dates
- Clear/Cancel button when in edit mode — resets form to create mode

**Bottom section — recent entries table**
- Same columns as TimeTracking: Task | Plan | Start | End | Duration
- Edit button per row: populate form fields with that entry's data, switch to edit mode
- Delete button per row: same inline "Sure? / Cancel" confirm pattern as TimeTracking
- `<EmptyState>` when no entries

## Complete the delete stub in TimeTracking.svelte

`TimeTracking.svelte` currently has:
```typescript
// TODO: call deleteEntry(id) once Phase 6 (TASK-31) is implemented.
confirmDeleteId = null;
addError('Delete is not yet available (Phase 6).');
```

Replace the stub with the real `deleteEntry` call and reload entries afterward.

## Form pre-fill for edit

When the user clicks Edit on an entry row, populate the form:
- Convert the ISO `startTime`/`endTime` to `datetime-local` format (`YYYY-MM-DDTHH:MM`)
- Set `editingId` to the entry's id
- On submit: call `updateEntry` instead of `createManualEntry`
- On success: reset `editingId` to null, clear form, reload entries

## State handling
- Disable the submit button while the request is in flight (show Spinner)
- Show a success toast on create/update/delete
- Show an error toast on failure
- Reload entries after any mutation
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Form submits valid data and creates a new entry
- [x] #2 Inline validation prevents: missing task, end before or equal to start
- [x] #3 Edit button pre-fills the form; submit calls updateEntry
- [x] #4 Cancel/Clear button in edit mode resets the form
- [x] #5 Delete button shows inline confirmation and calls deleteEntry on confirm
- [x] #6 Delete stub in TimeTracking.svelte replaced with real deleteEntry call
- [x] #7 Success toast shown after create, update, and delete
- [x] #8 Entries list refreshes after every mutation
- [x] #9 EmptyState shown when no entries exist
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented `ManualEntry.svelte` with plan/task dropdowns, datetime-local inputs for start/end, optional notes textarea, inline validation (missing task, end ≤ start), create/edit/delete flows with success/error toasts, and editing-row highlight. `isoToLocal`/`localToIso` helpers handle datetime format conversion. Added `createManualEntry`, `updateEntry`, `deleteEntry` wrappers to `api/index.ts`. Replaced the delete stub in `TimeTracking.svelte` with the real `deleteEntry` call + reload. TypeScript passes with no errors.
<!-- SECTION:FINAL_SUMMARY:END -->
