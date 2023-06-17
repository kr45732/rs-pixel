use crate::util::utils::parse_nbt;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockAuctionsResponse {
    pub success: bool,
    pub page: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
    #[serde(rename = "totalAuctions")]
    pub total_auctions: i64,
    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,
    pub auctions: Vec<SkyblockAuction>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockAuction {
    pub uuid: String,
    pub auctioneer: String,
    pub profile_id: String,
    pub coop: Vec<String>,
    pub start: i64,
    pub end: i64,
    pub item_name: String,
    pub item_lore: String,
    pub extra: String,
    pub category: String,
    pub tier: String,
    pub starting_bid: i64,
    #[serde(deserialize_with = "deserialize_item_bytes")]
    pub item_bytes: String,
    pub claimed: bool,
    pub last_updated: Option<i64>,
    #[serde(default = "Default::default")]
    pub bin: bool,
    pub bids: Vec<SkyblockAuctionBid>,
    pub item_uuid: Option<String>,
}

impl SkyblockAuction {
    pub fn get_nbt(&self) -> Option<Value> {
        parse_nbt(&self.item_bytes)
    }
}

fn deserialize_item_bytes<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct DeserializeItemBytesVisitor;

    impl<'de> de::Visitor<'de> for DeserializeItemBytesVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a String, &str or a Map")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_string(v.to_string())
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(v)
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: de::MapAccess<'de>,
        {
            while let Ok(entry_opt) = map.next_entry::<String, Value>() {
                if let Some(entry) = entry_opt {
                    if entry.0 == "data" && entry.1.is_string() {
                        return Ok(entry.1.as_str().unwrap().to_string());
                    }
                } else {
                    break;
                }
            }

            Err(de::Error::invalid_type(de::Unexpected::Map, &self))
        }
    }

    deserializer.deserialize_any(DeserializeItemBytesVisitor)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockAuctionBid {
    pub bidder: String,
    pub profile_id: String,
    pub amount: i64,
    pub timestamp: i64,
}
