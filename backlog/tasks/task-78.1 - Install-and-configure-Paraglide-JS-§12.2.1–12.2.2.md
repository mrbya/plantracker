---
id: TASK-78.1
title: Install and configure Paraglide JS (§12.2.1–12.2.2)
status: Done
assignee: []
created_date: '2026-03-26 07:14'
updated_date: '2026-03-26 07:23'
labels:
  - frontend
  - i18n
  - phase-12
dependencies: []
parent_task_id: TASK-78
priority: medium
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Install `@inlang/paraglide-js` and wire the Vite plugin so Paraglide compiles message files during the build.

## Steps

### Install

```bash
pnpm add @inlang/paraglide-js
pnpx @inlang/paraglide-js@latest init
```

When prompted:
- Languages: `en`, `sk`, `de`
- Framework: SvelteKit (Vite plugin)
- Output directory: `./src/lib/paraglide` (default)

This creates:
- `project.inlang/` — inlang project config
- `messages/en.json`, `messages/sk.json`, `messages/de.json` — translation files
- Adds the compile step to `package.json` build scripts

### Wire `vite.config.ts`

```typescript
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { paraglideVitePlugin } from '@inlang/paraglide-js';

export default defineConfig({
    plugins: [
        paraglideVitePlugin({
            project: './project.inlang',
            outdir:  './src/lib/paraglide',
            strategy: ['cookie', 'baseLocale'],
        }),
        sveltekit(),
    ],
});
```

`paraglideVitePlugin` **must** come before `sveltekit()`.

## Reference

`docs/phases/phase-12-quality-of-life.md` §12.2.1 and §12.2.2
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 pnpm add @inlang/paraglide-js succeeds and package is in package.json
- [x] #2 project.inlang/ config directory exists with en/sk/de languages configured
- [x] #3 vite.config.ts imports and registers paraglideVitePlugin before sveltekit()
- [x] #4 strategy is ['cookie', 'baseLocale'] and outdir is ./src/lib/paraglide
- [x] #5 Running pnpm dev starts without Vite plugin errors
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Manually bootstrapped Paraglide JS 2.15.1 without running the interactive init CLI.

**Changes made:**
- `package.json` — added `@inlang/paraglide-js 2.15.1` to dependencies (via `pnpm add`)
- `project.inlang/settings.json` — created inlang project config with `baseLocale: "en"`, `locales: ["en", "sk", "de"]`, `plugin-message-format@4` + `plugin-m-function-matcher@2` from CDN, `pathPattern: "./messages/{locale}.json"`
- `messages/en.json`, `messages/sk.json`, `messages/de.json` — created stub files with `$schema` only (content to be filled in TASK-78.2)
- `vite.config.js` — added `paraglideVitePlugin` import and registered it before `sveltekit()` with `project: "./project.inlang"`, `outdir: "./src/lib/paraglide"`, `strategy: ["cookie", "baseLocale"]`

**Verified:** `pnpm exec vite build` succeeds with `✔ [paraglide-js] Compilation complete` and generates `src/lib/paraglide/` (messages.js, runtime.js, server.js, registry.js).
<!-- SECTION:FINAL_SUMMARY:END -->
