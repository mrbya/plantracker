---
id: TASK-52
title: 'Testing: Frontend test infrastructure — Vitest setup'
status: Done
assignee: []
created_date: '2026-03-23 07:52'
updated_date: '2026-03-23 10:54'
labels:
  - testing
  - frontend
dependencies: []
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Install and configure Vitest for frontend unit tests. Covers section 10.7 of Phase 10.

**Install dev dependencies:**

```bash
pnpm add -D vitest @testing-library/svelte @testing-library/jest-dom jsdom
```

**Create `vitest.config.ts`** at the repo root:

```ts
import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
    plugins: [sveltekit()],
    test: {
        environment: 'jsdom',
        globals: true,
        setupFiles: ['src/tests/setup.ts'],
        include: ['src/**/*.test.ts'],
        coverage: {
            provider: 'v8',
            include: ['src/lib/**'],
            exclude: ['src/lib/theme/**'],
        },
    },
});
```

**Create `src/tests/setup.ts`:**

```ts
import { randomFillSync } from 'crypto';
import { clearMocks } from '@tauri-apps/api/mocks';
import '@testing-library/jest-dom';

// jsdom does not ship WebCrypto — polyfill it so @tauri-apps/api/mocks works
Object.defineProperty(window, 'crypto', {
    value: {
        getRandomValues: (buffer: BufferSource) =>
            randomFillSync(buffer as Buffer),
    },
    writable: true,
});

// Reset all Tauri IPC mocks between tests — prevents state leaking across files
afterEach(() => {
    clearMocks();
});
```

**Update `package.json` scripts:**

```json
"test": "vitest run",
"test:watch": "vitest",
"test:coverage": "vitest run --coverage"
```
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 vitest, @testing-library/svelte, @testing-library/jest-dom, jsdom installed as devDependencies
- [x] #2 vitest.config.ts exists at repo root with jsdom environment, globals: true, and setupFiles pointing to src/tests/setup.ts
- [x] #3 src/tests/setup.ts exists with WebCrypto polyfill and clearMocks afterEach hook
- [x] #4 package.json has test, test:watch, and test:coverage scripts
- [x] #5 pnpm test runs without error (zero test files found is acceptable at this stage)
- [x] #6 svelte-check passes with no errors
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Installed vitest 4.1.0, @testing-library/svelte 5.3.1, @testing-library/jest-dom 6.9.1, jsdom 29.0.1 as devDependencies. Created vitest.config.ts with jsdom environment, globals: true, passWithNoTests: true (so zero test files exit cleanly), and setupFiles pointing to src/tests/setup.ts. Created src/tests/setup.ts with WebCrypto polyfill and clearMocks afterEach hook. Added test/test:watch/test:coverage scripts to package.json. Added "vitest/globals" to tsconfig.json types so afterEach is recognized by svelte-check. pnpm test exits 0; svelte-check passes with 0 errors.
<!-- SECTION:FINAL_SUMMARY:END -->
