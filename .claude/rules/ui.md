# UI / UX Conventions

## Catppuccin Mocha Theme

All colours must be referenced as CSS custom properties. Never use hardcoded hex values.

### Full Palette (defined in `src/lib/theme/mocha.css`)

```css
/* Base surfaces */
--ctp-base:     #1e1e2e   /* app background */
--ctp-mantle:   #181825   /* slightly darker surface */
--ctp-crust:    #11111b   /* deepest background */
--ctp-surface0: #313244   /* raised surfaces, inputs */
--ctp-surface1: #45475a   /* borders */
--ctp-surface2: #585b70   /* subtle borders */

/* Text */
--ctp-overlay0: #6c7086   /* disabled / placeholder */
--ctp-overlay1: #7f849c
--ctp-overlay2: #9399b2
--ctp-subtext0: #a6adc8   /* muted labels */
--ctp-subtext1: #bac2de
--ctp-text:     #cdd6f4   /* primary text */

/* Accent colours */
--ctp-lavender: #b4befe
--ctp-blue:     #89b4fa
--ctp-sapphire: #74c7ec
--ctp-sky:      #89dceb
--ctp-teal:     #94e2d5
--ctp-green:    #a6e3a1   /* success, timer running */
--ctp-yellow:   #f9e2af   /* warning */
--ctp-peach:    #fab387
--ctp-maroon:   #eba0ac
--ctp-red:      #f38ba8   /* error, danger, stop */
--ctp-mauve:    #cba6f7   /* primary accent */
--ctp-pink:     #f5c2e7
--ctp-flamingo: #f2cdcd
--ctp-rosewater:#f5e0dc
```

### Semantic Aliases

```css
--bg:           var(--ctp-base)
--bg-raised:    var(--ctp-mantle)
--bg-input:     var(--ctp-surface0)
--border:       var(--ctp-surface1)
--text:         var(--ctp-text)
--text-muted:   var(--ctp-subtext0)
--accent:       var(--ctp-mauve)
--accent-hover: var(--ctp-lavender)
--success:      var(--ctp-green)
--warning:      var(--ctp-yellow)
--danger:       var(--ctp-red)
--timer-active: var(--ctp-green)
```

Always use semantic aliases in component styles. Use raw palette variables only in `mocha.css` to define the semantic aliases.

## Typography

Font family throughout the entire app: `'JetBrainsMono Nerd Font', 'JetBrains Mono', monospace`

```css
--font:           'JetBrainsMono Nerd Font', 'JetBrains Mono', monospace;
--font-size-sm:   0.75rem;
--font-size-base: 0.875rem;
--font-size-lg:   1rem;
--font-size-xl:   1.25rem;
```

## Focus Styles

Every interactive element must have a visible focus ring using the accent colour:
```css
:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
}
```

Never use `outline: none` without providing an alternative focus indicator.

## Loading States

Every async action must:
1. Disable the triggering control while pending
2. Show a `Spinner` inside the button or a skeleton loader in the content area
3. Re-enable the control and show result or error on completion

Never leave a button in a loading state permanently — always handle errors and resolve the pending state.

## Destructive Actions

Actions that delete data or sign the user out require a confirmation step before executing. Use a toast-based confirmation or a minimal inline confirm dialog — no native `window.confirm()`.

## Dropdowns: Plan / Task Relationship

When a **task** is selected anywhere in the app, the parent **plan** dropdown must update automatically to show that task's plan. This logic belongs in the `planner` store:

```typescript
export function selectTask(task: Task) {
    selectedTask.set(task);
    const plan = get(plans).find(p => p.id === task.planId) ?? null;
    selectedPlan.set(plan);
}
```

## Duration Formatting

| Context | Format | Example |
|---|---|---|
| UI display | `Xh Ym` | `2h 34m` |
| CSV export | `HH:MM:SS` | `2:34:00` |
| Tooltip / detailed | `Xh Ym Zs` | `2h 34m 12s` |

Utility functions live in `src/lib/utils/duration.ts`:
```typescript
export function formatDuration(seconds: number): string { ... }
export function formatDurationCSV(seconds: number): string { ... }
```

## Toast Notifications

Use toast notifications for all user feedback. Never use native `alert()`, `confirm()`, or `prompt()`.

Toasts are rendered by `ToastContainer.svelte` (fixed bottom-right). They auto-dismiss after 4 seconds. Colour coding:
- Green (`--success`): successful actions
- Red (`--danger`): errors
- Yellow (`--warning`): warnings

Trigger via the `notifications` store:
```typescript
import { addSuccess, addError } from '$lib/stores/notifications';
addSuccess('Entry saved');
addError('Sync failed: ' + message);
```

## Empty States

When a list or table has no items, show `EmptyState.svelte` with a contextual message rather than rendering nothing.

```svelte
{#if entries.length === 0}
    <EmptyState message="No entries yet. Start a timer to track time." />
{:else}
    <!-- table -->
{/if}
```

## No Inline Styles

No `style="..."` attributes on elements except for genuinely dynamic computed values (e.g., a progress bar width derived from a reactive variable). All other styles belong in `<style>` blocks or theme CSS files.

