import type { StorybookConfig } from '@storybook/sveltekit';

const config: StorybookConfig = {
  "stories": [
    '../docs/frontend-ui/**/*.mdx',
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
  "framework": "@storybook/sveltekit"
};
export default config;
