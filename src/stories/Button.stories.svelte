<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { fn } from "storybook/test";
  import Button from "$lib/components/ui/Button.svelte";

  const onClick = fn();

  const { Story } = defineMeta({
    title: "PlanTracker/UI/Button",
    component: Button,
    tags: ["autodocs"],
    argTypes: {
      variant: {
        control: { type: "select" },
        options: ["primary", "ghost", "danger", "success"],
      },
    },
  });
</script>

<Story name="Primary">
  <Button variant="primary" onclick={onClick}>Save entry</Button>
</Story>

<Story name="Ghost">
  <Button variant="ghost" onclick={onClick}>Cancel</Button>
</Story>

<Story name="Danger">
  <Button variant="danger" onclick={onClick}>Delete entry</Button>
</Story>

<Story name="Success">
  <Button variant="success" onclick={onClick}>Start timer</Button>
</Story>

<Story name="Loading">
  <Button variant="primary" loading={true}>Saving…</Button>
</Story>

<Story name="Disabled">
  <Button variant="primary" disabled={true}>Save entry</Button>
</Story>

<Story name="AllVariants">
  <div class="variants-row">
    <Button variant="primary" onclick={onClick}>Primary</Button>
    <Button variant="ghost" onclick={onClick}>Ghost</Button>
    <Button variant="danger" onclick={onClick}>Danger</Button>
    <Button variant="success" onclick={onClick}>Success</Button>
  </div>
</Story>

<style>
  .variants-row {
    display: flex;
    gap: 0.75rem;
    flex-wrap: wrap;
    align-items: center;
  }
</style>
