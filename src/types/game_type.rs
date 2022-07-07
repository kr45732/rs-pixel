use std::{borrow::Cow, fmt};

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, Debug)]
pub struct GameType(Cow<'static, str>, Cow<'static, str>, i32);

impl<'de> Deserialize<'de> for GameType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct I32Visitor;

        impl<'de> Visitor<'de> for I32Visitor {
            type Value = GameType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an u64")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                println!("{} d", value);
                Ok(GameType::QUAKECRAFT)
            }
        }

        // const FIELDS: &'static [&'static str] = &["gameType"];
        deserializer.deserialize_u64(I32Visitor)
    }
}

impl GameType {
    pub const QUAKECRAFT: GameType =
        GameType(Cow::Borrowed("Quakecraft"), Cow::Borrowed("Quake"), 2);
    pub const WALLS: GameType = GameType(Cow::Borrowed("Walls"), Cow::Borrowed("Walls"), 3);
    pub const PAINTBALL: GameType =
        GameType(Cow::Borrowed("Paintball"), Cow::Borrowed("Paintball"), 4);
    pub const SURVIVAL_GAMES: GameType = GameType(
        Cow::Borrowed("Blitz Survival Games"),
        Cow::Borrowed("HungerGames"),
        5,
    );
    pub const TNTGAMES: GameType =
        GameType(Cow::Borrowed("The TNT Games"), Cow::Borrowed("TNTGames"), 6);
    pub const VAMPIREZ: GameType =
        GameType(Cow::Borrowed("VampireZ"), Cow::Borrowed("VampireZ"), 7);
    pub const WALLS3: GameType = GameType(Cow::Borrowed("Mega Walls"), Cow::Borrowed("Walls3"), 13);
    pub const ARCADE: GameType = GameType(Cow::Borrowed("Arcade"), Cow::Borrowed("Arcade"), 14);
    pub const ARENA: GameType = GameType(Cow::Borrowed("Arena Brawl"), Cow::Borrowed("Arena"), 17);
    pub const MCGO: GameType = GameType(Cow::Borrowed("Cops and Crims"), Cow::Borrowed("MCGO"), 21);
    pub const UHC: GameType = GameType(Cow::Borrowed("UHC Champions"), Cow::Borrowed("UHC"), 20);
    pub const BATTLEGROUND: GameType =
        GameType(Cow::Borrowed("Warlords"), Cow::Borrowed("Battleground"), 23);
    pub const SUPER_SMASH: GameType = GameType(
        Cow::Borrowed("Smash Heroes"),
        Cow::Borrowed("SuperSmash"),
        24,
    );
    pub const GINGERBREAD: GameType = GameType(
        Cow::Borrowed("Turbo Kart Racers"),
        Cow::Borrowed("GingerBread"),
        25,
    );
    pub const HOUSING: GameType = GameType(Cow::Borrowed("Housing"), Cow::Borrowed("Housing"), 26);
    pub const SKYWARS: GameType = GameType(Cow::Borrowed("SkyWars"), Cow::Borrowed("SkyWars"), 51);
    pub const TRUE_COMBAT: GameType = GameType(
        Cow::Borrowed("Crazy Walls"),
        Cow::Borrowed("TrueCombat"),
        52,
    );
    pub const SPEED_UHC: GameType =
        GameType(Cow::Borrowed("Speed UHC"), Cow::Borrowed("SpeedUHC"), 54);
    pub const SKYCLASH: GameType =
        GameType(Cow::Borrowed("SkyClash"), Cow::Borrowed("SkyClash"), 55);
    pub const LEGACY: GameType =
        GameType(Cow::Borrowed("Classic Games"), Cow::Borrowed("Legacy"), 56);
    pub const PROTOTYPE: GameType =
        GameType(Cow::Borrowed("Prototype"), Cow::Borrowed("Prototype"), 57);
    pub const BEDWARS: GameType = GameType(Cow::Borrowed("Bed Wars"), Cow::Borrowed("Bedwars"), 58);
    pub const MURDER_MYSTERY: GameType = GameType(
        Cow::Borrowed("Murder Mystery"),
        Cow::Borrowed("MurderMystery"),
        59,
    );
    pub const BUILD_BATTLE: GameType = GameType(
        Cow::Borrowed("Build Battle"),
        Cow::Borrowed("BuildBattle"),
        60,
    );
    pub const DUELS: GameType = GameType(Cow::Borrowed("Duels"), Cow::Borrowed("Duels"), 61);
    pub const SKYBLOCK: GameType =
        GameType(Cow::Borrowed("SkyBlock"), Cow::Borrowed("SkyBlock"), 63);
    pub const PIT: GameType = GameType(Cow::Borrowed("Pit"), Cow::Borrowed("Pit"), 64);
    pub const REPLAY: GameType = GameType(Cow::Borrowed("Replay"), Cow::Borrowed("Replay"), 65);
    pub const SMP: GameType = GameType(Cow::Borrowed("SMP"), Cow::Borrowed("SMP"), 67);
    pub const WOOL_GAMES: GameType =
        GameType(Cow::Borrowed("Wool Wars"), Cow::Borrowed("WoolGames"), 68);
}
