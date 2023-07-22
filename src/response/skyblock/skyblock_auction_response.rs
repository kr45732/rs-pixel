use super::skyblock_auctions_response::SkyblockAuction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockAuctionResponse {
    pub success: bool,
    pub auctions: Vec<SkyblockAuction>,
}
