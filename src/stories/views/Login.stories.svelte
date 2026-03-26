<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { userEvent, within } from "storybook/test";
  import { setInvokeHandler } from "../__mocks__/tauri-api-core";
  import Login from "../../views/Login.svelte";

  /**
   * Login view stories.
   *
   * Note: Storybook runs without a persisted locale, so Paraglide falls back to
   * the base locale (`"en"`).  The `getByRole` selector in the `SigningIn` play
   * function matches the English label ("Sign in with Microsoft").  If you add
   * locale-switching stories, call `setLocale()` from `$lib/paraglide/runtime`
   * before querying the DOM.
   */
  const { Story } = defineMeta({
    title: "PlanTracker/Views/Login",
    component: Login,
    tags: ["autodocs"],
    parameters: {
      layout: "fullscreen",
    },
  });
</script>

<!-- Default idle state — sign-in button enabled, no spinner -->
<Story name="Idle" />

<!--
  Simulates the loading state while the OAuth flow is in progress.
  The invoke mock is overridden to return a never-resolving promise so
  the button stays disabled with a spinner for the duration of the story.
-->
<Story
  name="SigningIn"
  play={async ({ canvasElement }) => {
    setInvokeHandler("login", () => new Promise(() => {}));
    const canvas = within(canvasElement);
    await userEvent.click(canvas.getByRole("button", { name: /sign in/i }));
  }}
/>
