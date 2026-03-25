use serde::{Deserialize, Serialize};

/// A Microsoft Planner plan cached in the local `SQLite` database.
///
/// Each `Plan` has two identifiers that serve different purposes:
/// - `id` is a locally generated UUID that acts as the `SQLite` primary key and is used
///   throughout the rest of the codebase for foreign-key references.
/// - `graph_id` is the opaque identifier assigned by the Microsoft Graph API. It is stored
///   in a `UNIQUE` column and acts as the conflict key for upserts: when a sync re-encounters
///   a plan that already exists locally, the `ON CONFLICT(graph_id)` clause updates `title`
///   and `synced_at` while leaving the local `id` (and therefore all FK relationships)
///   unchanged.
///
/// The struct is serialised with `camelCase` field names so it can be consumed directly
/// by the TypeScript frontend without a mapping layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    /// Local UUID primary key (generated with [`uuid::Uuid::new_v4`]).
    pub id: String,
    /// Microsoft Graph ID (unique across tenants). Used as the upsert conflict key.
    pub graph_id: String,
    /// Display title of the plan as returned by the Graph API.
    pub title: String,
    /// ISO 8601 timestamp of the last successful sync from Microsoft Graph.
    pub synced_at: String,
}

/// A Microsoft Planner task cached in the local `SQLite` database.
///
/// Like [`Plan`], tasks carry both a local UUID (`id`) and a Graph-assigned identifier
/// (`graph_id`). The `plan_id` field holds the **local** UUID of the parent plan — not the
/// Graph plan ID — so foreign-key integrity is maintained entirely within `SQLite` without
/// cross-referencing the Graph namespace.
///
/// The `tasks` table has `plan_id REFERENCES plans(id) ON DELETE CASCADE`, meaning if a
/// plan is deleted (e.g. it no longer appears in a sync result and is explicitly removed),
/// all its tasks are deleted automatically. Conversely, the `time_entries.task_id` column
/// has `ON DELETE SET NULL`, so time entries that referenced a deleted task are downgraded
/// to plan-level entries rather than being lost.
///
/// Serialised with `camelCase` field names for frontend consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// Local UUID primary key (generated with [`uuid::Uuid::new_v4`]).
    pub id: String,
    /// Microsoft Graph ID (unique across tenants). Used as the upsert conflict key.
    pub graph_id: String,
    /// Local UUID of the parent [`Plan`]. Never the Graph plan ID.
    pub plan_id: String,
    /// Display title of the task as returned by the Graph API.
    pub title: String,
    /// ISO 8601 timestamp of the last successful sync from Microsoft Graph.
    pub synced_at: String,
}

/// A tracked time interval stored in the local `SQLite` database.
///
/// A `TimeEntry` represents either a live timer that is still running or a completed
/// interval with a known duration. The distinction is carried by `end_time`:
/// - `end_time = None` (`NULL` in `SQLite`) means the timer is **currently active**.
/// - `end_time = Some(...)` means the interval is **completed**.
///
/// The application enforces an at-most-one-active invariant: before inserting a new entry
/// with `end_time = NULL`, the `start_timer` command checks `db::entries::find_active_entry`
/// and returns an error if one already exists. On startup, [`crate::run`] queries for any row
/// with `end_time IS NULL` and restores the in-memory [`crate::commands::timer::ActiveTimer`] state
/// from it, ensuring the elapsed time is computed correctly across restarts.
///
/// The `task_id` field is optional. A `None` value means the entry is tracked at the plan
/// level (no specific task was selected). If the associated task is later deleted from the
/// database, `SQLite`'s `ON DELETE SET NULL` constraint converts a task-level entry into a
/// plan-level entry automatically, preserving the recorded time.
///
/// Serialised with `camelCase` field names for frontend consumption.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    /// Local UUID primary key (generated with [`uuid::Uuid::new_v4`]).
    pub id: String,
    /// Local UUID of the parent [`Plan`].
    pub plan_id: String,
    /// Local UUID of the associated [`Task`], or `None` for plan-level entries.
    ///
    /// Set to `None` automatically by `SQLite` if the referenced task is deleted.
    pub task_id: Option<String>,
    /// ISO 8601 start timestamp (stored via [`.to_rfc3339()`][chrono::DateTime::to_rfc3339]).
    pub start_time: String,
    /// ISO 8601 end timestamp, or `None` if the timer is still running.
    ///
    /// At most one row in the `time_entries` table may have `end_time = NULL` at any time.
    pub end_time: Option<String>,
    /// Optional free-text notes attached to this entry by the user.
    pub notes: Option<String>,
    /// ISO 8601 timestamp of when this row was first created.
    pub created_at: String,
}
