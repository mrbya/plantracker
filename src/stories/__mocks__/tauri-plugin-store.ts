/**
 * Storybook in-memory mock for `@tauri-apps/plugin-store`.
 *
 * Replaces the file-backed Tauri Store with a plain `Map` so settings and
 * theme stores initialise without trying to open a native file handle.
 * Data does not persist between story renders, which is the desired behaviour.
 */

class Store {
  private data = new Map<string, unknown>();

  static async load(_path: string): Promise<Store> {
    return new Store();
  }

  async get<T>(key: string): Promise<T | null> {
    return (this.data.get(key) as T) ?? null;
  }

  async set(key: string, value: unknown): Promise<void> {
    this.data.set(key, value);
  }

  async save(): Promise<void> {
    // no-op — nothing to flush in Storybook
  }
}

export { Store };
