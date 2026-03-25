<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { userEvent, within } from "storybook/test";
  import Layout from "$lib/components/Layout.svelte";

  /**
   * Stories for the main application shell.
   *
   * Note: `userDisplayName` is derived from a private `authStatus` writable in
   * the auth store that is only hydrated by `initAuth()` (called from
   * `+page.svelte`, not from `Layout` itself).  Because these stories mount
   * `Layout` in isolation, the avatar always shows "?" — the correct fallback
   * for an unknown user.  The full authenticated state is visible when running
   * the app normally.
   *
   * Each view component mounted inside `Layout` uses the `@tauri-apps/api/core`
   * mock (configured in `.storybook/main.ts`) so `invoke()` calls return safe
   * empty defaults without a running Tauri process.
   */
  const { Story } = defineMeta({
    title: "PlanTracker/Components/Layout",
    component: Layout,
    tags: ["autodocs"],
    parameters: {
      layout: "fullscreen",
    },
  });
</script>

<!-- Default render — Time Tracking view active, unknown user avatar -->
<Story name="TimeTrackingActive" />

<Story
  name="ManualEntryActive"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByTitle("Manual Entry"));
  }}
/>

<Story
  name="ReportsActive"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByTitle("Reports"));
  }}
/>

<Story
  name="SettingsActive"
  play={async ({ canvasElement }) => {
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByTitle("Settings"));
  }}
/>

<!-- Explicit story to document the unknown-user avatar fallback ("?") -->
<Story name="UnknownUser" />
