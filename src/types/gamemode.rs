use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Gamemode {
    Regular,
    Ironman,
    Stranded,
    Bingo,
}

impl Default for Gamemode {
    fn default() -> Self {
        Gamemode::Regular
    }
}
