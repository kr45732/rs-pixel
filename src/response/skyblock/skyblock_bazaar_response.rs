use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockBazaarResponse {
    pub success: bool,
    #[serde(rename = "lastUpdated")]
    pub last_updated: i64,
    pub products: HashMap<String, SkyblockBazaarProduct>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SkyblockBazaarProduct {
    pub product_id: String,
    pub sell_summary: Vec<ProductSummary>,
    pub buy_summary: Vec<ProductSummary>,
    pub quick_status: ProductStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductSummary {
    pub amount: i64,
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: f64,
    pub orders: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductStatus {
    #[serde(rename = "sellPrice")]
    pub sell_price: f64,
    #[serde(rename = "sellVolume")]
    pub sell_volume: i64,
    #[serde(rename = "sellMovingWeek")]
    pub sell_moving_week: i64,
    #[serde(rename = "sellOrders")]
    pub sell_orders: i64,
    #[serde(rename = "buyPrice")]
    pub buy_price: f64,
    #[serde(rename = "buyVolume")]
    pub buy_volume: i64,
    #[serde(rename = "buyMovingWeek")]
    pub buy_moving_week: i64,
    #[serde(rename = "buyOrders")]
    pub buy_orders: i64,
}
