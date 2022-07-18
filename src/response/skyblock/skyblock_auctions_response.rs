use crate::util::utils::parse_nbt;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub item_bytes: String,
    pub claimed: bool,
    pub last_updated: i64,
    pub bin: bool,
    pub bids: Vec<SkyblockAuctionBid>,
    pub item_uuid: Option<String>,
}

impl SkyblockAuction {
    pub fn get_nbt(&self) -> Option<Value> {
        parse_nbt(&self.item_bytes)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockAuctionBid {
    pub bidder: String,
    pub profile_id: String,
    pub amount: i64,
    pub timestamp: i64,
}
