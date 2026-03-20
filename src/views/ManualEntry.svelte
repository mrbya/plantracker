<script lang="ts">
  import { onMount } from 'svelte';

  import { createManualEntry, deleteEntry, getRecentEntries, updateEntry } from '$lib/api';
  import Button from '$lib/components/ui/Button.svelte';
  import EmptyState from '$lib/components/ui/EmptyState.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import Spinner from '$lib/components/ui/Spinner.svelte';
  import { addError, addSuccess } from '$lib/stores/notifications';
  import {
    plans,
    selectTask,
    selectedPlan,
    selectedTask,
    tasksByPlan,
  } from '$lib/stores/planner';
  import { entriesLimit } from '$lib/stores/settings';
  import type { TimeEntry } from '$lib/types';
  import { formatDuration } from '$lib/utils/duration';

  // ---------------------------------------------------------------------------
  // Dropdown state
  // ---------------------------------------------------------------------------

  let selectedPlanId = $state($selectedPlan?.id ?? '');
  let selectedTaskId = $state($selectedTask?.id ?? '');

  const planOptions = $derived($plans.map((p) => ({ value: p.id, label: p.title })));
  const taskOptions = $derived(
    selectedPlanId
      ? ($tasksByPlan[selectedPlanId] ?? []).map((t) => ({ value: t.id, label: t.title }))
      : []
  );

  function onPlanChange() {
    selectedTaskId = '';
    selectedPlan.set($plans.find((p) => p.id === selectedPlanId) ?? null);
    selectedTask.set(null);
    loadEntries();
  }

  function onTaskChange() {
    const task = ($tasksByPlan[selectedPlanId] ?? []).find((t) => t.id === selectedTaskId);
    if (task) {
      selectTask(task);
      selectedPlanId = task.planId;
    }
    loadEntries();
  }

  $effect(() => { selectedPlanId = $selectedPlan?.id ?? ''; });
  $effect(() => { selectedTaskId = $selectedTask?.id ?? ''; });

  // ---------------------------------------------------------------------------
  // Form state
  // ---------------------------------------------------------------------------

  let startTime = $state('');
  let endTime = $state('');
  let notes = $state('');
  let editingId = $state<string | null>(null);

  let startError = $state('');
  let endError = $state('');
  let taskError = $state('');

  const isEditing = $derived(editingId !== null);

  function resetForm() {
    startTime = '';
    endTime = '';
    notes = '';
    editingId = null;
    startError = '';
    endError = '';
    taskError = '';
  }

  /** Convert a local datetime-local string to ISO 8601 UTC. */
  function localToIso(local: string): string {
    return new Date(local).toISOString();
  }

  /** Convert an ISO 8601 string to datetime-local format (YYYY-MM-DDTHH:MM). */
  function isoToLocal(iso: string): string {
    const d = new Date(iso);
    const pad = (n: number) => String(n).padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`;
  }

  function validate(): boolean {
    let ok = true;
    taskError = '';
    startError = '';
    endError = '';

    if (!selectedTaskId) {
      taskError = 'Please select a task';
      ok = false;
    }
    if (!startTime) {
      startError = 'Start time is required';
      ok = false;
    }
    if (!endTime) {
      endError = 'End time is required';
      ok = false;
    }
    if (startTime && endTime && new Date(endTime) <= new Date(startTime)) {
      endError = 'End time must be after start time';
      ok = false;
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
      if (isEditing) {
        await updateEntry({
          id: editingId!,
          startTime: localToIso(startTime),
          endTime: localToIso(endTime),
          notes: notes || undefined,
        });
        addSuccess('Entry updated');
      } else {
        await createManualEntry({
          taskId: selectedTaskId,
          startTime: localToIso(startTime),
          endTime: localToIso(endTime),
          notes: notes || undefined,
        });
        addSuccess('Entry saved');
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
    startTime = isoToLocal(entry.startTime);
    endTime = entry.endTime ? isoToLocal(entry.endTime) : '';
    notes = entry.notes ?? '';
    // Scroll to top of form
    window.scrollTo({ top: 0, behavior: 'smooth' });
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
      addError('Failed to load entries: ' + String(e));
    } finally {
      loadingEntries = false;
    }
  }

  async function handleDelete(id: string) {
    confirmDeleteId = null;
    try {
      await deleteEntry(id);
      addSuccess('Entry deleted');
      if (editingId === id) resetForm();
      await loadEntries();
    } catch (e) {
      addError('Failed to delete entry: ' + String(e));
    }
  }

  const allTasks = $derived(Object.values($tasksByPlan).flat());
  const planById = $derived(Object.fromEntries($plans.map((p) => [p.id, p.title])));
  const taskById = $derived(Object.fromEntries(allTasks.map((t) => [t.id, t])));

  function entryDurationSeconds(entry: TimeEntry): number | null {
    if (!entry.endTime) return null;
    return Math.floor(
      (new Date(entry.endTime).getTime() - new Date(entry.startTime).getTime()) / 1000
    );
  }

  function formatDateTime(iso: string): string {
    return new Date(iso).toLocaleString(undefined, {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  onMount(() => { loadEntries(); });
</script>

<div class="view">
  <!-- Form -->
  <section class="form-section">
    <h3 class="section-title">{isEditing ? 'Edit Entry' : 'New Entry'}</h3>

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
            placeholder={selectedPlanId ? 'Select a task…' : 'Select a plan first'}
            onchange={onTaskChange}
          />
        </div>
      </div>

      <!-- Time row -->
      <div class="row-2">
        <Input
          type="datetime-local"
          label="Start"
          bind:value={startTime}
          error={startError}
        />
        <Input
          type="datetime-local"
          label="End"
          bind:value={endTime}
          error={endError}
        />
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
          {isEditing ? 'Update Entry' : 'Save Entry'}
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
      <EmptyState message="No entries yet. Fill in the form above to add one." />
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
              {@const planTitle = task ? planById[task.planId] : '—'}
              {@const dur = entryDurationSeconds(entry)}
              <tr class:editing-row={editingId === entry.id}>
                <td>{task?.title ?? entry.taskId}</td>
                <td class="muted">{planTitle ?? '—'}</td>
                <td class="muted">{formatDateTime(entry.startTime)}</td>
                <td class="muted">{entry.endTime ? formatDateTime(entry.endTime) : '—'}</td>
                <td class="muted">{dur !== null ? formatDuration(dur) : '—'}</td>
                <td class="action-cell">
                  {#if confirmDeleteId === entry.id}
                    <span class="confirm-row">
                      <button class="text-btn danger" onclick={() => handleDelete(entry.id)}>Sure?</button>
                      <button class="text-btn" onclick={() => (confirmDeleteId = null)}>Cancel</button>
                    </span>
                  {:else}
                    <span class="action-row">
                      <button class="text-btn" onclick={() => handleEdit(entry)} title="Edit entry">󰏫</button>
                      <button class="text-btn danger" onclick={() => (confirmDeleteId = entry.id)} title="Delete entry">󰆴</button>
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

  .error-msg {
    font-size: var(--font-size-sm);
    color: var(--danger);
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
