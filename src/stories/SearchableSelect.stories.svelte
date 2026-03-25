<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { userEvent, within, expect } from "storybook/test";
  import SearchableSelect from "$lib/components/ui/SearchableSelect.svelte";

  const taskOptions = [
    { value: "t1", label: "Implement auth flow" },
    { value: "t2", label: "Write migration" },
    { value: "t3", label: "Graph API client" },
    { value: "t4", label: "Timer Rust commands" },
    { value: "t5", label: "Reports view" },
  ];

  const taskOptionsWithSentinel = [
    { value: "", label: "No specific task" },
    ...taskOptions,
  ];

  const manyTaskOptions = Array.from({ length: 22 }, (_, i) => ({
    value: `t${i + 1}`,
    label: `Task ${i + 1} — ${["Implement", "Write", "Fix", "Refactor", "Review", "Test", "Deploy", "Document"][i % 8]} feature ${i + 1}`,
  }));

  const { Story } = defineMeta({
    title: "PlanTracker/UI/SearchableSelect",
    component: SearchableSelect,
    tags: ["autodocs"],
    argTypes: {
      disabled: { control: "boolean" },
      placeholder: { control: "text" },
    },
  });
</script>

<Story
  name="Idle"
  args={{
    options: taskOptions,
    placeholder: "Select a task…",
    value: "",
    id: "idle",
  }}
/>

<Story
  name="PreSelected"
  args={{ options: taskOptions, value: "t3", id: "preselected" }}
/>

<Story
  name="WithSentinelOption"
  args={{
    options: taskOptionsWithSentinel,
    placeholder: "Select a task…",
    value: "",
    id: "sentinel",
  }}
/>

<Story
  name="Disabled"
  args={{
    options: taskOptions,
    placeholder: "Select a task…",
    value: "",
    disabled: true,
    id: "disabled",
  }}
/>

<Story
  name="ManyOptions"
  args={{
    options: manyTaskOptions,
    placeholder: "Select a task…",
    value: "",
    id: "many",
  }}
/>

<Story
  name="NoResults"
  args={{
    options: taskOptions,
    placeholder: "Select a task…",
    value: "",
    id: "noresults",
  }}
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    const input = canvas.getByRole("combobox");
    await userEvent.click(input);
    await userEvent.type(input, "xyzzy");
    await expect(
      canvas.getByRole("option", { name: "No results" }),
    ).toBeInTheDocument();
  }}
/>
