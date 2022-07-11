use super::skills_struct::SkillsStruct;
use crate::util::constants::{
    CATACOMBS_XP, HOTM_XP, LEVELING_CAPS, LEVELING_XP, RUNECRAFTING_XP, SOCIAL_XP,
};
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

    pub fn get_purse_coins(&self) -> Option<f64> {
        self.get_player_float_property("coin_purse")
    }

    pub fn get_skill(&self, skill_name: &str) -> Option<SkillsStruct> {
        if let Some(skill_exp) = self.get_player_int_property(
            format!(
                "experience_skill_{}",
                if skill_name == "social" {
                    "social2"
                } else {
                    skill_name
                }
            )
            .as_str(),
        ) {
            return self.skill_exp_to_info(skill_name, skill_exp);
        }

        None
    }

    pub fn skill_exp_to_info(&self, skill_name: &str, skill_exp: i64) -> Option<SkillsStruct> {
        let leveling_table = match skill_name {
            "catacombs" => *CATACOMBS_XP,
            "runecrafting" => *RUNECRAFTING_XP,
            "social" => *SOCIAL_XP,
            "HOTM" => *HOTM_XP,
            _ => *LEVELING_XP,
        };
        let max_level = self.get_skill_max_level(skill_name);

        if skill_exp == 0 {
            return Some(SkillsStruct {
                name: skill_name.to_string(),
                current_level: 0,
                max_level,
                total_exp: 0,
                exp_current: 0,
                exp_for_next: 0,
                progress_to_next: 0.0,
            });
        }

        let mut xp_total = 0;
        let mut level = 1;
        for i in 0..max_level {
            let cur_xp_needed = leveling_table[i as usize];
            xp_total += cur_xp_needed;

            if xp_total > skill_exp {
                xp_total -= cur_xp_needed;
                break;
            } else {
                level = i + 1;
            }
        }

        let xp_current = skill_exp - xp_total;
        let xp_for_next = if level < max_level {
            leveling_table[level as usize]
        } else {
            0
        };

        let progress = if xp_for_next > 0 {
            (xp_current as f64 / xp_for_next as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        Some(SkillsStruct {
            name: skill_name.to_string(),
            current_level: level,
            max_level,
            total_exp: skill_exp,
            exp_current: xp_current,
            exp_for_next: xp_for_next,
            progress_to_next: progress,
        })
    }

    pub fn get_skill_max_level(&self, skill_name: &str) -> i64 {
        let max_level = LEVELING_CAPS.get(skill_name).unwrap_or(&50);

        if skill_name == "farming" {
            max_level + self.get_farming_cap_upgrade()
        } else {
            *max_level
        }
    }

    pub fn get_farming_cap_upgrade(&self) -> i64 {
        self.get_player_int_property("jacob2.perks.farming_level_cap")
            .unwrap_or(0)
    }

    pub fn get_hotm(&self) -> Option<SkillsStruct> {
        if let Some(xp) = self.get_player_int_property("mining_core.experience") {
            self.skill_exp_to_info("hotm", xp)
        } else {
            None
        }
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
