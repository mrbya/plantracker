<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import Input from "$lib/components/ui/Input.svelte";

  const { Story } = defineMeta({
    title: "PlanTracker/UI/Input",
    component: Input,
    tags: ["autodocs"],
    argTypes: {
      type: { control: "text" },
      label: { control: "text" },
      error: { control: "text" },
    },
  });
</script>

<Story name="Default" args={{ label: "Notes", type: "text" }} />

<Story
  name="WithError"
  args={{
    label: "Duration",
    type: "number",
    error: "Must be a positive number",
  }}
/>

<Story name="NumberType" args={{ type: "number", label: "Entries limit" }} />

<Story
  name="DateTimeLocal"
  args={{ type: "datetime-local", label: "Start time" }}
/>

<Story name="NoLabel" args={{ type: "text" }} />
