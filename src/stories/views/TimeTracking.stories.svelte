<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { setInvokeHandler } from "../__mocks__/tauri-api-core";
  import TimeTracking from "../../views/TimeTracking.svelte";
  import {
    plans,
    tasksByPlan,
    selectedPlan,
    selectedTask,
  } from "$lib/stores/planner";
  import { elapsedSeconds, start } from "$lib/stores/timer";

  // ── Shared fixture data ───────────────────────────────────────────────────

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

  const fixtureEntries = [
    {
      id: "e1",
      planId: "p1",
      taskId: "t1",
      startTime: "2026-03-24T09:00:00Z",
      endTime: "2026-03-24T11:30:00Z",
      notes: null,
      createdAt: "2026-03-24T09:00:00Z",
    },
    {
      id: "e2",
      planId: "p1",
      taskId: "t2",
      startTime: "2026-03-23T13:00:00Z",
      endTime: "2026-03-23T14:15:00Z",
      notes: "Added indexes",
      createdAt: "2026-03-23T13:00:00Z",
    },
  ];

  function seedPlans() {
    plans.set(fixturePlans);
    tasksByPlan.set({ p1: fixtureTasks, p2: [] });
    selectedPlan.set(null);
    selectedTask.set(null);
  }

  const { Story } = defineMeta({
    title: "PlanTracker/Views/TimeTracking",
    component: TimeTracking,
    tags: ["autodocs"],
    parameters: { layout: "fullscreen" },
  });
</script>

<!-- Default: no plans in store → EmptyState in entries section -->
<Story
  name="NoPlans"
  play={async () => {
    plans.set([]);
    tasksByPlan.set({});
    selectedPlan.set(null);
    selectedTask.set(null);
  }}
/>

<!-- Plans and tasks loaded, no selection, no timer -->
<Story
  name="IdleWithPlans"
  play={async () => {
    seedPlans();
    setInvokeHandler("get_recent_entries", () => fixtureEntries);
  }}
/>

<!-- Sprint 12 pre-selected, tasks visible in task dropdown -->
<Story
  name="PlanSelected"
  loaders={[() => {
    seedPlans();
    selectedPlan.set(fixturePlans[0]);
  }]}
  play={async () => {
    seedPlans();
    selectedPlan.set(fixturePlans[0]);
    setInvokeHandler("get_recent_entries", () => fixtureEntries);
  }}
/>

<!--
  Timer actively running.
  `start()` is called with the mocked invoke so `isRunning` becomes true.
  `elapsedSeconds` is then set to 754 (12m 34s) to show a non-trivial duration.
  Note: the 1-second tick interval continues running while this story is open.
-->
<Story
  name="TimerRunning"
  loaders={[() => {
    seedPlans();
    selectedPlan.set(fixturePlans[0]);
    selectedTask.set(fixtureTasks[0]);
  }]}
  play={async () => {
    seedPlans();
    selectedPlan.set(fixturePlans[0]);
    selectedTask.set(fixtureTasks[0]);
    setInvokeHandler("get_recent_entries", () => fixtureEntries);
    await start("p1", "t1");
    elapsedSeconds.set(754);
  }}
/>

<!-- Timer running but no task selected (plan-level entry) -->
<Story
  name="TimerRunningNoTask"
  loaders={[() => {
    seedPlans();
    selectedPlan.set(fixturePlans[0]);
  }]}
  play={async () => {
    seedPlans();
    selectedPlan.set(fixturePlans[0]);
    selectedTask.set(null);
    setInvokeHandler("get_recent_entries", () => fixtureEntries);
    await start("p1");
    elapsedSeconds.set(3661);
  }}
/>
