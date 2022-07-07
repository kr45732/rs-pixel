use crate::types::game_type::GameType;
use serde::{
    de::{self, SeqAccess},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct BoostersResponse {
    pub success: bool,
    pub boosters: Vec<Booster>,
    #[serde(rename = "boosterState")]
    pub booster_state: BoosterState,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Booster {
    #[serde(rename = "purchaserUuid")]
    pub purchaser_uuid: String,
    pub amount: f64,
    #[serde(rename = "originalLength")]
    pub original_length: i32,
    pub length: i32,
    #[serde(rename = "gameType")]
    pub game_type: GameType,
    #[serde(rename = "dateActivated")]
    pub date_activated: i64,
    #[serde(deserialize_with = "deserialize_stacked", default = "default_stacked")]
    pub stacked: Stacked,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Stacked {
    QueuedToStack(bool),
    Stacked(Vec<String>),
}

fn default_stacked() -> Stacked {
    Stacked::QueuedToStack(false)
}

fn deserialize_stacked<'de, D>(deserializer: D) -> Result<Stacked, D::Error>
where
    D: Deserializer<'de>,
{
    struct DeserializeStackedVisitor;

    impl<'de> de::Visitor<'de> for DeserializeStackedVisitor {
        type Value = Stacked;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a Vec<String> or a bool")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Stacked::QueuedToStack(v))
        }

        fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(elem) = visitor.next_element()? {
                vec.push(elem);
            }
            Ok(Stacked::Stacked(vec))
        }
    }

    deserializer.deserialize_any(DeserializeStackedVisitor)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoosterState {
    pub decrementing: bool,
}
