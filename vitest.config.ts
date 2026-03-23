import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
    plugins: [sveltekit()],
    test: {
        environment: 'jsdom',
        globals: true,
        setupFiles: ['src/tests/setup.ts'],
        include: ['src/**/*.test.ts'],
        passWithNoTests: true,
        coverage: {
            provider: 'v8',
            include: ['src/lib/**'],
            exclude: ['src/lib/theme/**'],
        },
    },
});
