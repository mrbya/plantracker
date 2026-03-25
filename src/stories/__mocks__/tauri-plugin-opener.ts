/** Storybook no-op mock for `@tauri-apps/plugin-opener`. */

export async function openPath(_path: string): Promise<void> {
  // no-op in Storybook — the OS file manager cannot be opened in a browser
}

export async function openUrl(_url: string): Promise<void> {
  // no-op in Storybook
}
