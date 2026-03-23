# Phase 1 — Theme & Design System

## 1.1 Define Catppuccin Mocha CSS variables

Create `src/lib/theme/mocha.css`:

```css
:root {
  /* Catppuccin Mocha palette */
  --ctp-base:    #1e1e2e;
  --ctp-mantle:  #181825;
  --ctp-crust:   #11111b;
  --ctp-surface0:#313244;
  --ctp-surface1:#45475a;
  --ctp-surface2:#585b70;
  --ctp-overlay0:#6c7086;
  --ctp-overlay1:#7f849c;
  --ctp-overlay2:#9399b2;
  --ctp-subtext0:#a6adc8;
  --ctp-subtext1:#bac2de;
  --ctp-text:    #cdd6f4;
  --ctp-lavender:#b4befe;
  --ctp-blue:    #89b4fa;
  --ctp-sapphire:#74c7ec;
  --ctp-sky:     #89dceb;
  --ctp-teal:    #94e2d5;
  --ctp-green:   #a6e3a1;
  --ctp-yellow:  #f9e2af;
  --ctp-peach:   #fab387;
  --ctp-maroon:  #eba0ac;
  --ctp-red:     #f38ba8;
  --ctp-mauve:   #cba6f7;
  --ctp-pink:    #f5c2e7;
  --ctp-flamingo:#f2cdcd;
  --ctp-rosewater:#f5e0dc;

  /* Semantic aliases */
  --bg:          var(--ctp-base);
  --bg-raised:   var(--ctp-mantle);
  --bg-input:    var(--ctp-surface0);
  --border:      var(--ctp-surface1);
  --text:        var(--ctp-text);
  --text-muted:  var(--ctp-subtext0);
  --accent:      var(--ctp-mauve);
  --accent-hover:var(--ctp-lavender);
  --success:     var(--ctp-green);
  --warning:     var(--ctp-yellow);
  --danger:      var(--ctp-red);
  --timer-active:var(--ctp-green);

  /* Typography */
  --font: 'JetBrainsMono Nerd Font', 'JetBrains Mono', monospace;
  --font-size-sm: 0.75rem;
  --font-size-base: 0.875rem;
  --font-size-lg: 1rem;
  --font-size-xl: 1.25rem;

  /* Spacing */
  --radius: 6px;
  --radius-lg: 10px;
  --gap: 0.75rem;
}
```

## 1.2 Global reset and base styles

Create `src/lib/theme/global.css`. Include:
- `* { box-sizing: border-box; margin: 0; padding: 0; }`
- `body { background: var(--bg); color: var(--text); font-family: var(--font); font-size: var(--font-size-base); }`
- Scrollbar styling (thin, Catppuccin-colored)
- Focus ring styles (accent color outline)
- Import both files in `src/app.css`

## 1.3 Base UI components

Create the following in `src/lib/components/ui/`:

| Component | Props | Notes |
|---|---|---|
| `Button.svelte` | `variant: 'primary'\|'ghost'\|'danger'`, `disabled`, `loading` | Accent-colored primary |
| `Select.svelte` | `options: {value, label}[]`, `value`, `placeholder` | Styled `<select>` wrapper |
| `Input.svelte` | `type`, `value`, `label`, `error` | Text/datetime input |
| `Card.svelte` | `title?` | Surface0 background, rounded |
| `Badge.svelte` | `color: 'green'\|'red'\|'yellow'` | Small status indicator |
| `Spinner.svelte` | `size: 'sm'\|'md'` | CSS-only loading animation |
| `EmptyState.svelte` | `message` | Centered muted text |

## 1.4 Layout shell

Create `src/lib/components/Layout.svelte`:
- Left sidebar (64px wide, icon-only navigation)
- Main content area (fills remaining space)
- Nav items: Time Tracking, Manual Entry, Reports, Settings (icons via Nerd Font glyphs)
- Active state highlighted with accent color
- User avatar / sign-out at the bottom of sidebar

## Verification checklist
- [x] App renders with dark Catppuccin Mocha background
- [x] All base components render correctly in isolation
- [x] Font is JetBrains Mono throughout
- [x] Layout shell navigates between placeholder views
