<script lang="ts">
  import { onMount } from "svelte";
  import { get } from "svelte/store";

  import { deleteEntry, getRecentEntries } from "$lib/api";
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
  import { elapsedSeconds, isRunning, start, stop } from "$lib/stores/timer";
  import { entriesLimit } from "$lib/stores/settings";
  import type { TimeEntry } from "$lib/types";
  import { formatDateTime } from "$lib/utils/datetime";
  import { formatDuration } from "$lib/utils/duration";

  // ---------------------------------------------------------------------------
  // Dropdown state (string IDs for <Select> binding)
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
      selectTask(task); // also syncs selectedPlan
      selectedPlanId = task.planId;
    }
    loadEntries();
  }

  // Sync selectedPlanId/selectedTaskId when store changes externally
  $effect(() => {
    selectedPlanId = $selectedPlan?.id ?? "";
  });
  $effect(() => {
    selectedTaskId = $selectedTask?.id ?? "";
  });

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

  // Task/plan lookup helpers for the entries table
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

  // ---------------------------------------------------------------------------
  // Timer controls
  // ---------------------------------------------------------------------------

  let timerBusy = $state(false);

  async function handleStart() {
    if (!selectedTaskId) return;
    timerBusy = true;
    try {
      await start(selectedTaskId);
      await loadEntries();
    } finally {
      timerBusy = false;
    }
  }

  async function handleStop() {
    timerBusy = true;
    try {
      await stop();
      await loadEntries();
    } finally {
      timerBusy = false;
    }
  }

  // ---------------------------------------------------------------------------
  // Delete (Phase 6 command not yet available — stubbed)
  // ---------------------------------------------------------------------------

  async function handleDelete(id: string) {
    confirmDeleteId = null;
    try {
      await deleteEntry(id);
      addSuccess("Entry deleted");
      await loadEntries();
    } catch (e) {
      addError("Failed to delete entry: " + String(e));
    }
  }

  // ---------------------------------------------------------------------------
  // Lifecycle
  // ---------------------------------------------------------------------------

  onMount(() => {
    loadEntries();
  });
</script>

<div class="view">
  <!-- Controls -->
  <section class="controls">
    <div class="dropdowns">
      <div class="field">
        <label class="label" for="plan-select">Plan</label>
        <Select
          id="plan-select"
          options={planOptions}
          bind:value={selectedPlanId}
          placeholder="Select a plan…"
          onchange={onPlanChange}
        />
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

    <div class="timer-btn-wrap">
      {#if $isRunning}
        <Button
          variant="danger"
          loading={timerBusy}
          disabled={timerBusy}
          onclick={handleStop}
        >
          Stop — {formatDuration($elapsedSeconds)}
        </Button>
      {:else}
        <Button
          variant="success"
          loading={timerBusy}
          disabled={timerBusy || !selectedTaskId}
          onclick={handleStart}
          title={$isRunning ? "A timer is already running" : undefined}
        >
          Start Timer
        </Button>
      {/if}
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
      <EmptyState message="No entries yet. Select a task and start a timer." />
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
              <tr>
                <td>{task?.title ?? entry.taskId}</td>
                <td class="muted">{planTitle ?? "—"}</td>
                <td class="muted">{formatDateTime(entry.startTime)}</td>
                <td>
                  {#if entry.endTime}
                    <span class="muted">{formatDateTime(entry.endTime)}</span>
                  {:else}
                    <span class="running">Running…</span>
                  {/if}
                </td>
                <td class="muted">
                  {dur !== null ? formatDuration(dur) : "—"}
                </td>
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
                    <button
                      class="text-btn danger"
                      onclick={() => (confirmDeleteId = entry.id)}
                      title="Delete entry"
                    >
                      󰆴
                    </button>
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

  /* Controls */
  .controls {
    display: flex;
    align-items: flex-end;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .dropdowns {
    display: flex;
    gap: 0.75rem;
    flex: 1;
    min-width: 0;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    flex: 1;
    min-width: 0;
  }

  .label {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }

  .timer-btn-wrap {
    flex-shrink: 0;
  }

  /* Section heading */
  .section-title {
    font-size: var(--font-size-base);
    color: var(--text-muted);
    margin-bottom: 0.75rem;
    font-weight: 500;
  }

  /* Loading row */
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

  .muted {
    color: var(--text-muted);
  }

  .running {
    color: var(--timer-active);
    font-weight: 500;
  }

  /* Delete controls */
  .action-cell {
    text-align: right;
    width: 1%;
  }

  .confirm-row {
    display: flex;
    gap: 0.4rem;
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
