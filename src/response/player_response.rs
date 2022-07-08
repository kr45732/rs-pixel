use crate::util::{
    generic_json::{Property, Raw},
    utils,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerResponse {
    pub success: bool,
    pub player: Value,
}

impl Raw for PlayerResponse {
    fn raw(&self) -> &Value {
        &self.player
    }
}

impl PlayerResponse {
    pub fn get_uuid(&self) -> Option<&str> {
        self.get_string_property("uuid")
    }

    pub fn get_name(&self) -> Option<&str> {
        match self.get_string_property("displayname") {
            Some(name) => Some(name),
            None => {
                match self
                    .get_array_property("knownAliases")
                    .and_then(|aliases| aliases.last())
                    .and_then(|alias| alias.as_str())
                {
                    Some(alias) => {
                        return Some(alias);
                    }
                    None => {
                        return self
                            .get_string_property("playername")
                            .or(self.get_string_property("username"))
                    }
                }
            }
        }
    }

    pub fn get_network_exp(&self) -> i64 {
        self.get_int_property("networkExp").unwrap_or(0)
            + (utils::get_total_exp_to_full_level(
                (self.get_int_property("networkLevel").unwrap_or(0) + 1) as f64,
            ) as i64)
    }

    pub fn get_network_level(&self) -> f64 {
        utils::get_exact_level(self.get_network_exp() as f64)
    }

    pub fn get_karma(&self) -> i64 {
        self.get_int_property("karma").unwrap_or(0)
    }

    pub fn get_rank(&self) -> &str {
        if self.has_rank_in_field("rank") {
            return self.get_string_property("rank").unwrap_or("NONE");
        } else if self.has_rank_in_field("monthlyPackageRank") {
            return self
                .get_string_property("monthlyPackageRank")
                .unwrap_or("NONE");
        } else if self.has_rank_in_field("newPackageRank") {
            return self.get_string_property("newPackageRank").unwrap_or("NONE");
        } else if self.has_rank_in_field("packageRank") {
            return self.get_string_property("packageRank").unwrap_or("NONE");
        }

        return "NONE";
    }

    pub fn has_rank(&self) -> bool {
        self.get_rank() != "NONE"
    }

    fn has_rank_in_field(&self, name: &str) -> bool {
        let value = self.get_string_property(name).unwrap_or("NONE");
        !value.is_empty() && value != "NONE" && value != "NORMAL"
    }
}
