use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize, Debug, Default)]
pub enum Gamemode {
    #[default]
    Regular,
    Ironman,
    Stranded,
    Bingo,
}

impl Serialize for Gamemode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::Regular => "",
            Self::Ironman => "ironman",
            Self::Stranded => "island",
            Self::Bingo => "bingo",
        })
    }
}
