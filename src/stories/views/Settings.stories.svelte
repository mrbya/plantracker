<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { userEvent, within } from "storybook/test";
  import { setInvokeHandler } from "../__mocks__/tauri-api-core";
  import Settings from "../../views/Settings.svelte";
  import { entriesLimit } from "$lib/stores/settings";

  /**
   * Settings view stories.
   *
   * Notes:
   * - `getDataDir` is mocked via the global `@tauri-apps/api/core` mock to
   *   return a fake path so the Storage section renders a path string.
   * - `@tauri-apps/plugin-store` is mocked globally, so settings load/save
   *   use an in-memory map instead of a real config file.
   * - `userDisplayName` comes from the private `authStatus` in the auth store
   *   which is not hydrated in these isolated stories — the Account section
   *   shows "—" as the fallback.
   */
  const { Story } = defineMeta({
    title: "PlanTracker/Views/Settings",
    component: Settings,
    tags: ["autodocs"],
    parameters: { layout: "fullscreen" },
  });
</script>

<!-- Default: all controls enabled, settings at their in-memory defaults -->
<Story name="Default" />

<!--
  "Sync Now" button in loading state.
  `sync_plans_and_tasks` never resolves so the spinner stays visible.
-->
<Story
  name="SyncInProgress"
  play={async ({ canvasElement }) => {
    setInvokeHandler("sync_plans_and_tasks", () => new Promise(() => {}));
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: /sync now/i }));
  }}
/>

<!--
  Inline validation error on the Entries Limit field.
  The play function clears the field and types a non-numeric value, then
  blurs it to trigger the `onblur` validation handler in the Settings view.
  The field resets to the current store value on invalid input, so the
  validation error shown here is the transient "reset" visual state.
  We demonstrate it by typing a clearly invalid string before blur.
-->
<Story
  name="ValidationError"
  play={async ({ canvasElement }) => {
    entriesLimit.set(20);
    const canvas = within(canvasElement);
    const limitInput = canvas.getByDisplayValue("20");
    await userEvent.clear(limitInput);
    await userEvent.type(limitInput, "-5");
    // blur triggers the validation; -5 fails (must be > 0) so field resets
    await userEvent.tab();
  }}
/>
