<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import ToastContainer from "$lib/components/ToastContainer.svelte";
  import { notifications } from "$lib/stores/notifications";

  const { Story } = defineMeta({
    title: "PlanTracker/Components/ToastContainer",
    component: ToastContainer,
    tags: ["autodocs"],
  });
</script>

<!--
  Each story seeds the notifications store directly via `play` (bypassing the
  4-second auto-dismiss timer) so the toasts remain visible for inspection.
-->

<Story
  name="SuccessToast"
  play={async () => {
    notifications.set([{ id: 1, type: "success", message: "Entry saved." }]);
  }}
/>

<Story
  name="ErrorToast"
  play={async () => {
    notifications.set([
      { id: 2, type: "error", message: "Sync failed: 401 Unauthorized" },
    ]);
  }}
/>

<Story
  name="WarningToast"
  play={async () => {
    notifications.set([
      {
        id: 3,
        type: "warning",
        message: "Timer was restored from a previous session.",
      },
    ]);
  }}
/>

<Story
  name="MultipleToasts"
  play={async () => {
    notifications.set([
      { id: 4, type: "success", message: "Entry saved." },
      { id: 5, type: "error", message: "Sync failed: network error." },
      { id: 6, type: "warning", message: "No plans found — run a sync first." },
    ]);
  }}
/>

<Story
  name="Empty"
  play={async () => {
    notifications.set([]);
  }}
/>
