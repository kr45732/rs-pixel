use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct LobbyType(Cow<'static, str>);

impl From<String> for LobbyType {
    fn from(v: String) -> Self {
        let v_str = v.as_str();
        match v_str {
            "MAIN" => LobbyType::MAIN,
            "TOURNAMENT" => LobbyType::TOURNAMENT,
            _ => LobbyType::UNKNOWN,
        }
    }
}

impl LobbyType {
    pub const fn new(name: &'static str) -> LobbyType {
        LobbyType(Cow::Borrowed(name))
    }

    pub fn name(&self) -> String {
        self.0.to_string()
    }

    pub const MAIN: LobbyType = LobbyType::new("Main Lobby");
    pub const TOURNAMENT: LobbyType = LobbyType::new("Tournament Hall");
    pub const UNKNOWN: LobbyType = LobbyType::new("Unknown");
}
