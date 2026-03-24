use crate::graph::{
    client::GraphClient,
    models::{GraphPagedResponse, GraphPlan, GraphTask, GraphUser},
};

/// Fetches all pages of a paginated Graph list endpoint.
/// Starts with the given path (e.g. `/me/planner/plans`) and follows
/// `@odata.nextLink` until no more pages remain.
///
/// # Errors
/// Returns an error if any page request fails or cannot be deserialised.
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

/// Returns all Planner plans visible to the signed-in user.
///
/// # Errors
/// Returns an error if the Graph request fails.
pub async fn fetch_my_plans(client: &GraphClient) -> anyhow::Result<Vec<GraphPlan>> {
    fetch_all_pages(client, "/me/planner/plans").await
}

/// Returns all tasks in the given plan.
///
/// # Errors
/// Returns an error if the Graph request fails.
pub async fn fetch_tasks_for_plan(
    client: &GraphClient,
    plan_id: &str,
) -> anyhow::Result<Vec<GraphTask>> {
    fetch_all_pages(client, &format!("/planner/plans/{plan_id}/tasks")).await
}

/// Returns the signed-in user's profile from `/me`.
///
/// # Errors
/// Returns an error if the Graph request fails.
pub async fn fetch_user_info(client: &GraphClient) -> anyhow::Result<GraphUser> {
    client.get("/me").await
}
