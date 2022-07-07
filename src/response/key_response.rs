use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyResponse {
    pub success: bool,
    pub record: Record,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub key: String,
    pub owner: String,
    pub limit: i32,
    #[serde(rename = "queriesInPastMin")]
    pub queries_in_past_min: i32,
    #[serde(rename = "totalQueries")]
    pub total_queries: i32,
}
