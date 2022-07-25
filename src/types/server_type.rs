use super::{game_type::GameType, lobby_type::LobbyType};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

#[derive(Serialize, Debug)]
pub enum ServerType {
    GameType(GameType),
    LobbyType(LobbyType),
    Unknown,
}

impl ServerType {
    pub fn get_game_type(&self) -> Option<&GameType> {
        if let ServerType::GameType(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_lobby_type(&self) -> Option<&LobbyType> {
        if let ServerType::LobbyType(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

impl<'de> Deserialize<'de> for ServerType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServerTypeVisitor;

        impl<'de> Visitor<'de> for ServerTypeVisitor {
            type Value = ServerType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a String")
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Ok(game_type) = GameType::try_from(value.clone()) {
                    return Ok(ServerType::GameType(game_type));
                }

                if let Ok(lobby_type) = LobbyType::try_from(value) {
                    return Ok(ServerType::LobbyType(lobby_type));
                }

                Ok(ServerType::Unknown)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Ok(game_type) = GameType::try_from(value.to_string()) {
                    return Ok(ServerType::GameType(game_type));
                }

                if let Ok(lobby_type) = LobbyType::try_from(value.to_string()) {
                    return Ok(ServerType::LobbyType(lobby_type));
                }

                Ok(ServerType::Unknown)
            }
        }

        deserializer.deserialize_any(ServerTypeVisitor)
    }
}
