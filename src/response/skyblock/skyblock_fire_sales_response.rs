use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockFireSalesResponse {
    pub success: bool,
    pub sales: Vec<SkyblockFireSale>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockFireSale {
    pub item_id: String,
    pub start: i64,
    pub end: i64,
    pub amount: i64,
    pub price: i64,
}
