use crate::util::skyblock_profile::SkyblockProfile;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockProfileResponse {
    pub success: bool,
    pub profile: SkyblockProfile,
}
