# Phase 8 — Settings & Polish

## 8.1 Settings view

Create `src/views/Settings.svelte`:
- Recent entries limit: number input (default: 20)
- Sync frequency: dropdown (manual only / every 30 min / every hour)
- Data directory: read-only path display + "Open folder" button
- Account section: logged-in user name + avatar, Sign out button
- "Sync Now" button with last sync timestamp

Store settings via `tauri-plugin-store` in `config.json`.

## 8.2 App initialization sequence

In `App.svelte` `onMount`:
1. Check auth status → show Login or main app
2. If authenticated: load cached plans/tasks from SQLite
3. Trigger background sync (if due)
4. Check for active timer (in-progress entry with no `end_time`)
5. Load user settings

## 8.3 Error handling

Create `src/lib/stores/notifications.ts`:
- `notifications: Writable<Notification[]>`
- `addError(message: string)`, `addSuccess(message: string)`

Create `src/lib/components/ToastContainer.svelte`:
- Fixed bottom-right position
- Auto-dismiss after 4 seconds
- Color-coded: green (success), red (error), yellow (warning)

## 8.4 Loading states

Wrap each view's data fetching in:
- Skeleton loaders for tables (pulsing surface0 rectangles)
- `Spinner` inside buttons while async actions are pending
- Disable interactive controls during pending operations

## 8.5 Window configuration

In `src-tauri/tauri.conf.json`:
```json
{
  "app": {
    "windows": [{
      "title": "PlanTracker",
      "width": 1024,
      "height": 700,
      "minWidth": 800,
      "minHeight": 600,
      "decorations": true,
      "transparent": false
    }]
  }
}
```

## Verification checklist
- [x] Settings persist across app restarts
- [x] Toast notifications appear for errors and successes
- [x] Loading skeletons visible during data fetches
- [x] Window respects min dimensions
