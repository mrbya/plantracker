<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { userEvent, within } from "storybook/test";
  import { setInvokeHandler } from "../__mocks__/tauri-api-core";
  import Reports from "../../views/Reports.svelte";
  import {
    plans,
    tasksByPlan,
    selectedPlan,
    selectedTask,
  } from "$lib/stores/planner";

  const fixturePlans = [
    {
      id: "p1",
      graphId: "g1",
      title: "Sprint 12",
      syncedAt: "2026-03-01T00:00:00Z",
    },
    {
      id: "p2",
      graphId: "g2",
      title: "Backlog",
      syncedAt: "2026-03-01T00:00:00Z",
    },
  ];
  const fixtureTasks = [
    {
      id: "t1",
      graphId: "gt1",
      planId: "p1",
      title: "Implement auth flow",
      syncedAt: "2026-03-01T00:00:00Z",
    },
    {
      id: "t2",
      graphId: "gt2",
      planId: "p1",
      title: "Write migration",
      syncedAt: "2026-03-01T00:00:00Z",
    },
  ];

  const fixtureReport = {
    subjectLabel: "Sprint 12",
    grandTotalSeconds: 14520,
    entries: [
      {
        taskTitle: "Implement auth flow",
        planTitle: "Sprint 12",
        startTime: "2026-03-20T09:00:00Z",
        endTime: "2026-03-20T11:30:00Z",
        durationSeconds: 9000,
        notes: null,
      },
      {
        taskTitle: "Write migration",
        planTitle: "Sprint 12",
        startTime: "2026-03-21T13:00:00Z",
        endTime: "2026-03-21T14:30:00Z",
        durationSeconds: 5400,
        notes: "Added indexes",
      },
      {
        taskTitle: "Write migration",
        planTitle: "Sprint 12",
        startTime: "2026-03-22T10:00:00Z",
        endTime: "2026-03-22T10:02:00Z",
        durationSeconds: 120,
        notes: null,
      },
    ],
  };

  const multiPlanReport = {
    subjectLabel: "All plans",
    grandTotalSeconds: 32400,
    entries: [
      ...fixtureReport.entries,
      {
        taskTitle: "Triage backlog",
        planTitle: "Backlog",
        startTime: "2026-03-19T10:00:00Z",
        endTime: "2026-03-19T14:00:00Z",
        durationSeconds: 14400,
        notes: null,
      },
      {
        taskTitle: "Estimate Q2 items",
        planTitle: "Backlog",
        startTime: "2026-03-18T15:00:00Z",
        endTime: "2026-03-18T16:00:00Z",
        durationSeconds: 3600,
        notes: null,
      },
    ],
  };

  function seedPlans() {
    plans.set(fixturePlans);
    tasksByPlan.set({ p1: fixtureTasks, p2: [] });
    selectedPlan.set(null);
    selectedTask.set(null);
  }

  const { Story } = defineMeta({
    title: "PlanTracker/Views/Reports",
    component: Reports,
    tags: ["autodocs"],
    parameters: { layout: "fullscreen" },
  });
</script>

<!-- Clicks Generate; report returns no entries for the period -->
<Story
  name="NoEntries"
  play={async ({ canvasElement }) => {
    seedPlans();
    setInvokeHandler("generate_report", () => ({
      entries: [],
      grandTotalSeconds: 0,
      subjectLabel: "Sprint 12",
    }));
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: /generate/i }));
  }}
/>

<!-- Clicks Generate; report returns fixture entries with a non-zero grand total -->
<Story
  name="WithEntries"
  play={async ({ canvasElement }) => {
    seedPlans();
    selectedPlan.set(fixturePlans[0]);
    setInvokeHandler("generate_report", () => fixtureReport);
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: /generate/i }));
  }}
/>

<!-- No plan selected (all-plans scope), entries from multiple plans -->
<Story
  name="AllPlansFilter"
  play={async ({ canvasElement }) => {
    seedPlans();
    setInvokeHandler("generate_report", () => multiPlanReport);
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: /generate/i }));
  }}
/>

<!--
  `generate_report` never resolves so the Generate button stays in loading state.
-->
<Story
  name="Loading"
  play={async ({ canvasElement }) => {
    seedPlans();
    setInvokeHandler("generate_report", () => new Promise(() => {}));
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: /generate/i }));
  }}
/>
