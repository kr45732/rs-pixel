use crate::util::skyblock_profile::SkyblockProfile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockProfilesResponse {
    pub success: bool,
    pub profiles: Vec<SkyblockProfile>,
}

impl SkyblockProfilesResponse {
    pub fn get_selected_profile(&self) -> Option<&SkyblockProfile> {
        self.profiles.iter().find(|&profile| profile.selected)
    }

    pub fn get_profile_by_name(&self, profile_name: &str) -> Option<&SkyblockProfile> {
        for profile in &self.profiles {
            if let Some(cur_profile_name) = &profile.cute_name {
                if cur_profile_name == profile_name {
                    return Some(profile);
                }
            }
        }

        None
    }
}
