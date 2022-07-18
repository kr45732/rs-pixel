use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
