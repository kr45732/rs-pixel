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
    pub original_length: i64,
    pub length: i64,
    #[serde(rename = "gameType")]
    pub game_type: GameType,
    #[serde(rename = "dateActivated")]
    pub date_activated: i64,
    #[serde(deserialize_with = "deserialize_stacked", default = "Default::default")]
    pub stacked: Stacked,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Stacked {
    QueuedToStack(bool),
    Stacked(Vec<String>),
}

impl Default for Stacked {
    fn default() -> Self {
        Stacked::QueuedToStack(false)
    }
}

impl Stacked {
    pub fn get_queued_to_stack(&self) -> Option<&bool> {
        if let Stacked::QueuedToStack(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn get_stacked(&self) -> Option<&Vec<String>> {
        if let Stacked::Stacked(v) = self {
            Some(v)
        } else {
            None
        }
    }
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
