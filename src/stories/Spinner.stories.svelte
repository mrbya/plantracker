<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import Spinner from "$lib/components/ui/Spinner.svelte";

  const { Story } = defineMeta({
    title: "PlanTracker/UI/Spinner",
    component: Spinner,
    tags: ["autodocs"],
    argTypes: {
      size: {
        control: { type: "select" },
        options: ["sm", "md"],
      },
    },
  });
</script>

<script>
  import Button from "$lib/components/ui/Button.svelte";
</script>

<Story name="Medium" args={{ size: "md" }} />

<Story name="Small" args={{ size: "sm" }} />

<Story name="InsideButton">
  <Button variant="primary" loading={true}>Saving…</Button>
</Story>
