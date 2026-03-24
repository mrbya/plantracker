use serde::Deserialize;

/// A Microsoft Planner plan returned by the Graph API.
#[derive(Debug, Deserialize)]
pub struct GraphPlan {
    /// Graph-assigned plan ID.
    pub id: String,
    /// Display title of the plan.
    pub title: String,
}

/// A Microsoft Planner task returned by the Graph API.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphTask {
    /// Graph-assigned task ID.
    pub id: String,
    /// Display title of the task.
    pub title: String,
    /// ID of the plan this task belongs to.
    pub plan_id: String,
}

/// Basic profile information for the signed-in user.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphUser {
    /// Graph-assigned user ID.
    pub id: String,
    /// Human-readable display name.
    pub display_name: String,
    /// User's principal name (email address).
    pub user_principal_name: String,
}

/// Wrapper for paginated Graph API responses.
#[derive(Debug, Deserialize)]
pub struct GraphPagedResponse<T> {
    /// Items returned on this page.
    pub value: Vec<T>,
    /// URL of the next page, or `None` if this is the last page.
    #[serde(rename = "@odata.nextLink")]
    pub next_link: Option<String>,
}
