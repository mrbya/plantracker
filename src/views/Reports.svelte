<script lang="ts">
  import { exportReportCsv, generateReport } from "$lib/api";
  import Button from "$lib/components/ui/Button.svelte";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import { addError, addSuccess } from "$lib/stores/notifications";
  import {
    plans,
    selectTask,
    selectedPlan,
    selectedTask,
    tasksByPlan,
  } from "$lib/stores/planner";
  import type { ReportResult } from "$lib/types";
  import { formatDateTime } from "$lib/utils/datetime";
  import { formatDuration } from "$lib/utils/duration";

  // ---------------------------------------------------------------------------
  // Month helpers
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
  // Date range state (defaults to current month/year)
  // ---------------------------------------------------------------------------

  const now = new Date();
  let fromMonth = $state(String(now.getMonth() + 1));
  let fromYear = $state(String(now.getFullYear()));
  let toMonth = $state(String(now.getMonth() + 1));
  let toYear = $state(String(now.getFullYear()));

  // ---------------------------------------------------------------------------
  // Plan / task filter state
  // ---------------------------------------------------------------------------

  let selectedPlanId = $state($selectedPlan?.id ?? "");
  let selectedTaskId = $state($selectedTask?.id ?? "");

  const planOptions = $derived([
    { value: "", label: "All plans" },
    ...$plans.map((p) => ({ value: p.id, label: p.title })),
  ]);
  const taskOptions = $derived(
    selectedPlanId
      ? [
          { value: "", label: "All tasks" },
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
  }

  function onTaskChange() {
    const task = ($tasksByPlan[selectedPlanId] ?? []).find(
      (t) => t.id === selectedTaskId,
    );
    if (task) {
      selectTask(task);
      selectedPlanId = task.planId;
    }
  }

  $effect(() => {
    selectedPlanId = $selectedPlan?.id ?? "";
  });
  $effect(() => {
    selectedTaskId = $selectedTask?.id ?? "";
  });

  // ---------------------------------------------------------------------------
  // Report state
  // ---------------------------------------------------------------------------

  let report = $state<ReportResult | null>(null);
  let generated = $state(false); // true once a generate attempt has completed
  let generating = $state(false);
  let exporting = $state(false);

  async function handleGenerate() {
    generating = true;
    report = null;
    try {
      report = await generateReport({
        planId: selectedPlanId || undefined,
        taskId: selectedTaskId || undefined,
        fromYear: parseInt(fromYear, 10),
        fromMonth: parseInt(fromMonth, 10),
        toYear: parseInt(toYear, 10),
        toMonth: parseInt(toMonth, 10),
      });
      generated = true;
    } catch (e) {
      addError("Failed to generate report: " + String(e));
    } finally {
      generating = false;
    }
  }

  async function handleExport() {
    if (!report) return;
    exporting = true;
    try {
      const path = await exportReportCsv(report);
      addSuccess("Exported to " + path);
    } catch (e) {
      const msg = String(e);
      if (!msg.includes("Export cancelled")) {
        addError("Export failed: " + msg);
      }
    } finally {
      exporting = false;
    }
  }

</script>

<div class="view">
  <!-- Controls -->
  <section class="controls">
    <!-- Date range -->
    <div class="range-group">
      <span class="range-label">From</span>
      <div class="range-fields">
        <Select
          options={MONTH_OPTIONS}
          bind:value={fromMonth}
          placeholder="Month"
        />
        <Input type="number" bind:value={fromYear} label="" />
      </div>
    </div>

    <div class="range-group">
      <span class="range-label">To</span>
      <div class="range-fields">
        <Select
          options={MONTH_OPTIONS}
          bind:value={toMonth}
          placeholder="Month"
        />
        <Input type="number" bind:value={toYear} label="" />
      </div>
    </div>

    <!-- Plan / task filter -->
    <div class="filter-group">
      <div class="field">
        <label class="label" for="plan-select">Plan</label>
        <Select
          id="plan-select"
          options={planOptions}
          bind:value={selectedPlanId}
          onchange={onPlanChange}
        />
      </div>
      <div class="field">
        <label class="label" for="task-select">Task</label>
        <Select
          id="task-select"
          options={taskOptions}
          bind:value={selectedTaskId}
          placeholder={selectedPlanId ? undefined : "Select a plan first"}
          onchange={onTaskChange}
        />
      </div>
    </div>

    <!-- Action buttons -->
    <div class="action-btns">
      <Button
        variant="primary"
        loading={generating}
        disabled={generating}
        onclick={handleGenerate}
      >
        Generate
      </Button>
      <Button
        variant="ghost"
        loading={exporting}
        disabled={exporting || !report}
        onclick={handleExport}
      >
        Export CSV
      </Button>
    </div>
  </section>

  <!-- Report results -->
  {#if generated}
    <section class="results-section">
      {#if report}
        <h3 class="subject-label">{report.subjectLabel}</h3>

        {#if report.entries.length === 0}
          <EmptyState message="No entries found for this period." />
        {:else}
          <div class="grand-total">
            Total: <strong>{formatDuration(report.grandTotalSeconds)}</strong>
          </div>
          <div class="table-wrap">
            <table>
              <thead>
                <tr>
                  <th>Task</th>
                  <th>Plan</th>
                  <th>Start</th>
                  <th>End</th>
                  <th class="num-col">Duration</th>
                  <th>Notes</th>
                </tr>
              </thead>
              <tbody>
                {#each report.entries as entry, i (i)}
                  <tr>
                    <td>{entry.taskTitle}</td>
                    <td class="muted">{entry.planTitle}</td>
                    <td class="muted">{formatDateTime(entry.startTime)}</td>
                    <td class="muted">{formatDateTime(entry.endTime)}</td>
                    <td class="num-col">{formatDuration(entry.durationSeconds)}</td>
                    <td class="muted notes-col">{entry.notes ?? "—"}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      {/if}
    </section>
  {/if}
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

  .range-group {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .range-label {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }

  .range-fields {
    display: flex;
    gap: 0.5rem;
    align-items: flex-end;
  }

  .range-fields :global(.field) {
    margin: 0;
  }

  /* Year input — compact width */
  .range-fields :global(input[type="number"]) {
    width: 5rem;
  }

  .filter-group {
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

  .action-btns {
    display: flex;
    gap: 0.5rem;
    flex-shrink: 0;
  }

  /* Results */
  .subject-label {
    font-size: var(--font-size-base);
    color: var(--text-muted);
    font-weight: 500;
    margin-bottom: 0.75rem;
  }

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

  tbody tr:last-child td {
    border-bottom: none;
  }


  .num-col {
    text-align: right;
  }

  .muted {
    color: var(--text-muted);
  }

  .grand-total {
    font-size: var(--font-size-base);
    color: var(--text-muted);
    margin-bottom: 0.75rem;
  }

  .grand-total strong {
    color: var(--text);
    font-weight: 600;
  }

  .notes-col {
    max-width: 20rem;
    overflow: hidden;
    text-overflow: ellipsis;
  }
</style>
