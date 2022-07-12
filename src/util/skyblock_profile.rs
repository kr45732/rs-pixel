use crate::util::constants::{
    BLAZE_EXP, CATACOMBS_EXP, ENDERMAN_EXP, HOTM_EXP, LEVELING_CAPS, LEVELING_EXP, PET_EXP,
    PET_RARITY_OFFSET, RUNECRAFTING_EXP, SOCIAL_EXP, SPIDER_EXP, WOLF_EXP, ZOMBIE_EXP,
};
use crate::{
    types::gamemode::Gamemode,
    util::generic_json::{Property, Raw},
};
use lazy_static::__Deref;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

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

    pub fn get_player_str_property(&self, full_path: &str) -> Option<&str> {
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

    pub fn get_player_object_property(&self, full_path: &str) -> Option<&Map<String, Value>> {
        self.get_player_property(full_path)
            .and_then(|v| v.as_object())
    }

    pub fn get_purse_coins(&self) -> Option<f64> {
        self.get_player_float_property("coin_purse")
    }

    pub fn get_skill(&self, skill_name: &str) -> Option<LevelingStruct> {
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
            Some(self.skill_exp_to_info(skill_name, skill_exp))
        } else {
            None
        }
    }

    pub fn skill_exp_to_info(&self, skill_name: &str, skill_exp: i64) -> LevelingStruct {
        let leveling_table = match skill_name {
            "catacombs" => *CATACOMBS_EXP,
            "runecrafting" => *RUNECRAFTING_EXP,
            "social" => *SOCIAL_EXP,
            "hotm" => *HOTM_EXP,
            _ => *LEVELING_EXP,
        };
        let max_level = self.get_max_level(skill_name);

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
            } else {
                level = i + 1;
            }
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

    pub fn get_max_level(&self, name: &str) -> i64 {
        LEVELING_CAPS.get(name).unwrap_or(&50)
            + if name == "farming" {
                self.get_farming_cap_upgrade()
            } else {
                0
            }
    }

    pub fn get_farming_cap_upgrade(&self) -> i64 {
        self.get_player_int_property("jacob2.perks.farming_level_cap")
            .unwrap_or(0)
    }

    pub fn get_hotm(&self) -> Option<LevelingStruct> {
        if let Some(exp) = self.get_player_int_property("mining_core.experience") {
            Some(self.skill_exp_to_info("hotm", exp))
        } else {
            None
        }
    }

    pub fn slayer_exp_to_info(&self, slayer_name: &str, slayer_exp: i64) -> LevelingStruct {
        let leveling_table = match slayer_name {
            "zombie" => *ZOMBIE_EXP,
            "wolf" => *WOLF_EXP,
            "spider" => *SPIDER_EXP,
            "enderman" => *ENDERMAN_EXP,
            _ => *BLAZE_EXP,
        };
        let max_level = self.get_max_level(slayer_name);

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
            } else {
                level = i + 1;
            }
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

    pub fn get_slayer(&self, slayer_name: &str) -> Option<LevelingStruct> {
        if let Some(exp) =
            self.get_player_int_property(format!("slayer_bosses.{}.xp", slayer_name).as_str())
        {
            Some(self.slayer_exp_to_info(slayer_name, exp))
        } else {
            None
        }
    }

    pub fn get_catacombs(&self) -> Option<LevelingStruct> {
        if let Some(exp) =
            self.get_player_int_property("dungeons.dungeon_types.catacombs.experience")
        {
            Some(self.skill_exp_to_info("catacombs", exp))
        } else {
            None
        }
    }

    pub fn get_dungeon_class(&self, class_name: &str) -> Option<LevelingStruct> {
        if let Some(exp) = self.get_player_int_property(
            format!("dungeons.player_classes.{}.experience", class_name).as_str(),
        ) {
            Some(self.skill_exp_to_info("catacombs", exp))
        } else {
            None
        }
    }

    fn parse_inventory_nbt(&self, data: &str) -> Option<Value> {
        base64::decode(data).ok().and_then(|bytes| {
            nbt::from_gzip_reader::<_, nbt::Blob>(std::io::Cursor::new(bytes))
                .ok()
                .and_then(|nbt| nbt.get("i").and_then(|v| serde_json::to_value(v).ok()))
        })
    }

    pub fn get_inventory(&self) -> Option<Value> {
        if let Some(data) = self.get_player_str_property("inv_contents.data") {
            self.parse_inventory_nbt(data)
        } else {
            None
        }
    }

    pub fn get_personal_vault(&self) -> Option<Value> {
        if let Some(data) = self.get_player_str_property("personal_vault_contents.data") {
            self.parse_inventory_nbt(data)
        } else {
            None
        }
    }

    pub fn get_talisman_bag(&self) -> Option<Value> {
        if let Some(data) = self.get_player_str_property("talisman_bag.data") {
            self.parse_inventory_nbt(data)
        } else {
            None
        }
    }

    pub fn get_equippment(&self) -> Option<Value> {
        if let Some(data) = self.get_player_str_property("equippment_contents.data") {
            self.parse_inventory_nbt(data)
        } else {
            None
        }
    }

    pub fn get_armor(&self) -> Option<Value> {
        if let Some(data) = self.get_player_str_property("inv_armor.data") {
            self.parse_inventory_nbt(data)
        } else {
            None
        }
    }

    pub fn get_wardrobe(&self) -> Option<Value> {
        if let Some(data) = self.get_player_str_property("wardrobe_contents.data") {
            self.parse_inventory_nbt(data)
        } else {
            None
        }
    }

    pub fn get_ender_chest(&self) -> Option<Value> {
        if let Some(data) = self.get_player_str_property("ender_chest_contents.data") {
            self.parse_inventory_nbt(data)
        } else {
            None
        }
    }

    pub fn get_storage(&self) -> Option<HashMap<&str, Value>> {
        if let Some(data) = self.get_player_object_property("backpack_contents") {
            let mut storage = HashMap::new();
            for ele in data {
                if let Some(bp) = self
                    .parse_inventory_nbt(ele.1.get("data").and_then(|data| data.as_str()).unwrap())
                {
                    storage.insert(ele.0.deref(), bp);
                }
            }
            Some(storage)
        } else {
            None
        }
    }

    pub fn get_sacks(&self) -> Option<HashMap<&str, i64>> {
        if let Some(data) = self.get_object_property("sacks_counts") {
            let mut sacks = HashMap::new();
            for ele in data {
                sacks.insert(ele.0.deref(), ele.1.as_i64().unwrap_or(0));
            }
            Some(sacks)
        } else {
            None
        }
    }

    pub fn get_pets(&self) -> Option<Vec<PetStruct>> {
        if let Some(pets) = self.get_player_array_property("pets") {
            let mut parsed_pets = Vec::new();
            for pet in pets {
                let rarity = pet.get_string_property("tier").unwrap();
                parsed_pets.push(PetStruct {
                    leveling: self.pet_exp_to_info(
                        pet.get_str_property("type").unwrap(),
                        pet.get_int_property("exp").unwrap(),
                        rarity.as_str(),
                    ),
                    rarity: rarity,
                    skin: pet.get_string_property("skin"),
                    held_item: pet.get_string_property("heldItem"),
                });
            }
            Some(parsed_pets)
        } else {
            None
        }
    }

    pub fn pet_exp_to_info(
        &self,
        pet_name: &str,
        pet_exp: i64,
        pet_rarity: &str,
    ) -> LevelingStruct {
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
            } else {
                level += 1;
            }
        }

        if is_maxed {
            level = 100
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
