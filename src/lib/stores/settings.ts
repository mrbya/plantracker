import { Store } from '@tauri-apps/plugin-store';
import { writable } from 'svelte/store';

export type SyncFrequency = 'manual' | '30min' | '1hour';

export const entriesLimit = writable<number>(20);
export const syncFrequency = writable<SyncFrequency>('manual');
export const lastSyncedAt = writable<string | null>(null);

let _store: Store | null = null;

async function getStore(): Promise<Store> {
  if (!_store) _store = await Store.load('config.json');
  return _store;
}

export async function loadSettings(): Promise<void> {
  const store = await getStore();
  entriesLimit.set((await store.get<number>('entriesLimit')) ?? 20);
  syncFrequency.set((await store.get<SyncFrequency>('syncFrequency')) ?? 'manual');
  lastSyncedAt.set((await store.get<string>('lastSyncedAt')) ?? null);
}

export async function saveEntriesLimit(value: number): Promise<void> {
  entriesLimit.set(value);
  const store = await getStore();
  await store.set('entriesLimit', value);
  await store.save();
}

export async function saveSyncFrequency(value: SyncFrequency): Promise<void> {
  syncFrequency.set(value);
  const store = await getStore();
  await store.set('syncFrequency', value);
  await store.save();
}

export async function saveLastSyncedAt(value: string): Promise<void> {
  lastSyncedAt.set(value);
  const store = await getStore();
  await store.set('lastSyncedAt', value);
  await store.save();
}
