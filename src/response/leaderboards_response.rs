use crate::types::game_type::GameType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaderboardsResponse {
    pub success: bool,
    pub leaderboards: HashMap<GameType, Vec<Leaderboard>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Leaderboard {
    pub path: String,
    pub prefix: String,
    pub count: i64,
    pub leaders: Vec<String>,
    pub title: String,
}
