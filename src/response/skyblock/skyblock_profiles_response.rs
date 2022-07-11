use crate::util::skyblock_profile::SkyblockProfile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockProfilesResponse {
    pub success: bool,
    pub profiles: Vec<SkyblockProfile>,
}

impl SkyblockProfilesResponse {
    pub fn set_uuid(&mut self, uuid: &str) {
        for i in 0..self.profiles.len() {
            if let Some(profile) = self.profiles.get_mut(i) {
                profile.set_uuid(uuid);
            }
        }
    }

    pub fn get_last_played_profile(&self) -> Option<&SkyblockProfile> {
        let last_save = 0;
        let mut last_played_profile = None;
        for profile in &self.profiles {
            if profile.last_save > last_save {
                last_played_profile = Some(profile);
            }
        }
        last_played_profile
    }
}
