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
   *
   * i18n: Nav button `title` attributes are generated from Paraglide message
   * functions (e.g. `m.nav_manual_entry()`).  Storybook runs without a persisted
   * locale, so Paraglide falls back to the base locale (`"en"`).  The
   * `getByTitle` selectors in the play functions below match the English labels.
   * If you add a locale-switching story, call `setLocale()` inside the play
   * function before querying the DOM.
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
