use serde::{Deserialize, Serialize};

/// A Microsoft Planner plan cached in the local database.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    /// Local UUID primary key.
    pub id: String,
    /// Microsoft Graph ID (unique across tenants).
    pub graph_id: String,
    /// Display title of the plan.
    pub title: String,
    /// ISO 8601 timestamp of the last successful sync.
    pub synced_at: String,
}

/// A Microsoft Planner task cached in the local database.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    /// Local UUID primary key.
    pub id: String,
    /// Microsoft Graph ID (unique across tenants).
    pub graph_id: String,
    /// Local UUID of the parent plan.
    pub plan_id: String,
    /// Display title of the task.
    pub title: String,
    /// ISO 8601 timestamp of the last successful sync.
    pub synced_at: String,
}

/// A tracked time interval stored in the local database.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    /// Local UUID primary key.
    pub id: String,
    /// Local UUID of the parent plan.
    pub plan_id: String,
    /// Local UUID of the associated task, or `None` for plan-level entries.
    pub task_id: Option<String>,
    /// ISO 8601 start timestamp.
    pub start_time: String,
    /// ISO 8601 end timestamp, or `None` if the timer is still running.
    pub end_time: Option<String>,
    /// Optional free-text notes attached to this entry.
    pub notes: Option<String>,
    /// ISO 8601 creation timestamp.
    pub created_at: String,
}
