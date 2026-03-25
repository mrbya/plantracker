use crate::graph::{
    client::GraphClient,
    models::{GraphPagedResponse, GraphPlan, GraphTask, GraphUser},
};

/// Fetches all items from a paginated Microsoft Graph list endpoint.
///
/// Graph list endpoints return results in pages. When more items are available than
/// fit on the current page, the response includes an `@odata.nextLink` field containing
/// the absolute URL of the next page. This function follows those links until no more
/// pages remain, accumulating all items into a single `Vec`.
///
/// The first page is fetched via [`GraphClient::get`] (which prepends the Graph base URL),
/// so `initial_path` must be a relative path starting with `/`
/// (e.g. `"/me/planner/plans"`). All subsequent pages are fetched via
/// [`GraphClient::get_url`] using the absolute `@odata.nextLink` URL directly.
///
/// # Arguments
///
/// - `client`: Reference to the authenticated [`GraphClient`].
/// - `initial_path`: The Graph API path for the first page (e.g. `"/me/planner/plans"`).
///
/// # Returns
///
/// `Ok(items)` — a `Vec<T>` containing all items from all pages, in the order returned
/// by the Graph API.
///
/// # Errors
///
/// Returns an error if any page request fails (network error, auth failure, non-2xx
/// response) or if any response body cannot be deserialised as `GraphPagedResponse<T>`.
async fn fetch_all_pages<T>(client: &GraphClient, initial_path: &str) -> anyhow::Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let mut results: Vec<T> = Vec::new();

    // First page — use path-based helper so the base URL is prepended.
    let first: GraphPagedResponse<T> = client.get(initial_path).await?;
    results.extend(first.value);
    let mut next = first.next_link;

    // Subsequent pages — use the absolute nextLink URL directly.
    while let Some(url) = next {
        let page: GraphPagedResponse<T> = client.get_url(&url).await?;
        results.extend(page.value);
        next = page.next_link;
    }

    Ok(results)
}

/// Returns all Microsoft Planner plans visible to the signed-in user.
///
/// Calls `GET /me/planner/plans` and follows all `@odata.nextLink` pages via
/// [`fetch_all_pages`]. The returned [`GraphPlan`] values are subsequently mapped to
/// local [`crate::models::Plan`] structs and upserted into `SQLite` by
/// [`crate::commands::sync::sync_plans_and_tasks`].
///
/// # Arguments
///
/// - `client`: Reference to the authenticated [`GraphClient`].
///
/// # Returns
///
/// `Ok(plans)` — all [`GraphPlan`] items accessible to the signed-in user, across
/// all pages.
///
/// # Errors
///
/// Returns an error if any Graph request fails or the response cannot be deserialised.
pub async fn fetch_my_plans(client: &GraphClient) -> anyhow::Result<Vec<GraphPlan>> {
    fetch_all_pages(client, "/me/planner/plans").await
}

/// Returns all tasks in a specific Microsoft Planner plan.
///
/// Calls `GET /planner/plans/{plan_id}/tasks` and follows all `@odata.nextLink` pages
/// via [`fetch_all_pages`]. The returned [`GraphTask`] values are subsequently mapped to
/// local [`crate::models::Task`] structs and upserted into `SQLite` by
/// [`crate::commands::sync::sync_plans_and_tasks`].
///
/// # Arguments
///
/// - `client`: Reference to the authenticated [`GraphClient`].
/// - `plan_id`: The **Graph** plan ID (not the local UUID) of the plan whose tasks to fetch.
///
/// # Returns
///
/// `Ok(tasks)` — all [`GraphTask`] items in the specified plan, across all pages.
///
/// # Errors
///
/// Returns an error if any Graph request fails or the response cannot be deserialised.
pub async fn fetch_tasks_for_plan(
    client: &GraphClient,
    plan_id: &str,
) -> anyhow::Result<Vec<GraphTask>> {
    fetch_all_pages(client, &format!("/planner/plans/{plan_id}/tasks")).await
}

/// Returns the signed-in user's profile from the Microsoft Graph `/me` endpoint.
///
/// Makes a single `GET /me` request (non-paginated). The returned [`GraphUser`] is
/// used to populate the display name cache in
/// [`crate::auth::manager::AuthManager::set_display_name`], which is then exposed
/// through [`crate::commands::auth::AuthStatus`] to the frontend.
///
/// # Arguments
///
/// - `client`: Reference to the authenticated [`GraphClient`].
///
/// # Returns
///
/// `Ok(user)` — the [`GraphUser`] profile of the currently signed-in Microsoft account.
///
/// # Errors
///
/// Returns an error if the Graph request fails or the response cannot be deserialised.
pub async fn fetch_user_info(client: &GraphClient) -> anyhow::Result<GraphUser> {
    client.get("/me").await
}
