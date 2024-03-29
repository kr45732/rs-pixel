use super::utils::parse_nbt;
use crate::util::constants::{
    BLAZE_EXP, CATACOMBS_EXP, CRAFTED_MINIONS_TO_SLOTS, ENDERMAN_EXP, HOTM_EXP, LEVELING_CAPS,
    LEVELING_EXP, PET_EXP, PET_RARITY_OFFSET, RUNECRAFTING_EXP, SOCIAL_EXP, SPIDER_EXP, WOLF_EXP,
    ZOMBIE_EXP,
};
use crate::{
    types::gamemode::Gamemode,
    util::generic_json::{Property, Raw},
};

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockProfile {
    pub profile_id: String,
    pub members: Value,
    pub community_upgrades: Option<SkyblockCommunityUpgrades>,
    #[serde(default = "Default::default")]
    pub selected: bool,
    pub cute_name: Option<String>,
    pub banking: Option<SkyblockBanking>,
    #[serde(
        deserialize_with = "deserialize_gamemode",
        default = "Default::default"
    )]
    pub game_mode: Gamemode,
}

fn deserialize_gamemode<'de, D>(deserializer: D) -> Result<Gamemode, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(match opt.unwrap_or_default().as_str() {
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
    pub fn get_purse_coins(&self, uuid: &str) -> Option<f64> {
        self.get_float_property(&format!("{uuid}.coin_purse"))
    }

    pub fn get_skill(&self, uuid: &str, skill_name: &str) -> Option<LevelingStruct> {
        self.get_int_property(&format!(
            "{}.experience_skill_{}",
            uuid,
            if skill_name == "social" {
                "social2"
            } else {
                skill_name
            }
        ))
        .map(|skill_exp| self.skill_exp_to_info(uuid, skill_name, skill_exp))
    }

    fn skill_exp_to_info(&self, uuid: &str, skill_name: &str, skill_exp: i64) -> LevelingStruct {
        let leveling_table = match skill_name {
            "catacombs" => *CATACOMBS_EXP,
            "runecrafting" => *RUNECRAFTING_EXP,
            "social" => *SOCIAL_EXP,
            "hotm" => *HOTM_EXP,
            _ => *LEVELING_EXP,
        };
        let max_level = self.get_max_level(uuid, skill_name);

        if skill_exp == 0 {
            return LevelingStruct {
                name: skill_name.to_string(),
                level: 0,
                max_level,
                total_exp: 0,
                current_exp: 0,
                exp_for_next: 0,
                progress_to_next: 0.0,
            };
        }

        let mut exp_total = 0;
        let mut level = 0;
        for i in 0..max_level {
            let cur_exp_needed = leveling_table[i as usize];
            exp_total += cur_exp_needed;

            if exp_total > skill_exp {
                exp_total -= cur_exp_needed;
                break;
            }

            level = i + 1;
        }

        let exp_current = skill_exp - exp_total;
        let exp_for_next = if level < max_level {
            leveling_table[level as usize]
        } else {
            0
        };

        let progress = if exp_for_next > 0 {
            (exp_current as f64 / exp_for_next as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        LevelingStruct {
            name: skill_name.to_string(),
            level,
            max_level,
            total_exp: skill_exp,
            current_exp: exp_current,
            exp_for_next,
            progress_to_next: progress,
        }
    }

    pub fn get_max_level(&self, uuid: &str, name: &str) -> i64 {
        LEVELING_CAPS.get(name).unwrap_or(&50)
            + if name == "farming" {
                self.get_farming_cap_upgrade(uuid)
            } else {
                0
            }
    }

    pub fn get_farming_cap_upgrade(&self, uuid: &str) -> i64 {
        self.get_int_property(&format!("{uuid}.jacob2.perks.farming_level_cap"))
            .unwrap_or(0)
    }

    pub fn get_hotm(&self, uuid: &str) -> Option<LevelingStruct> {
        self.get_int_property(&format!("{uuid}.mining_core.experience"))
            .map(|exp| self.skill_exp_to_info(uuid, "hotm", exp))
    }

    fn slayer_exp_to_info(&self, uuid: &str, slayer_name: &str, slayer_exp: i64) -> LevelingStruct {
        let leveling_table = match slayer_name {
            "zombie" => *ZOMBIE_EXP,
            "wolf" => *WOLF_EXP,
            "spider" => *SPIDER_EXP,
            "enderman" => *ENDERMAN_EXP,
            _ => *BLAZE_EXP,
        };
        let max_level = self.get_max_level(uuid, slayer_name);

        if slayer_exp == 0 {
            return LevelingStruct {
                name: slayer_name.to_string(),
                level: 0,
                max_level,
                total_exp: 0,
                current_exp: 0,
                exp_for_next: 0,
                progress_to_next: 0.0,
            };
        }

        let mut level = 0;
        for i in 0..max_level {
            if leveling_table[i as usize] > slayer_exp {
                break;
            }

            level = i + 1;
        }

        let exp_prev = if level > 0 {
            leveling_table[(level - 1) as usize]
        } else {
            0
        };
        let exp_current = slayer_exp - exp_prev;
        let exp_for_next = if level < max_level {
            leveling_table[level as usize] - exp_prev
        } else {
            0
        };

        let progress = if exp_for_next > 0 {
            (exp_current as f64 / exp_for_next as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        LevelingStruct {
            name: slayer_name.to_string(),
            level,
            max_level,
            total_exp: slayer_exp,
            current_exp: exp_current,
            exp_for_next,
            progress_to_next: progress,
        }
    }

    pub fn get_slayer(&self, uuid: &str, slayer_name: &str) -> Option<LevelingStruct> {
        self.get_int_property(&format!("{uuid}.slayer_bosses.{slayer_name}.xp"))
            .map(|exp| self.slayer_exp_to_info(uuid, slayer_name, exp))
    }

    pub fn get_catacombs(&self, uuid: &str) -> Option<LevelingStruct> {
        self.get_int_property(&format!(
            "{uuid}.dungeons.dungeon_types.catacombs.experience"
        ))
        .map(|exp| self.skill_exp_to_info(uuid, "catacombs", exp))
    }

    pub fn get_dungeon_class(&self, uuid: &str, class_name: &str) -> Option<LevelingStruct> {
        self.get_int_property(&format!(
            "{uuid}.dungeons.player_classes.{class_name}.experience"
        ))
        .map(|exp| self.skill_exp_to_info(uuid, "catacombs", exp))
    }

    pub fn get_inventory(&self, uuid: &str) -> Option<Value> {
        if let Some(data) = self.get_str_property(&format!("{uuid}.inv_contents.data")) {
            parse_nbt(data)
        } else {
            None
        }
    }

    pub fn get_personal_vault(&self, uuid: &str) -> Option<Value> {
        if let Some(data) = self.get_str_property(&format!("{uuid}.personal_vault_contents.data")) {
            parse_nbt(data)
        } else {
            None
        }
    }

    pub fn get_talisman_bag(&self, uuid: &str) -> Option<Value> {
        if let Some(data) = self.get_str_property(&format!("{uuid}.talisman_bag.data")) {
            parse_nbt(data)
        } else {
            None
        }
    }

    pub fn get_equippment(&self, uuid: &str) -> Option<Value> {
        if let Some(data) = self.get_str_property(&format!("{uuid}.equippment_contents.data")) {
            parse_nbt(data)
        } else {
            None
        }
    }

    pub fn get_armor(&self, uuid: &str) -> Option<Value> {
        if let Some(data) = self.get_str_property(&format!("{uuid}.inv_armor.data")) {
            parse_nbt(data)
        } else {
            None
        }
    }

    pub fn get_wardrobe(&self, uuid: &str) -> Option<Value> {
        if let Some(data) = self.get_str_property(&format!("{uuid}.wardrobe_contents.data")) {
            parse_nbt(data)
        } else {
            None
        }
    }

    pub fn get_ender_chest(&self, uuid: &str) -> Option<Value> {
        if let Some(data) = self.get_str_property(&format!("{uuid}.ender_chest_contents.data")) {
            parse_nbt(data)
        } else {
            None
        }
    }

    pub fn get_storage(&self, uuid: &str) -> Option<HashMap<&str, Value>> {
        if let Some(data) = self.get_object_property(&format!("{uuid}.backpack_contents")) {
            let mut storage = HashMap::new();
            for ele in data {
                if let Some(bp) = parse_nbt(
                    ele.1
                        .get("data")
                        .and_then(serde_json::Value::as_str)
                        .unwrap(),
                ) {
                    storage.insert(&**ele.0, bp);
                }
            }
            Some(storage)
        } else {
            None
        }
    }

    pub fn get_sacks(&self, uuid: &str) -> Option<HashMap<&str, i64>> {
        if let Some(data) = self.get_object_property(&format!("{uuid}.sacks_counts")) {
            let mut sacks = HashMap::new();
            for ele in data {
                sacks.insert(&**ele.0, ele.1.as_i64().unwrap_or(0));
            }
            Some(sacks)
        } else {
            None
        }
    }

    pub fn get_pets(&self, uuid: &str) -> Option<Vec<PetStruct>> {
        if let Some(pets) = self.get_array_property(&format!("{uuid}.pets")) {
            let mut parsed_pets = Vec::new();
            for pet in pets {
                let rarity = pet.get_string_property("tier").unwrap();
                parsed_pets.push(PetStruct {
                    leveling: Self::pet_exp_to_info(
                        pet.get_str_property("type").unwrap(),
                        pet.get_int_property("exp").unwrap(),
                        rarity.as_str(),
                    ),
                    rarity,
                    skin: pet.get_string_property("skin"),
                    held_item: pet.get_string_property("heldItem"),
                });
            }
            Some(parsed_pets)
        } else {
            None
        }
    }

    fn pet_exp_to_info(pet_name: &str, pet_exp: i64, pet_rarity: &str) -> LevelingStruct {
        let leveling_table = *PET_EXP;
        let max_level = 100; // TODO: golden dragon

        if pet_exp == 0 {
            return LevelingStruct {
                name: pet_name.to_string(),
                level: 0,
                max_level,
                total_exp: 0,
                current_exp: 0,
                exp_for_next: 0,
                progress_to_next: 0.0,
            };
        }

        let offset = *PET_RARITY_OFFSET.get(pet_rarity).unwrap();

        let mut exp_total = 0;
        let mut level = 1;
        let mut is_maxed = true;
        for i in offset..(leveling_table.len() as i64) {
            let cur_exp_needed = leveling_table[i as usize];
            exp_total += cur_exp_needed;

            if exp_total > pet_exp {
                exp_total -= cur_exp_needed;
                is_maxed = false;
                break;
            }

            level += 1;
        }

        if is_maxed {
            level = 100;
        }

        let exp_current = pet_exp - exp_total;
        let exp_for_next = if level < max_level {
            leveling_table[(level + offset - 1) as usize]
        } else {
            0
        };

        let progress = if exp_for_next > 0 {
            (exp_current as f64 / exp_for_next as f64).clamp(0.0, 1.0)
        } else {
            0.0
        };

        LevelingStruct {
            name: pet_name.to_string(),
            level,
            max_level,
            total_exp: pet_exp,
            current_exp: exp_current,
            exp_for_next,
            progress_to_next: progress,
        }
    }

    pub fn get_fairy_souls(&self, uuid: &str) -> i64 {
        self.get_int_property(&format!("{uuid}.fairy_souls_collected"))
            .unwrap_or(0)
    }

    pub fn get_minion_slots(&self) -> i64 {
        let mut unique_minions = HashSet::new();

        for member in self.members.as_object().unwrap().values() {
            if let Some(minions_unwrap) = member.get_array_property("crafted_generators") {
                for ele in minions_unwrap {
                    unique_minions.insert(ele.as_str().unwrap());
                }
            }
        }

        let mut max = 0;
        for i in 0..CRAFTED_MINIONS_TO_SLOTS.len() {
            if &(unique_minions.len() as i64) < CRAFTED_MINIONS_TO_SLOTS.get(i).unwrap() {
                break;
            }

            max = i;
        }

        (max as i64) + 5
    }

    pub fn get_pet_score(&self, uuid: &str) -> i64 {
        let mut pets_map = HashMap::new();

        if let Some(pets) = self.get_pets(uuid) {
            for ele in pets {
                let rarity = match ele.rarity.to_lowercase().as_str() {
                    "common" => 1,
                    "uncommon" => 2,
                    "rare" => 3,
                    "epic" => 4,
                    "legendary" => 5,
                    _ => 0,
                };

                if let Some(cur_rarity) = pets_map.get(&ele.name) {
                    if cur_rarity < &rarity {
                        pets_map.insert(ele.name.clone(), rarity);
                    }
                } else {
                    pets_map.insert(ele.name.clone(), rarity);
                }
            }
        }

        pets_map.values().sum()
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

#[derive(Debug)]
pub struct LevelingStruct {
    pub name: String,
    pub level: i64,
    pub max_level: i64,
    pub total_exp: i64,
    /// Exp only for the current level
    pub current_exp: i64,
    /// Total exp needed for the next level
    pub exp_for_next: i64,
    /// Progress to next level (0.0 to 1.0)
    pub progress_to_next: f64,
}

impl LevelingStruct {
    pub fn is_maxed(&self) -> bool {
        self.level == self.max_level
    }

    pub fn get_progress_level(&self) -> f64 {
        (self.current_exp as f64) + self.progress_to_next
    }
}

#[derive(Debug)]
pub struct PetStruct {
    pub leveling: LevelingStruct,
    pub rarity: String,
    pub skin: Option<String>,
    pub held_item: Option<String>,
}

impl std::ops::Deref for PetStruct {
    type Target = LevelingStruct;
    fn deref(&self) -> &Self::Target {
        &self.leveling
    }
}
