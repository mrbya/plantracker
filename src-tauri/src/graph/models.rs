use serde::Deserialize;

/// A Microsoft Planner plan returned by the Graph API (`/me/planner/plans`).
///
/// Only the fields required by `PlanTracker` are modelled; the Graph API returns
/// additional metadata (e.g. `owner`, `createdBy`, `container`) that is discarded
/// by `serde` during deserialisation.
///
/// After a successful sync, the data in this struct is mapped to a local
/// [`crate::models::Plan`] and upserted into `SQLite` via
/// [`crate::db::plans::upsert_plan`].
#[derive(Debug, Deserialize)]
pub struct GraphPlan {
    /// Graph-assigned plan ID. Stored in `plans.graph_id` for future upsert conflict detection.
    pub id: String,
    /// Human-readable display title of the plan.
    pub title: String,
}

/// A Microsoft Planner task returned by the Graph API (`/planner/plans/{id}/tasks`).
///
/// Only the fields required by `PlanTracker` are modelled. The Graph API returns many
/// additional task attributes (e.g. `percentComplete`, `priority`, `assignments`) that
/// are ignored.
///
/// After a successful sync, the data in this struct is mapped to a local
/// [`crate::models::Task`] (with the Graph `planId` resolved to a local UUID) and
/// upserted into `SQLite` via [`crate::db::tasks::upsert_task`].
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphTask {
    /// Graph-assigned task ID. Stored in `tasks.graph_id` for future upsert conflict detection.
    pub id: String,
    /// Human-readable display title of the task.
    pub title: String,
    /// Graph ID of the plan this task belongs to.
    ///
    /// This is the raw Graph plan ID, not the local UUID. The sync command resolves
    /// the local plan UUID from this value via [`crate::db::plans::get_plan_by_graph_id`]
    /// before creating the local [`crate::models::Task`].
    pub plan_id: String,
}

/// Basic profile information for the signed-in Microsoft account.
///
/// Returned by `GET /me` and used to cache the user's display name in
/// [`crate::auth::manager::AuthManager::set_display_name`]. The cached name is then
/// exposed to the frontend through [`crate::commands::auth::AuthStatus`] so it can be
/// shown in the Settings view and the login screen.
///
/// Only three fields are modelled; the `/me` endpoint returns many more (e.g. `mail`,
/// `jobTitle`, `officeLocation`) that are not used by `PlanTracker`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphUser {
    /// Graph-assigned user object ID (a GUID).
    pub id: String,
    /// Human-readable display name (e.g. `"Alice Smith"`).
    pub display_name: String,
    /// User Principal Name, typically the user's email address.
    pub user_principal_name: String,
}

/// Generic wrapper for paginated Microsoft Graph list responses.
///
/// All Graph list endpoints may return results across multiple pages using `OData`
/// continuation links. When a response contains more items than fit on a single page,
/// the `@odata.nextLink` field holds the absolute URL of the next page. The
/// [`crate::graph::planner::fetch_all_pages`] helper follows these links until
/// `next_link` is `None`, accumulating all items into a single `Vec`.
///
/// Only the two fields required for pagination are modelled; other `OData` metadata
/// fields (`@odata.context`, `@odata.count`) are ignored.
#[derive(Debug, Deserialize)]
pub struct GraphPagedResponse<T> {
    /// Items returned on this page of results.
    pub value: Vec<T>,
    /// Absolute URL of the next page, or `None` if this is the last page.
    ///
    /// When `Some`, this URL is passed directly to [`crate::graph::client::GraphClient::get_url`]
    /// to fetch the next page without prepending the Graph base URL.
    #[serde(rename = "@odata.nextLink")]
    pub next_link: Option<String>,
}
