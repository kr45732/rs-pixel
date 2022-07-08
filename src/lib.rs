pub mod error;
pub mod response;
pub mod types;
pub mod util;

use error::RsPixelError;
use response::{
    boosters_response::BoostersResponse, key_response::KeyResponse,
    leaderboards_response::LeaderboardsResponse, player_response::PlayerResponse,
    punishment_stats_response::PunishmentStatsResponse,
};
use serde_json::{json, Value};
use std::time::Duration;
use surf::Client;
use util::minecraft::{self, MinecraftApiType, MinecraftResponse};

struct Key {
    pub key: String,
    remaining_limit: i64,
    time_till_reset: i64,
    time: i64,
}

impl Key {
    pub fn new(key: &str) -> Key {
        Key {
            key: key.to_string(),
            remaining_limit: 0,
            time_till_reset: 0,
            time: 0,
        }
    }

    pub fn update_remaining_limit(&mut self, remaining_limit: i64) {
        self.remaining_limit = remaining_limit;
        self.time = chrono::Utc::now().timestamp_millis();
    }

    pub fn update_time_till_reset(&mut self, time_till_reset: i64) {
        self.time_till_reset = time_till_reset;
        self.time = chrono::Utc::now().timestamp_millis();
    }

    pub fn is_rate_limited(&self) -> bool {
        return self.remaining_limit < 5
            && self.time_till_reset > 0
            && self.time + self.time_till_reset * 1000 > chrono::Utc::now().timestamp_millis();
    }

    pub fn get_time_till_reset(&self) -> i64 {
        return std::cmp::max(
            0,
            ((self.time + self.time_till_reset * 1000) - chrono::Utc::now().timestamp_millis())
                / 1000,
        );
    }
}

pub struct RsPixel {
    pub client: Client,
    key: Key,
    pub minecraft_api_type: MinecraftApiType,
}

impl RsPixel {
    pub async fn new(key: &str) -> Result<RsPixel, RsPixelError> {
        let client = surf::Config::new()
            .set_timeout(Some(Duration::from_secs(15)))
            .try_into()
            .unwrap();
        let mut rs_pixel = RsPixel {
            client,
            key: Key::new(key),
            minecraft_api_type: MinecraftApiType::Ashcon,
        };

        rs_pixel
            .simple_get("key".to_string())
            .await
            .map(|_| rs_pixel)
    }

    pub fn set_minecraft_api_type(&mut self, minecraft_api_type: MinecraftApiType) {
        self.minecraft_api_type = minecraft_api_type;
    }

    pub async fn username_to_uuid(
        &self,
        username: &str,
    ) -> Result<MinecraftResponse, RsPixelError> {
        minecraft::username_to_uuid(self, username).await
    }

    pub async fn uuid_to_username(&self, uuid: &str) -> Result<MinecraftResponse, RsPixelError> {
        minecraft::uuid_to_username(self, uuid).await
    }

    pub async fn simple_get(&mut self, path: String) -> Result<Value, RsPixelError> {
        self.get(path, json!({})).await
    }

    pub async fn get(&mut self, path: String, params: Value) -> Result<Value, RsPixelError> {
        if self.key.is_rate_limited() {
            let time_till_reset = self.key.get_time_till_reset();
            println!("Sleeping for {} seconds", time_till_reset);
            std::thread::sleep(Duration::from_secs(time_till_reset.try_into().unwrap()));
        }

        match self
            .client
            .get(format!("https://api.hypixel.net/{}", path))
            .query(&params)
            .unwrap()
            .header("API-Key", self.key.key.to_string())
            .send()
            .await
        {
            Ok(mut res_unwrap) => {
                if let Some(remaining_limit) = res_unwrap
                    .header("RateLimit-Remaining")
                    .map(|header| header.as_str().parse::<i64>().ok())
                    .unwrap_or(None)
                {
                    self.key.update_remaining_limit(remaining_limit);
                }
                if let Some(time_till_reset) = res_unwrap
                    .header("RateLimit-Reset")
                    .map(|header| header.as_str().parse::<i64>().ok())
                    .unwrap_or(None)
                {
                    self.key.update_time_till_reset(time_till_reset);
                }

                let json = res_unwrap
                    .body_json::<Value>()
                    .await
                    .map_err(|err| RsPixelError::from(err));

                if res_unwrap.status() == 200 {
                    return json;
                }

                Err(RsPixelError::from((
                    res_unwrap.status(),
                    json.ok()
                        .as_ref()
                        .and_then(|json_unwrap| json_unwrap.get("cause"))
                        .and_then(|cause| cause.as_str())
                        .unwrap_or("Unknown fail cause")
                        .to_string(),
                )))
            }
            Err(err) => Err(RsPixelError::from(err)),
        }
    }

    pub async fn get_key(&mut self) -> Result<KeyResponse, RsPixelError> {
        self.simple_get("key".to_string())
            .await
            .and_then(|response| {
                serde_json::from_value(response).map_err(|err| RsPixelError::from(err))
            })
    }

    pub async fn get_boosters(&mut self) -> Result<BoostersResponse, RsPixelError> {
        self.simple_get("boosters".to_string())
            .await
            .and_then(|response| {
                serde_json::from_value(response).map_err(|err| RsPixelError::from(err))
            })
    }

    pub async fn get_leaderboards(&mut self) -> Result<LeaderboardsResponse, RsPixelError> {
        self.simple_get("leaderboards".to_string())
            .await
            .and_then(|response| {
                serde_json::from_value(response).map_err(|err| RsPixelError::from(err))
            })
    }

    pub async fn get_punishment_stats(&mut self) -> Result<PunishmentStatsResponse, RsPixelError> {
        self.simple_get("punishmentstats".to_string())
            .await
            .and_then(|response| {
                serde_json::from_value(response).map_err(|err| RsPixelError::from(err))
            })
    }

    pub async fn get_player_by_uuid(&mut self, uuid: &str) -> Result<PlayerResponse, RsPixelError> {
        self.get("player".to_string(), json!({ "uuid": uuid }))
            .await
            .and_then(|response| {
                serde_json::from_value(response).map_err(|err| RsPixelError::from(err))
            })
    }
}
