use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct GameType(&'static str, &'static str, &'static str, i64);

impl GameType {
    pub fn name(&self) -> String {
        self.1.to_string()
    }

    pub fn db_name(&self) -> String {
        self.2.to_string()
    }

    pub fn id(&self) -> i64 {
        self.3
    }

    pub const QUAKECRAFT: Self = Self("QUAKECRAFT", "Quakecraft", "Quake", 2);
    pub const WALLS: Self = Self("WALLS", "Walls", "Walls", 3);
    pub const PAINTBALL: Self = Self("PAINTBALL", "Paintball", "Paintball", 4);
    pub const SURVIVAL_GAMES: Self =
        Self("SURVIVAL_GAMES", "Blitz Survival Games", "HungerGames", 5);
    pub const TNTGAMES: Self = Self("TNTGAMES", "The TNT Games", "TNTGames", 6);
    pub const VAMPIREZ: Self = Self("VAMPIREZ", "VampireZ", "VampireZ", 7);
    pub const WALLS3: Self = Self("WALLS3", "Mega Walls", "Walls3", 13);
    pub const ARCADE: Self = Self("ARCADE", "Arcade", "Arcade", 14);
    pub const ARENA: Self = Self("ARENA", "Arena Brawl", "Arena", 17);
    pub const MCGO: Self = Self("MCGO", "Cops and Crims", "MCGO", 21);
    pub const UHC: Self = Self("UHC", "UHC Champions", "UHC", 20);
    pub const BATTLEGROUND: Self = Self("BATTLEGROUND", "Warlords", "Battleground", 23);
    pub const SUPER_SMASH: Self = Self("SUPER_SMASH", "Smash Heroes", "SuperSmash", 24);
    pub const GINGERBREAD: Self = Self("GINGERBREAD", "Turbo Kart Racers", "GingerBread", 25);
    pub const HOUSING: Self = Self("HOUSING", "Housing", "Housing", 26);
    pub const SKYWARS: Self = Self("SKYWARS", "SkyWars", "SkyWars", 51);
    pub const TRUE_COMBAT: Self = Self("TRUE_COMBAT", "Crazy Walls", "TrueCombat", 52);
    pub const SPEED_UHC: Self = Self("SPEED_UHC", "Speed UHC", "SpeedUHC", 54);
    pub const SKYCLASH: Self = Self("SKYCLASH", "SkyClash", "SkyClash", 55);
    pub const LEGACY: Self = Self("LEGACY", "Classic Games", "Legacy", 56);
    pub const PROTOTYPE: Self = Self("PROTOTYPE", "Prototype", "Prototype", 57);
    pub const BEDWARS: Self = Self("BEDWARS", "Bed Wars", "Bedwars", 58);
    pub const MURDER_MYSTERY: Self = Self("MURDER_MYSTERY", "Murder Mystery", "MurderMystery", 59);
    pub const BUILD_BATTLE: Self = Self("BUILD_BATTLE", "Build Battle", "BuildBattle", 60);
    pub const DUELS: Self = Self("DUELS", "Duels", "Duels", 61);
    pub const SKYBLOCK: Self = Self("SKYBLOCK", "SkyBlock", "SkyBlock", 63);
    pub const PIT: Self = Self("PIT", "Pit", "Pit", 64);
    pub const REPLAY: Self = Self("REPLAY", "Replay", "Replay", 65);
    pub const SMP: Self = Self("SMP", "SMP", "SMP", 67);
    pub const WOOL_GAMES: Self = Self("WOOL_GAMES", "Wool Wars", "WoolGames", 68);
    pub const UNKNOWN: Self = Self("UNKNOWN", "Unknown", "Unknown", -1);
}

impl Serialize for GameType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0)
    }
}

impl<'de> Deserialize<'de> for GameType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GameTypeVisitor;

        impl<'de> Visitor<'de> for GameTypeVisitor {
            type Value = GameType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a u64 or String")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GameType::from(value))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GameType::from(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(GameType::from(value.to_string()))
            }
        }

        deserializer.deserialize_any(GameTypeVisitor)
    }
}

impl From<u64> for GameType {
    fn from(v: u64) -> Self {
        match v {
            2 => GameType::QUAKECRAFT,
            3 => GameType::WALLS,
            4 => GameType::PAINTBALL,
            5 => GameType::SURVIVAL_GAMES,
            6 => GameType::TNTGAMES,
            7 => GameType::VAMPIREZ,
            13 => GameType::WALLS3,
            14 => GameType::ARCADE,
            17 => GameType::ARENA,
            21 => GameType::MCGO,
            20 => GameType::UHC,
            23 => GameType::BATTLEGROUND,
            24 => GameType::SUPER_SMASH,
            25 => GameType::GINGERBREAD,
            26 => GameType::HOUSING,
            51 => GameType::SKYWARS,
            52 => GameType::TRUE_COMBAT,
            54 => GameType::SPEED_UHC,
            55 => GameType::SKYCLASH,
            56 => GameType::LEGACY,
            57 => GameType::PROTOTYPE,
            58 => GameType::BEDWARS,
            59 => GameType::MURDER_MYSTERY,
            60 => GameType::BUILD_BATTLE,
            61 => GameType::DUELS,
            63 => GameType::SKYBLOCK,
            64 => GameType::PIT,
            65 => GameType::REPLAY,
            67 => GameType::SMP,
            68 => GameType::WOOL_GAMES,
            _ => GameType::UNKNOWN,
        }
    }
}

impl From<String> for GameType {
    fn from(v: String) -> Self {
        match v.as_str() {
            "QUAKECRAFT" => GameType::QUAKECRAFT,
            "WALLS" => GameType::WALLS,
            "PAINTBALL" => GameType::PAINTBALL,
            "SURVIVAL_GAMES" => GameType::SURVIVAL_GAMES,
            "TNTGAMES" => GameType::TNTGAMES,
            "VAMPIREZ" => GameType::VAMPIREZ,
            "WALLS3" => GameType::WALLS3,
            "ARCADE" => GameType::ARCADE,
            "ARENA" => GameType::ARENA,
            "MCGO" => GameType::MCGO,
            "UHC" => GameType::UHC,
            "BATTLEGROUND" => GameType::BATTLEGROUND,
            "SUPER_SMASH" => GameType::SUPER_SMASH,
            "GINGERBREAD" => GameType::GINGERBREAD,
            "HOUSING" => GameType::HOUSING,
            "SKYWARS" => GameType::SKYWARS,
            "TRUE_COMBAT" => GameType::TRUE_COMBAT,
            "SPEED_UHC" => GameType::SPEED_UHC,
            "SKYCLASH" => GameType::SKYCLASH,
            "LEGACY" => GameType::LEGACY,
            "PROTOTYPE" => GameType::PROTOTYPE,
            "BEDWARS" => GameType::BEDWARS,
            "MURDER_MYSTERY" => GameType::MURDER_MYSTERY,
            "BUILD_BATTLE" => GameType::BUILD_BATTLE,
            "DUELS" => GameType::DUELS,
            "SKYBLOCK" => GameType::SKYBLOCK,
            "PIT" => GameType::PIT,
            "REPLAY" => GameType::REPLAY,
            "SMP" => GameType::SMP,
            "WOOL_GAMES" => GameType::WOOL_GAMES,
            _ => GameType::UNKNOWN,
        }
    }
}
