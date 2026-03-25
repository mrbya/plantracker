import path from "node:path";
import type { StorybookConfig } from '@storybook/sveltekit';

const config: StorybookConfig = {
  "stories": [
    '../.storybook/docs/**/*.mdx',
    "../src/**/*.mdx",
    "../src/**/*.stories.@(js|ts|svelte)"
  ],
  "addons": [
    "@storybook/addon-svelte-csf",
    "@chromatic-com/storybook",
    "@storybook/addon-vitest",
    "@storybook/addon-a11y",
    "@storybook/addon-docs"
  ],
  docs: {
    autodocs: 'tag'
  },
  staticDirs: ['../static'],
  "framework": "@storybook/sveltekit",
  async viteFinal(config) {
    // Replace Tauri native modules with lightweight in-browser mocks so that
    // stories render without a running Tauri process.
    const mocks = path.resolve(import.meta.dirname, '../src/stories/__mocks__');
    config.resolve ??= {};
    config.resolve.alias = {
      ...(config.resolve.alias ?? {}),
      '@tauri-apps/api/core': path.join(mocks, 'tauri-api-core.ts'),
      '@tauri-apps/plugin-opener': path.join(mocks, 'tauri-plugin-opener.ts'),
      '@tauri-apps/plugin-store': path.join(mocks, 'tauri-plugin-store.ts'),
    };
    return config;
  },
};
export default config;
