use crate::types::game_type::GameType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecentGamesResponse {
    pub success: bool,
    pub games: Vec<GameSession>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSession {
    pub date: i64,
    #[serde(rename = "gameType")]
    pub game_type: GameType,
    pub mode: String,
    pub map: String,
    pub ended: i64,
}
