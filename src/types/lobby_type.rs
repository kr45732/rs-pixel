use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LobbyType(&'static str);

impl From<String> for LobbyType {
    fn from(v: String) -> Self {
        match v.as_str() {
            "MAIN" => LobbyType::MAIN,
            "TOURNAMENT" => LobbyType::TOURNAMENT,
            _ => LobbyType::UNKNOWN,
        }
    }
}

impl LobbyType {
    pub fn name(&self) -> String {
        self.0.to_string()
    }

    pub const MAIN: Self = Self("Main Lobby");
    pub const TOURNAMENT: Self = Self("Tournament Hall");
    pub const UNKNOWN: Self = Self("Unknown");
}
