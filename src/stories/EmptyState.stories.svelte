<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import EmptyState from "$lib/components/ui/EmptyState.svelte";

  const { Story } = defineMeta({
    title: "PlanTracker/UI/EmptyState",
    component: EmptyState,
    tags: ["autodocs"],
    argTypes: {
      message: { control: "text" },
    },
  });
</script>

<Story
  name="NoEntries"
  args={{ message: "No entries yet. Start a timer to track time." }}
/>

<Story name="NoTasks" args={{ message: "No tasks found for this plan." }} />

<Story name="NoResults" args={{ message: "No results match your filter." }} />
