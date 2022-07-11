use crate::types::server_type::ServerType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    pub success: bool,
    pub session: Session,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub online: bool,
    #[serde(rename = "gameType")]
    pub game_type: Option<ServerType>,
    pub mode: Option<String>,
    pub map: Option<String>,
}
