use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{borrow::Cow, fmt};

#[derive(Serialize, Debug, PartialEq, Eq, Hash)]
pub struct GameType(Cow<'static, str>, Cow<'static, str>, i32);

impl<'de> Deserialize<'de> for GameType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GameTypeVisitor;

        impl<'de> Visitor<'de> for GameTypeVisitor {
            type Value = GameType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a u64")
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
        let v_str = v.as_str();
        match v_str {
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

impl GameType {
    pub const fn new(name: &'static str, db_name: &'static str, id: i32) -> GameType {
        GameType(Cow::Borrowed(name), Cow::Borrowed(db_name), id)
    }

    pub fn name(&self) -> String {
        self.0.to_string()
    }

    pub fn db_name(&self) -> String {
        self.1.to_string()
    }

    pub fn id(&self) -> i32 {
        self.2
    }

    pub const QUAKECRAFT: GameType = GameType::new("Quakecraft", "Quake", 2);
    pub const WALLS: GameType = GameType::new("Walls", "Walls", 3);
    pub const PAINTBALL: GameType = GameType::new("Paintball", "Paintball", 4);
    pub const SURVIVAL_GAMES: GameType = GameType::new("Blitz Survival Games", "HungerGames", 5);
    pub const TNTGAMES: GameType = GameType::new("The TNT Games", "TNTGames", 6);
    pub const VAMPIREZ: GameType = GameType::new("VampireZ", "VampireZ", 7);
    pub const WALLS3: GameType = GameType::new("Mega Walls", "Walls3", 13);
    pub const ARCADE: GameType = GameType::new("Arcade", "Arcade", 14);
    pub const ARENA: GameType = GameType::new("Arena Brawl", "Arena", 17);
    pub const MCGO: GameType = GameType::new("Cops and Crims", "MCGO", 21);
    pub const UHC: GameType = GameType::new("UHC Champions", "UHC", 20);
    pub const BATTLEGROUND: GameType = GameType::new("Warlords", "Battleground", 23);
    pub const SUPER_SMASH: GameType = GameType::new("Smash Heroes", "SuperSmash", 24);
    pub const GINGERBREAD: GameType = GameType::new("Turbo Kart Racers", "GingerBread", 25);
    pub const HOUSING: GameType = GameType::new("Housing", "Housing", 26);
    pub const SKYWARS: GameType = GameType::new("SkyWars", "SkyWars", 51);
    pub const TRUE_COMBAT: GameType = GameType::new("Crazy Walls", "TrueCombat", 52);
    pub const SPEED_UHC: GameType = GameType::new("Speed UHC", "SpeedUHC", 54);
    pub const SKYCLASH: GameType = GameType::new("SkyClash", "SkyClash", 55);
    pub const LEGACY: GameType = GameType::new("Classic Games", "Legacy", 56);
    pub const PROTOTYPE: GameType = GameType::new("Prototype", "Prototype", 57);
    pub const BEDWARS: GameType = GameType::new("Bed Wars", "Bedwars", 58);
    pub const MURDER_MYSTERY: GameType = GameType::new("Murder Mystery", "MurderMystery", 59);
    pub const BUILD_BATTLE: GameType = GameType::new("Build Battle", "BuildBattle", 60);
    pub const DUELS: GameType = GameType::new("Duels", "Duels", 61);
    pub const SKYBLOCK: GameType = GameType::new("SkyBlock", "SkyBlock", 63);
    pub const PIT: GameType = GameType::new("Pit", "Pit", 64);
    pub const REPLAY: GameType = GameType::new("Replay", "Replay", 65);
    pub const SMP: GameType = GameType::new("SMP", "SMP", 67);
    pub const WOOL_GAMES: GameType = GameType::new("Wool Wars", "WoolGames", 68);
    pub const UNKNOWN: GameType = GameType::new("Unknown", "Unknown", -1);
}
