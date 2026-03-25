<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { userEvent, within } from "storybook/test";
  import { setInvokeHandler } from "../__mocks__/tauri-api-core";
  import ManualEntry from "../../views/ManualEntry.svelte";
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

  function seedPlans() {
    plans.set(fixturePlans);
    tasksByPlan.set({ p1: fixtureTasks, p2: [] });
    selectedPlan.set(null);
    selectedTask.set(null);
    setInvokeHandler("get_recent_entries", () => []);
  }

  const { Story } = defineMeta({
    title: "PlanTracker/Views/ManualEntry",
    component: ManualEntry,
    tags: ["autodocs"],
    parameters: { layout: "fullscreen" },
  });
</script>

<!-- Plans loaded, form fields at their default (today's date, current time) -->
<Story
  name="EmptyForm"
  loaders={[
    () => {
      seedPlans();
    },
  ]}
/>

<!--
  Form with realistic pre-populated values.
  The play function selects the plan via the combobox and fills in time fields.
-->
<Story
  name="FormFilled"
  loaders={[
    () => {
      seedPlans();
      selectedPlan.set(fixturePlans[0]);
    },
  ]}
  play={async ({ canvasElement }) => {
    seedPlans();
    selectedPlan.set(fixturePlans[0]);

    const canvas = within(canvasElement);

    // Fill start date / time
    const dateInputs = canvas.getAllByDisplayValue(/\d{4}-\d{2}-\d{2}/);
    if (dateInputs[0]) {
      await userEvent.clear(dateInputs[0]);
      await userEvent.type(dateInputs[0], "2026-03-24");
    }
    const timeInputs = canvas.getAllByPlaceholderText("HH:MM");
    if (timeInputs[0]) {
      await userEvent.clear(timeInputs[0]);
      await userEvent.type(timeInputs[0], "09:00");
    }
    // Fill end date / time
    if (dateInputs[1]) {
      await userEvent.clear(dateInputs[1]);
      await userEvent.type(dateInputs[1], "2026-03-24");
    }
    if (timeInputs[1]) {
      await userEvent.clear(timeInputs[1]);
      await userEvent.type(timeInputs[1], "11:30");
    }
    // Fill notes
    const textarea = canvas.getByPlaceholderText("Add a note…");
    await userEvent.clear(textarea);
    await userEvent.type(
      textarea,
      "Implemented PKCE verifier and OAuth redirect.",
    );
  }}
/>

<!-- No plans synced yet — plan dropdown is empty -->
<Story
  name="NoPlans"
  play={async () => {
    plans.set([]);
    tasksByPlan.set({});
    selectedPlan.set(null);
    selectedTask.set(null);
    setInvokeHandler("get_recent_entries", () => []);
  }}
/>

<!--
  Save button in loading state.
  `create_manual_entry` never resolves so the spinner stays visible.
-->
<Story
  name="Saving"
  loaders={[
    () => {
      seedPlans();
      selectedPlan.set(fixturePlans[0]);
    },
  ]}
  play={async ({ canvasElement }) => {
    setInvokeHandler("create_manual_entry", () => new Promise(() => {}));

    const canvas = within(canvasElement);

    // Fill required time fields so validation passes
    const timeInputs = canvas.getAllByPlaceholderText("HH:MM");
    if (timeInputs[0]) {
      await userEvent.clear(timeInputs[0]);
      await userEvent.type(timeInputs[0], "09:00");
    }
    if (timeInputs[1]) {
      await userEvent.clear(timeInputs[1]);
      await userEvent.type(timeInputs[1], "11:30");
    }

    await userEvent.click(canvas.getByRole("button", { name: /save entry/i }));
  }}
/>
