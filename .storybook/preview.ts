import type { Preview } from "@storybook/sveltekit";
import "../src/lib/theme/fonts.css";
import "../src/lib/theme/mocha.css";
import "../src/lib/theme/global.css";

const preview: Preview = {
  parameters: {
    // Disable the Storybook backgrounds toolbar — our global.css sets
    // body { background: var(--bg) } via mocha.css / latte.css, so the
    // Catppuccin theme is always active without needing the backgrounds addon.
    backgrounds: { disable: true },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
    a11y: {
      // 'todo' - show a11y violations in the test UI only
      // 'error' - fail CI on a11y violations
      // 'off' - skip a11y checks entirely
      test: "todo",
    },
  },
};

export default preview;
