use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PunishmentStatsResponse {
    pub success: bool,
    #[serde(rename = "staff_rollingDaily")]
    pub staff_rolling_daily: i32,
    pub staff_total: i32,
    pub watchdog_total: i32,
    #[serde(rename = "watchdog_lastMinute")]
    pub watchdog_last_minute: i32,
    #[serde(rename = "watchdog_rollingDaily")]
    pub watchdog_rolling_daily: i32,
}
