use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PunishmentStatsResponse {
    pub success: bool,
    #[serde(rename = "staff_rollingDaily")]
    pub staff_rolling_daily: i64,
    pub staff_total: i64,
    pub watchdog_total: i64,
    #[serde(rename = "watchdog_lastMinute")]
    pub watchdog_last_minute: i64,
    #[serde(rename = "watchdog_rollingDaily")]
    pub watchdog_rolling_daily: i64,
}
