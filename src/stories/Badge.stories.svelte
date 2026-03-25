<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import Badge from "$lib/components/ui/Badge.svelte";

  const { Story } = defineMeta({
    title: "PlanTracker/UI/Badge",
    component: Badge,
    tags: ["autodocs"],
    argTypes: {
      color: {
        control: { type: "select" },
        options: ["green", "red", "yellow"],
      },
    },
  });
</script>

<Story name="Running">
  <Badge color="green">Running</Badge>
</Story>

<Story name="Stopped">
  <Badge color="red">Stopped</Badge>
</Story>

<Story name="Warning">
  <Badge color="yellow">Warning</Badge>
</Story>

<Story name="AllColors">
  <div class="badges-row">
    <Badge color="green">Running</Badge>
    <Badge color="red">Stopped</Badge>
    <Badge color="yellow">Warning</Badge>
  </div>
</Story>

<style>
  .badges-row {
    display: flex;
    gap: 0.75rem;
    align-items: center;
  }
</style>
