/// Authenticated HTTP client wrapper for Microsoft Graph.
///
/// The [`client::GraphClient`] struct wraps `reqwest::Client` with automatic bearer-token
/// injection via [`crate::auth::manager::AuthManager::get_valid_token`]. It handles
/// `401 Unauthorized` responses by performing a silent token refresh and retrying the
/// request once, and handles `429 Too Many Requests` by reading the `Retry-After` header
/// and sleeping before retrying.
pub mod client;

/// Deserialisation types for Microsoft Graph API responses.
///
/// Only the fields consumed by `PlanTracker` are modelled. Additional fields returned
/// by the Graph API are silently ignored by `serde`.
pub mod models;

/// Microsoft Graph Planner API helpers.
///
/// Provides high-level functions ([`planner::fetch_my_plans`],
/// [`planner::fetch_tasks_for_plan`], [`planner::fetch_user_info`]) that handle `OData`
/// pagination via `planner::fetch_all_pages`, returning fully collected `Vec` results
/// to the caller.
pub mod planner;
