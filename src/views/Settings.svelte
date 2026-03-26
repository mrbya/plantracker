<script lang="ts">
  /**
   * Settings view — user preferences and account management.
   *
   * Organised into six sections:
   *
   * | Section    | Controls                                              |
   * |------------|-------------------------------------------------------|
   * | Entries    | Recent entries limit (number input, saves on blur)    |
   * | Sync       | Auto-sync frequency (select); manual "Sync Now" button|
   * | Storage    | Data directory path display; "Open Folder" shortcut   |
   * | Appearance | Theme choice (dark / light / system)                  |
   * | Language   | Language choice (en / sk / de)                        |
   * | Account    | Signed-in user name; "Sign Out" button                |
   *
   * Blur-vs-keystroke saving rationale:
   *   The entries-limit field saves via `onblur` (when the user leaves the
   *   field) rather than on every keystroke to avoid persisting intermediate
   *   invalid states (e.g., an empty field while the user is deleting and
   *   retyping).  If the blurred value is invalid the field resets to the
   *   current store value.  All other controls (selects) save immediately
   *   on change because they can only produce valid values.
   */
  import { onMount } from "svelte";
  import { openPath } from "@tauri-apps/plugin-opener";

  import { getDataDir, getAppVersion } from "$lib/api";
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
  import {
    saveLocale,
    LOCALE_LABELS,
    type AppLocale,
  } from "$lib/stores/locale";
  import { getLocale } from "$lib/paraglide/runtime";
  import { formatDateTime } from "$lib/utils/datetime";
  import * as m from "$lib/paraglide/messages";

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

  const THEME_OPTIONS = $derived([
    { value: "dark", label: m.settings_theme_dark() },
    { value: "light", label: m.settings_theme_light() },
    { value: "system", label: m.settings_theme_system() },
  ]);

  let themeValue = $state($themeChoice);

  async function onThemeChange() {
    await saveTheme(themeValue as ThemeChoice);
  }

  // ---------------------------------------------------------------------------
  // Language
  // ---------------------------------------------------------------------------

  const localeOptions = Object.entries(LOCALE_LABELS).map(([value, label]) => ({
    value,
    label,
  }));

  let localeValue = $state(getLocale() as AppLocale);

  async function onLocaleChange() {
    await saveLocale(localeValue);
  }

  // ---------------------------------------------------------------------------
  // Sync frequency
  // ---------------------------------------------------------------------------

  const SYNC_OPTIONS = $derived([
    { value: "manual", label: m.settings_sync_manual() },
    { value: "30min", label: m.settings_sync_30min() },
    { value: "1hour", label: m.settings_sync_1hour() },
  ]);

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
    if (!iso) return m.settings_sync_never();
    return m.settings_sync_last({ time: formatDateTime(iso) });
  }

  // ---------------------------------------------------------------------------
  // Data directory
  // ---------------------------------------------------------------------------

  let dataDir = $state("");
  let appVersion = $state<string | null>(null);

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
    try {
      appVersion = await getAppVersion();
    } catch {
      // Non-critical — footer simply won't render.
    }
  });
</script>

<div class="view">
  {#if appVersion}
    <header class="version-header">PlanTracker · v{appVersion}</header>
  {/if}

  <!-- Entries section -->
  <section class="settings-section">
    <h3 class="section-title">{m.settings_section_entries()}</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">{m.settings_entries_limit_label()}</span>
        <span class="setting-desc">{m.settings_entries_limit_desc()}</span>
      </div>
      <div class="setting-control narrow">
        <Input type="number" bind:value={limitInput} onblur={onLimitBlur} />
      </div>
    </div>
  </section>

  <!-- Sync section -->
  <section class="settings-section">
    <h3 class="section-title">{m.settings_section_sync()}</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">{m.settings_sync_frequency_label()}</span>
        <span class="setting-desc">{m.settings_sync_frequency_desc()}</span>
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
        <span class="setting-label">{m.settings_sync_now()}</span>
        <span class="setting-desc">{formatLastSynced($lastSyncedAt)}</span>
      </div>
      <div class="setting-control">
        <Button
          variant="primary"
          loading={syncing}
          disabled={syncing}
          onclick={handleSyncNow}
        >
          {m.settings_sync_now()}
        </Button>
      </div>
    </div>
  </section>

  <!-- Data directory section -->
  <section class="settings-section">
    <h3 class="section-title">{m.settings_section_storage()}</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">{m.settings_storage_label()}</span>
        <span class="setting-desc path-text">{dataDir || "—"}</span>
      </div>
      <div class="setting-control">
        <Button variant="ghost" disabled={!dataDir} onclick={handleOpenFolder}>
          {m.settings_open_folder()}
        </Button>
      </div>
    </div>
  </section>

  <!-- Appearance section -->
  <section class="settings-section">
    <h3 class="section-title">{m.settings_section_appearance()}</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">{m.settings_theme_label()}</span>
        <span class="setting-desc">{m.settings_theme_desc()}</span>
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

  <!-- Language section -->
  <section class="settings-section">
    <h3 class="section-title">{m.settings_section_language()}</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">{m.settings_language_label()}</span>
        <span class="setting-desc">{m.settings_language_desc()}</span>
      </div>
      <div class="setting-control">
        <Select
          options={localeOptions}
          bind:value={localeValue}
          onchange={onLocaleChange}
        />
      </div>
    </div>
  </section>

  <!-- Account section -->
  <section class="settings-section">
    <h3 class="section-title">{m.settings_section_account()}</h3>
    <div class="setting-row">
      <div class="setting-info">
        <span class="setting-label">{m.settings_account_signed_in_as()}</span>
        <span class="setting-desc">{$userDisplayName ?? "—"}</span>
      </div>
      <div class="setting-control">
        <Button variant="danger" onclick={logout}
          >{m.settings_sign_out()}</Button
        >
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

  .version-header {
    margin-bottom: 0.5rem;
    padding-bottom: 1rem;
    font-size: var(--font-size-sm);
    color: var(--text-muted);
    text-align: center;
    user-select: text;
  }
</style>
