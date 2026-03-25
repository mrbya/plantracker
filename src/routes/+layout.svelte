<script lang="ts">
  /**
   * SvelteKit root layout.
   *
   * Imports the global CSS bundle (`app.css`) which chains:
   *   1. `src/lib/theme/fonts.css`   — `@font-face` declarations for all 16
   *      JetBrains Mono weight/style variants bundled in `static/fonts/`
   *   2. `src/lib/theme/mocha.css`   — Catppuccin Mocha palette + semantic
   *      aliases + light-theme overrides under `.theme-light`
   *   3. `src/lib/theme/global.css`  — CSS reset, base element styles,
   *      focus rings, and scrollbar theming
   *
   * All child routes inherit these styles automatically.
   */
  import "../app.css";
  let { children } = $props();
</script>

{@render children()}
