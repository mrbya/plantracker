<script lang="ts">
  /**
   * Time Tracking view — the primary timer interface.
   *
   * Allows the user to select a plan and optional task, start/stop a timer,
   * and review recent time entries in a table.  The plan and task selections
   * are shared with the other views via the `planner` store so switching tabs
   * preserves the context.
   *
   * Scoping for `getRecentEntries`:
   *   - Task selected  → entries for that specific task
   *   - Plan only      → entries for the plan (all tasks combined)
   *   - Neither        → entries for all plans
   *
   * Timer button states:
   *   - `$isRunning === false` → green "Start Timer" button (disabled if no plan selected)
   *   - `$isRunning === true`  → red "Stop — Xh Ym" button showing elapsed time
   *   Both states are disabled while `timerBusy` is `true` to prevent double-clicks.
   *
   * Delete confirmation:
   *   Rather than a modal dialog, a two-step inline pattern is used: the first
   *   click on the delete icon sets `confirmDeleteId` to the entry's ID,
   *   replacing the icon with "Sure? / Cancel" text buttons.  This avoids
   *   blocking the entire UI.  Only one row can be in the confirm state at a
   *   time because `confirmDeleteId` holds a single ID.
   */
  import { onMount } from "svelte";

  import { deleteEntry, getRecentEntries } from "$lib/api";
  import Button from "$lib/components/ui/Button.svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";
  import SearchableSelect from "$lib/components/ui/SearchableSelect.svelte";
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
  import * as m from "$lib/paraglide/messages";

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
      ? [
          { value: "", label: m.label_no_task() },
          ...($tasksByPlan[selectedPlanId] ?? []).map((t) => ({
            value: t.id,
            label: t.title,
          })),
        ]
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
      addError(m.toast_load_failed({ error: String(e) }));
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
    if (!selectedPlanId) return;
    timerBusy = true;
    try {
      await start(selectedPlanId, selectedTaskId || undefined);
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
      addSuccess(m.toast_entry_deleted());
      await loadEntries();
    } catch (e) {
      addError(m.toast_delete_failed({ error: String(e) }));
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
        <label class="label" for="plan-select">{m.label_plan()}</label>
        <SearchableSelect
          id="plan-select"
          options={planOptions}
          bind:value={selectedPlanId}
          placeholder={m.placeholder_select_plan()}
          onchange={onPlanChange}
        />
      </div>

      <div class="field">
        <label class="label" for="task-select">{m.label_task()}</label>
        <SearchableSelect
          id="task-select"
          options={taskOptions}
          bind:value={selectedTaskId}
          placeholder={selectedPlanId
            ? undefined
            : m.placeholder_select_plan_first()}
          disabled={!selectedPlanId}
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
          {m.timer_stop({ elapsed: formatDuration($elapsedSeconds) })}
        </Button>
      {:else}
        <Button
          variant="success"
          loading={timerBusy}
          disabled={timerBusy || !selectedPlanId}
          onclick={handleStart}
          title={$isRunning ? "A timer is already running" : undefined}
        >
          {m.timer_start()}
        </Button>
      {/if}
    </div>
  </section>

  <!-- Recent entries -->
  <section class="entries-section">
    <h3 class="section-title">{m.entries_title()}</h3>

    {#if loadingEntries}
      <div class="loading-row">
        <Spinner size="sm" />
        <span>{m.entries_loading()}</span>
      </div>
    {:else if entries.length === 0}
      <EmptyState message={m.entries_empty()} />
    {:else}
      <div class="table-wrap">
        <table>
          <thead>
            <tr>
              <th>{m.label_task()}</th>
              <th>{m.label_plan()}</th>
              <th>{m.label_start()}</th>
              <th>{m.label_end()}</th>
              <th>{m.label_duration()}</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            {#each entries as entry (entry.id)}
              {@const task = entry.taskId ? taskById[entry.taskId] : null}
              {@const planTitle = planById[entry.planId] ?? "—"}
              {@const dur = entryDurationSeconds(entry)}
              <tr>
                <td>{task?.title ?? m.label_no_task()}</td>
                <td class="muted">{planTitle}</td>
                <td class="muted">{formatDateTime(entry.startTime)}</td>
                <td>
                  {#if entry.endTime}
                    <span class="muted">{formatDateTime(entry.endTime)}</span>
                  {:else}
                    <span class="running">{m.timer_running()}</span>
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
                        onclick={() => handleDelete(entry.id)}
                        >{m.entries_delete_confirm()}</button
                      >
                      <button
                        class="text-btn"
                        onclick={() => (confirmDeleteId = null)}
                        >{m.entries_delete_cancel()}</button
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
