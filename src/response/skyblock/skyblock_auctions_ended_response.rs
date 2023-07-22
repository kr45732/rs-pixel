use crate::util::utils::parse_nbt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockAuctionsEndedResponse {
    pub success: bool,
    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,
    pub auctions: Vec<SkyblockAuctionEnded>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockAuctionEnded {
    pub auction_id: String,
    pub seller: String,
    pub seller_profile: String,
    pub buyer: String,
    pub timestamp: i64,
    pub price: i64,
    pub bin: bool,
    pub item_bytes: String,
}

impl SkyblockAuctionEnded {
    pub fn get_nbt(&self) -> Option<Value> {
        parse_nbt(&self.item_bytes)
    }
}
