use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GraphPlan {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphTask {
    pub id: String,
    pub title: String,
    pub plan_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphUser {
    pub id: String,
    pub display_name: String,
    pub user_principal_name: String,
}

#[derive(Debug, Deserialize)]
pub struct GraphPagedResponse<T> {
    pub value: Vec<T>,
    #[serde(rename = "@odata.nextLink")]
    pub next_link: Option<String>,
}
