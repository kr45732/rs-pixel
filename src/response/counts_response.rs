use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CountsResponse {
    pub success: bool,
    pub games: HashMap<String, GameCount>,
    #[serde(rename = "playerCount")]
    pub player_count: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameCount {
    #[serde(default = "Default::default")]
    pub modes: HashMap<String, i64>,
    pub players: i64,
}
