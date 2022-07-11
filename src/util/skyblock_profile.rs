use crate::{
    types::gamemode::Gamemode,
    util::generic_json::{Property, Raw},
};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockProfile {
    pub profile_id: String,
    pub members: Value,
    pub community_upgrades: Option<SkyblockCommunityUpgrades>,
    #[serde(default = "Default::default")]
    pub last_save: i64,
    pub cute_name: Option<String>,
    pub banking: Option<SkyblockBanking>,
    #[serde(
        deserialize_with = "deserialize_gamemode",
        default = "Default::default"
    )]
    pub game_mode: Gamemode,
    #[serde(skip_deserializing)]
    uuid: Option<String>,
}

fn deserialize_gamemode<'de, D>(deserializer: D) -> Result<Gamemode, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(match opt.unwrap_or("regular".to_string()).as_str() {
        "ironman" => Gamemode::Ironman,
        "island" => Gamemode::Stranded,
        "bingo" => Gamemode::Bingo,
        _ => Gamemode::Regular,
    })
}

impl Raw for SkyblockProfile {
    fn raw(&self) -> &Value {
        &self.members
    }
}

impl SkyblockProfile {
    pub fn set_uuid(&mut self, uuid: &str) {
        self.uuid = Some(uuid.to_string());
    }

    pub fn get_player_property(&self, full_path: &str) -> Option<&Value> {
        if let Some(uuid) = &self.uuid {
            self.get_property(format!("{}.{}", uuid, full_path).as_str())
        } else {
            None
        }
    }

    pub fn get_player_string_property(&self, full_path: &str) -> Option<&str> {
        self.get_player_property(full_path).and_then(|v| v.as_str())
    }

    pub fn get_player_int_property(&self, full_path: &str) -> Option<i64> {
        self.get_player_property(full_path)
            .and_then(|v| v.as_i64())
            .or(self
                .get_player_property(full_path)
                .and_then(|v| v.as_f64())
                .map(|v| v as i64))
    }

    pub fn get_player_float_property(&self, full_path: &str) -> Option<f64> {
        self.get_player_property(full_path)
            .and_then(|v| v.as_f64())
            .or(self
                .get_player_property(full_path)
                .and_then(|v| v.as_i64())
                .map(|v| v as f64))
    }

    pub fn get_player_array_property(&self, full_path: &str) -> Option<&Vec<Value>> {
        self.get_player_property(full_path)
            .and_then(|v| v.as_array())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockCommunityUpgrades {
    pub currently_upgrading: Option<SkyblockCurrentCommunityUpgrade>,
    pub upgrade_states: Vec<SkyblockCommunityUpgrade>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockCommunityUpgrade {
    pub upgrade: String,
    pub tier: i64,
    pub started_ms: i64,
    pub started_by: String,
    pub claimed_ms: i64,
    pub claimed_by: String,
    pub fasttracked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockCurrentCommunityUpgrade {
    pub upgrade: String,
    pub new_tier: i64,
    pub start_ms: i64,
    pub who_started: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockBanking {
    pub balance: f64,
    pub transactions: Vec<SkyblockTransaction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockTransaction {
    pub amount: f64,
    pub timestamp: i64,
    pub action: String,
    pub initiator_name: String,
}
