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
    #[serde(
        deserialize_with = "deserialize_bool_or_vec",
        default = "default_resource"
    )]
    stacked: Vec<String>,
}

impl Booster {
    fn get_stacked(self) -> Vec<String> {
        if self.stacked.get(0).is_some() && self.stacked.get(0).unwrap() == "true" {
            return Vec::new();
        }

        self.stacked
    }

    fn queued_to_stack(self) -> bool {
        self.stacked.get(0).is_some() && self.stacked.get(0).unwrap() == "true"
    }
}

fn default_resource() -> Vec<String> {
    Vec::new()
}

fn deserialize_bool_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeserializeU64OrEmptyStringVisitor)
}
struct DeserializeU64OrEmptyStringVisitor;

impl<'de> de::Visitor<'de> for DeserializeU64OrEmptyStringVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer or a string")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(vec![v.to_string()])
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut vec = Vec::new();

        while let Some(elem) = visitor.next_element()? {
            vec.push(elem);
        }

        Ok(vec)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoosterState {
    pub decrementing: bool,
}
