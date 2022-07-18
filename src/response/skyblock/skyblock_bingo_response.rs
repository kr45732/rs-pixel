use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockBingoResponse {
    pub success: bool,
    pub events: Vec<BingoEvent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BingoEvent {
    pub key: i64,
    pub points: i64,
    pub completed_goals: Vec<String>,
}
