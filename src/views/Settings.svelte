<script lang="ts">
  import { onMount } from "svelte";
  import { openPath } from "@tauri-apps/plugin-opener";

  import { getDataDir } from "$lib/api";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import { logout, userDisplayName } from "$lib/stores/auth";
  import { addError } from "$lib/stores/notifications";
  import { syncAndLoad } from "$lib/stores/planner";
  import {
    entriesLimit,
    lastSyncedAt,
    saveEntriesLimit,
    saveLastSyncedAt,
    saveSyncFrequency,
    syncFrequency,
    type SyncFrequency,
  } from "$lib/stores/settings";
  import { themeChoice, saveTheme, type ThemeChoice } from "$lib/stores/theme";
  import { formatDateTime } from "$lib/utils/datetime";

  // ---------------------------------------------------------------------------
  // Entries limit
  // ---------------------------------------------------------------------------

  let limitInput = $state(String($entriesLimit));

  async function onLimitBlur() {
    const parsed = parseInt(limitInput, 10);
    if (!isNaN(parsed) && parsed > 0) {
      await saveEntriesLimit(parsed);
    } else {
      limitInput = String($entriesLimit); // reset on invalid input
    }
  }

  // ---------------------------------------------------------------------------
  // Theme
  // ---------------------------------------------------------------------------

  const THEME_OPTIONS = [
    { value: "dark", label: "Dark" },
    { value: "light", label: "Light" },
    { value: "system", label: "System Default" },
  ];

  let themeValue = $state($themeChoice);

  async function onThemeChange() {
    await saveTheme(themeValue as ThemeChoice);
  }

  // ---------------------------------------------------------------------------
  // Sync frequency
  // ---------------------------------------------------------------------------

  const SYNC_OPTIONS = [
    { value: "manual", label: "Manual only" },
    { value: "30min", label: "Every 30 minutes" },
    { value: "1hour", label: "Every hour" },
  ];

  let syncFreqValue = $state($syncFrequency);

  async function onSyncFreqChange() {
    await saveSyncFrequency(syncFreqValue as SyncFrequency);
  }

  // ---------------------------------------------------------------------------
  // Sync now
  // ---------------------------------------------------------------------------

  let syncing = $state(false);

  async function handleSyncNow() {
    syncing = true;
    try {
      await syncAndLoad();
      await saveLastSyncedAt(new Date().toISOString());
    } finally {
      syncing = false;
    }
  }

  function formatLastSynced(iso: string | null): string {
    if (!iso) return "Never";
    return formatDateTime(iso);
  }

  // ---------------------------------------------------------------------------
  // Data directory
  // ---------------------------------------------------------------------------

  let dataDir = $state("");

  async function handleOpenFolder() {
    if (dataDir) {
      try {
        await openPath(dataDir);
      } catch (e) {
        addError("Could not open folder: " + String(e));
      }
    }
  }

  // ---------------------------------------------------------------------------
  // Lifecycle
  // ---------------------------------------------------------------------------

  onMount(async () => {
    try {
      dataDir = await getDataDir();
    } catch (e) {
      addError("Could not get data directory: " + String(e));
    }
  });
</script>

<div class="view">
  <!-- Entries section -->
  <section class="settings-section">
    <h3 class="section-title">Entries</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">Recent Entries Limit</span>
        <span class="setting-desc"
          >Number of entries shown in Time Tracking and Manual Entry</span
        >
      </div>
      <div class="setting-control narrow">
        <Input type="number" bind:value={limitInput} onblur={onLimitBlur} />
      </div>
    </div>
  </section>

  <!-- Sync section -->
  <section class="settings-section">
    <h3 class="section-title">Sync</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">Sync Frequency</span>
        <span class="setting-desc"
          >How often to automatically sync plans and tasks from Microsoft
          Planner</span
        >
      </div>
      <div class="setting-control">
        <Select
          options={SYNC_OPTIONS}
          bind:value={syncFreqValue}
          onchange={onSyncFreqChange}
        />
      </div>
    </div>

    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">Manual Sync</span>
        <span class="setting-desc"
          >Last synced: {formatLastSynced($lastSyncedAt)}</span
        >
      </div>
      <div class="setting-control">
        <Button
          variant="primary"
          loading={syncing}
          disabled={syncing}
          onclick={handleSyncNow}
        >
          Sync Now
        </Button>
      </div>
    </div>
  </section>

  <!-- Data directory section -->
  <section class="settings-section">
    <h3 class="section-title">Storage</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">Data Directory</span>
        <span class="setting-desc path-text">{dataDir || "—"}</span>
      </div>
      <div class="setting-control">
        <Button variant="ghost" disabled={!dataDir} onclick={handleOpenFolder}>
          Open Folder
        </Button>
      </div>
    </div>
  </section>

  <!-- Appearance section -->
  <section class="settings-section">
    <h3 class="section-title">Appearance</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">Theme</span>
        <span class="setting-desc"
          >Controls the colour scheme of the application.</span
        >
      </div>
      <div class="setting-control">
        <Select
          options={THEME_OPTIONS}
          bind:value={themeValue}
          onchange={onThemeChange}
        />
      </div>
    </div>
  </section>

  <!-- Account section -->
  <section class="settings-section">
    <h3 class="section-title">Account</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">Signed in as</span>
        <span class="setting-desc">{$userDisplayName ?? "—"}</span>
      </div>
      <div class="setting-control">
        <Button variant="danger" onclick={logout}>Sign Out</Button>
      </div>
    </div>
  </section>
</div>

<style>
  .view {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0;
    max-width: 720px;
  }

  .settings-section {
    border-top: 1px solid var(--border);
    padding: 1.25rem 0;
  }

  .settings-section:first-child {
    border-top: none;
    padding-top: 0;
  }

  .section-title {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    margin-bottom: 1rem;
  }

  .setting-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1.5rem;
    padding: 0.5rem 0;
  }

  .setting-row + .setting-row {
    border-top: 1px solid color-mix(in srgb, var(--border) 50%, transparent);
    padding-top: 0.75rem;
    margin-top: 0.25rem;
  }

  .setting-info {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    min-width: 0;
  }

  .setting-label {
    font-size: var(--font-size-base);
    color: var(--text);
  }

  .setting-desc {
    font-size: var(--font-size-sm);
    color: var(--text-muted);
  }

  .path-text {
    word-break: break-all;
  }

  .setting-control {
    flex-shrink: 0;
    min-width: 160px;
  }

  .setting-control.narrow {
    min-width: 80px;
    max-width: 80px;
  }

  /* Remove the inner .field wrapper margin from Input */
  .setting-control :global(.field) {
    gap: 0;
  }
</style>
