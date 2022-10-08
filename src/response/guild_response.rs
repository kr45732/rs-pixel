use crate::types::game_type::GameType;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildResponse {
    pub success: bool,
    pub guild: Option<Guild>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    #[serde(rename = "_id")]
    pub id: String,
    pub created: i64,
    pub exp: i64,
    #[serde(rename = "publiclyListed", default = "Default::default")]
    pub publicly_listed: bool,
    #[serde(default = "Default::default")]
    pub joinable: bool,
    pub name: String,
    #[serde(default = "Default::default")]
    pub description: String,
    pub tag: String,
    #[serde(rename = "tagColor")]
    pub tag_color: String,
    // pub banner: Banner,
    pub members: Vec<Member>,
    pub ranks: Vec<Rank>,
    #[serde(rename = "preferredGames")]
    pub preferred_games: Vec<GameType>,
    #[serde(rename = "guildExpByGameType")]
    pub guild_exp_by_game_type: HashMap<GameType, i64>,
    pub achievements: HashMap<GuildAchievement, i64>,
    pub coins: i64,
    #[serde(rename = "coinsEver")]
    pub coins_ever: i64,
    #[serde(rename = "legacyRanking")]
    pub legacy_ranking: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub uuid: String,
    pub rank: String,
    pub joined: i64,
    #[serde(rename = "expHistory")]
    pub exp_history: HashMap<String, i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rank {
    pub name: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub tag: String,
    pub priority: i64,
    pub default: bool,
    pub created: i64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuildAchievement {
    ExperienceKings,
    OnlinePlayers,
    Prestige,
    Winners,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
