use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Gamemode {
    #[default]
    Regular,
    Ironman,
    Stranded,
    Bingo,
}
