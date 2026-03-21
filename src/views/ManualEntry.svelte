<script lang="ts">
  import { onMount } from "svelte";

  import {
    createManualEntry,
    deleteEntry,
    getRecentEntries,
    updateEntry,
  } from "$lib/api";
  import Button from "$lib/components/ui/Button.svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import Spinner from "$lib/components/ui/Spinner.svelte";
  import { addError, addSuccess } from "$lib/stores/notifications";
  import {
    plans,
    selectTask,
    selectedPlan,
    selectedTask,
    tasksByPlan,
  } from "$lib/stores/planner";
  import { entriesLimit } from "$lib/stores/settings";
  import type { TimeEntry } from "$lib/types";
  import { formatDuration } from "$lib/utils/duration";

  // ---------------------------------------------------------------------------
  // Month options (shared with Reports)
  // ---------------------------------------------------------------------------

  const MONTH_OPTIONS = [
    { value: "1", label: "January" },
    { value: "2", label: "February" },
    { value: "3", label: "March" },
    { value: "4", label: "April" },
    { value: "5", label: "May" },
    { value: "6", label: "June" },
    { value: "7", label: "July" },
    { value: "8", label: "August" },
    { value: "9", label: "September" },
    { value: "10", label: "October" },
    { value: "11", label: "November" },
    { value: "12", label: "December" },
  ];

  // ---------------------------------------------------------------------------
  // Dropdown state
  // ---------------------------------------------------------------------------

  let selectedPlanId = $state($selectedPlan?.id ?? "");
  let selectedTaskId = $state($selectedTask?.id ?? "");

  const planOptions = $derived(
    $plans.map((p) => ({ value: p.id, label: p.title })),
  );
  const taskOptions = $derived(
    selectedPlanId
      ? ($tasksByPlan[selectedPlanId] ?? []).map((t) => ({
          value: t.id,
          label: t.title,
        }))
      : [],
  );

  function onPlanChange() {
    selectedTaskId = "";
    selectedPlan.set($plans.find((p) => p.id === selectedPlanId) ?? null);
    selectedTask.set(null);
    loadEntries();
  }

  function onTaskChange() {
    const task = ($tasksByPlan[selectedPlanId] ?? []).find(
      (t) => t.id === selectedTaskId,
    );
    if (task) {
      selectTask(task);
      selectedPlanId = task.planId;
    }
    loadEntries();
  }

  $effect(() => {
    selectedPlanId = $selectedPlan?.id ?? "";
  });
  $effect(() => {
    selectedTaskId = $selectedTask?.id ?? "";
  });

  // ---------------------------------------------------------------------------
  // Form state
  // ---------------------------------------------------------------------------

  let startDay = $state("");
  let startMonth = $state("");
  let startYear = $state("");
  let startTime = $state(""); // HH:MM from <input type="time">

  let endDay = $state("");
  let endMonth = $state("");
  let endYear = $state("");
  let endTime = $state(""); // HH:MM from <input type="time">

  let notes = $state("");
  let editingId = $state<string | null>(null);

  let startError = $state("");
  let endError = $state("");
  let taskError = $state("");

  const isEditing = $derived(editingId !== null);

  function resetForm() {
    startDay = "";
    startMonth = "";
    startYear = "";
    startTime = "";
    endDay = "";
    endMonth = "";
    endYear = "";
    endTime = "";
    notes = "";
    editingId = null;
    startError = "";
    endError = "";
    taskError = "";
  }

  /** Assemble ISO 8601 UTC string from four picker values. */
  function fieldsToIso(
    day: string,
    month: string,
    year: string,
    time: string,
  ): string {
    const [hours, minutes] = time.split(":").map(Number);
    return new Date(
      parseInt(year, 10),
      parseInt(month, 10) - 1,
      parseInt(day, 10),
      hours,
      minutes,
    ).toISOString();
  }

  /** Decompose an ISO 8601 string into picker field values. */
  function isoToFields(iso: string): {
    day: string;
    month: string;
    year: string;
    time: string;
  } {
    const d = new Date(iso);
    const pad = (n: number) => String(n).padStart(2, "0");
    return {
      day: String(d.getDate()),
      month: String(d.getMonth() + 1),
      year: String(d.getFullYear()),
      time: `${pad(d.getHours())}:${pad(d.getMinutes())}`,
    };
  }

  function validate(): boolean {
    let ok = true;
    taskError = "";
    startError = "";
    endError = "";

    if (!selectedTaskId) {
      taskError = "Please select a task";
      ok = false;
    }

    if (!startDay) {
      startError = "Start day is required";
      ok = false;
    } else if (!startMonth) {
      startError = "Start month is required";
      ok = false;
    } else if (!startYear) {
      startError = "Start year is required";
      ok = false;
    } else if (!startTime) {
      startError = "Start time is required";
      ok = false;
    }

    if (!endDay) {
      endError = "End day is required";
      ok = false;
    } else if (!endMonth) {
      endError = "End month is required";
      ok = false;
    } else if (!endYear) {
      endError = "End year is required";
      ok = false;
    } else if (!endTime) {
      endError = "End time is required";
      ok = false;
    }

    if (ok) {
      const start = fieldsToIso(startDay, startMonth, startYear, startTime);
      const end = fieldsToIso(endDay, endMonth, endYear, endTime);
      if (new Date(end) <= new Date(start)) {
        endError = "End time must be after start time";
        ok = false;
      }
    }

    return ok;
  }

  // ---------------------------------------------------------------------------
  // Form submit
  // ---------------------------------------------------------------------------

  let submitting = $state(false);

  async function handleSubmit() {
    if (!validate()) return;
    submitting = true;
    try {
      const startIso = fieldsToIso(startDay, startMonth, startYear, startTime);
      const endIso = fieldsToIso(endDay, endMonth, endYear, endTime);
      if (isEditing) {
        await updateEntry({
          id: editingId!,
          startTime: startIso,
          endTime: endIso,
          notes: notes || undefined,
        });
        addSuccess("Entry updated");
      } else {
        await createManualEntry({
          taskId: selectedTaskId,
          startTime: startIso,
          endTime: endIso,
          notes: notes || undefined,
        });
        addSuccess("Entry saved");
      }
      resetForm();
      await loadEntries();
    } catch (e) {
      addError(String(e));
    } finally {
      submitting = false;
    }
  }

  function handleEdit(entry: TimeEntry) {
    editingId = entry.id;
    const s = isoToFields(entry.startTime);
    startDay = s.day;
    startMonth = s.month;
    startYear = s.year;
    startTime = s.time;
    const e = isoToFields(entry.endTime ?? entry.startTime);
    endDay = e.day;
    endMonth = e.month;
    endYear = e.year;
    endTime = e.time;
    notes = entry.notes ?? "";
    window.scrollTo({ top: 0, behavior: "smooth" });
  }

  // ---------------------------------------------------------------------------
  // Entries table
  // ---------------------------------------------------------------------------

  let entries = $state<TimeEntry[]>([]);
  let loadingEntries = $state(false);
  let confirmDeleteId = $state<string | null>(null);

  async function loadEntries() {
    loadingEntries = true;
    try {
      entries = await getRecentEntries({
        taskId: selectedTaskId || undefined,
        planId: !selectedTaskId && selectedPlanId ? selectedPlanId : undefined,
        limit: $entriesLimit,
      });
    } catch (e) {
      addError("Failed to load entries: " + String(e));
    } finally {
      loadingEntries = false;
    }
  }

  async function handleDelete(id: string) {
    confirmDeleteId = null;
    try {
      await deleteEntry(id);
      addSuccess("Entry deleted");
      if (editingId === id) resetForm();
      await loadEntries();
    } catch (e) {
      addError("Failed to delete entry: " + String(e));
    }
  }

  const allTasks = $derived(Object.values($tasksByPlan).flat());
  const planById = $derived(
    Object.fromEntries($plans.map((p) => [p.id, p.title])),
  );
  const taskById = $derived(Object.fromEntries(allTasks.map((t) => [t.id, t])));

  function entryDurationSeconds(entry: TimeEntry): number | null {
    if (!entry.endTime) return null;
    return Math.floor(
      (new Date(entry.endTime).getTime() -
        new Date(entry.startTime).getTime()) /
        1000,
    );
  }

  function formatDateTime(iso: string): string {
    return new Date(iso).toLocaleString(undefined, {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  }

  onMount(() => {
    loadEntries();
  });
</script>

<div class="view">
  <!-- Form -->
  <section class="form-section">
    <h3 class="section-title">{isEditing ? "Edit Entry" : "New Entry"}</h3>

    <div class="form">
      <!-- Plan / Task row -->
      <div class="row-2">
        <div class="field">
          <label class="label" for="plan-select">Plan</label>
          <Select
            id="plan-select"
            options={planOptions}
            bind:value={selectedPlanId}
            placeholder="Select a plan…"
            onchange={onPlanChange}
          />
          {#if taskError && !selectedTaskId}
            <span class="error-msg">{taskError}</span>
          {/if}
        </div>
        <div class="field">
          <label class="label" for="task-select">Task</label>
          <Select
            id="task-select"
            options={taskOptions}
            bind:value={selectedTaskId}
            placeholder={selectedPlanId
              ? "Select a task…"
              : "Select a plan first"}
            onchange={onTaskChange}
          />
        </div>
      </div>

      <!-- Start picker -->
      <div class="picker-group">
        <span class="picker-label">Start</span>
        <div class="picker-row" class:has-error={!!startError}>
          <input
            class="picker-input day-input"
            type="number"
            min="1"
            max="31"
            placeholder="DD"
            bind:value={startDay}
          />
          <div class="month-select">
            <Select
              options={MONTH_OPTIONS}
              bind:value={startMonth}
              placeholder="Month"
            />
          </div>
          <input
            class="picker-input year-input"
            type="number"
            placeholder="YYYY"
            bind:value={startYear}
          />
          <input
            class="picker-input time-input"
            type="time"
            step="60"
            bind:value={startTime}
          />
        </div>
        {#if startError}
          <span class="error-msg">{startError}</span>
        {/if}
      </div>

      <!-- End picker -->
      <div class="picker-group">
        <span class="picker-label">End</span>
        <div class="picker-row" class:has-error={!!endError}>
          <input
            class="picker-input day-input"
            type="number"
            min="1"
            max="31"
            placeholder="DD"
            bind:value={endDay}
          />
          <div class="month-select">
            <Select
              options={MONTH_OPTIONS}
              bind:value={endMonth}
              placeholder="Month"
            />
          </div>
          <input
            class="picker-input year-input"
            type="number"
            placeholder="YYYY"
            bind:value={endYear}
          />
          <input
            class="picker-input time-input"
            type="time"
            step="60"
            bind:value={endTime}
          />
        </div>
        {#if endError}
          <span class="error-msg">{endError}</span>
        {/if}
      </div>

      <!-- Notes -->
      <div class="field">
        <label class="label" for="notes-input">Notes (optional)</label>
        <textarea
          id="notes-input"
          class="notes-textarea"
          bind:value={notes}
          rows="3"
          placeholder="Add a note…"
        ></textarea>
      </div>

      <!-- Actions -->
      <div class="form-actions">
        <Button
          variant="primary"
          loading={submitting}
          disabled={submitting}
          onclick={handleSubmit}
        >
          {isEditing ? "Update Entry" : "Save Entry"}
        </Button>
        {#if isEditing}
          <Button variant="ghost" disabled={submitting} onclick={resetForm}>
            Cancel
          </Button>
        {/if}
      </div>
    </div>
  </section>

  <!-- Recent entries -->
  <section class="entries-section">
    <h3 class="section-title">Recent Entries</h3>

    {#if loadingEntries}
      <div class="loading-row">
        <Spinner size="sm" />
        <span>Loading…</span>
      </div>
    {:else if entries.length === 0}
      <EmptyState
        message="No entries yet. Fill in the form above to add one."
      />
    {:else}
      <div class="table-wrap">
        <table>
          <thead>
            <tr>
              <th>Task</th>
              <th>Plan</th>
              <th>Start</th>
              <th>End</th>
              <th>Duration</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {#each entries as entry (entry.id)}
              {@const task = taskById[entry.taskId]}
              {@const planTitle = task ? planById[task.planId] : "—"}
              {@const dur = entryDurationSeconds(entry)}
              <tr class:editing-row={editingId === entry.id}>
                <td>{task?.title ?? entry.taskId}</td>
                <td class="muted">{planTitle ?? "—"}</td>
                <td class="muted">{formatDateTime(entry.startTime)}</td>
                <td class="muted"
                  >{entry.endTime ? formatDateTime(entry.endTime) : "—"}</td
                >
                <td class="muted">{dur !== null ? formatDuration(dur) : "—"}</td
                >
                <td class="action-cell">
                  {#if confirmDeleteId === entry.id}
                    <span class="confirm-row">
                      <button
                        class="text-btn danger"
                        onclick={() => handleDelete(entry.id)}>Sure?</button
                      >
                      <button
                        class="text-btn"
                        onclick={() => (confirmDeleteId = null)}>Cancel</button
                      >
                    </span>
                  {:else}
                    <span class="action-row">
                      <button
                        class="text-btn"
                        onclick={() => handleEdit(entry)}
                        title="Edit entry">Edit</button
                      >
                      <button
                        class="text-btn danger"
                        onclick={() => (confirmDeleteId = entry.id)}
                        title="Delete entry">Delete</button
                      >
                    </span>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </section>
</div>

<style>
  .view {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .section-title {
    font-size: var(--font-size-base);
    color: var(--text-muted);
    margin-bottom: 0.75rem;
    font-weight: 500;
  }

  /* Form */
  .form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-width: 640px;
  }

  .row-2 {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.75rem;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .label {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }

  /* Picker */
  .picker-group {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .picker-label {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }

  .picker-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .picker-input {
    padding: 0.45rem 0.75rem;
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text);
    font-family: var(--font);
    font-size: var(--font-size-base);
    transition: border-color 0.15s;
  }

  .picker-input:hover {
    border-color: var(--text-muted);
  }

  .picker-input:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  .picker-row.has-error .picker-input,
  .picker-row.has-error :global(select) {
    border-color: var(--danger);
  }

  .day-input {
    width: 3.75rem;
  }

  .month-select {
    flex: 1;
    min-width: 8rem;
    max-width: 11rem;
  }

  .year-input {
    width: 5rem;
  }

  .time-input {
    width: 6.5rem;
  }

  .error-msg {
    font-size: var(--font-size-sm);
    color: var(--danger);
  }

  .notes-textarea {
    width: 100%;
    padding: 0.45rem 0.75rem;
    background: var(--bg-input);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text);
    font-family: var(--font);
    font-size: var(--font-size-base);
    resize: vertical;
    transition: border-color 0.15s;
  }

  .notes-textarea:hover {
    border-color: var(--text-muted);
  }

  .notes-textarea:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }

  .form-actions {
    display: flex;
    gap: 0.5rem;
  }

  /* Loading */
  .loading-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--text-muted);
    font-size: var(--font-size-sm);
    padding: 1rem 0;
  }

  /* Table */
  .table-wrap {
    overflow-x: auto;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    font-size: var(--font-size-sm);
  }

  th {
    text-align: left;
    padding: 0.4rem 0.75rem;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
    font-weight: 500;
    white-space: nowrap;
  }

  td {
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid color-mix(in srgb, var(--border) 50%, transparent);
    white-space: nowrap;
  }

  tr:last-child td {
    border-bottom: none;
  }

  .editing-row td {
    background: color-mix(in srgb, var(--accent) 8%, transparent);
  }

  .muted {
    color: var(--text-muted);
  }

  .action-cell {
    text-align: right;
    width: 1%;
  }

  .action-row,
  .confirm-row {
    display: flex;
    gap: 0.25rem;
    justify-content: flex-end;
  }

  .text-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-family: var(--font);
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    padding: 0.1rem 0.25rem;
    border-radius: var(--radius);
    transition: color 0.15s;
  }

  .text-btn:hover {
    color: var(--text);
  }

  .text-btn.danger {
    color: var(--danger);
  }

  .text-btn.danger:hover {
    opacity: 0.75;
  }

  .text-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
</style>
