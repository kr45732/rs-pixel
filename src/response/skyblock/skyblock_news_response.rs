use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockNewsResponse {
    pub success: bool,
    pub items: Vec<SkyblockNews>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockNews {
    pub item: HashMap<String, Value>,
    pub link: String,
    pub text: String,
    pub title: String,
}
