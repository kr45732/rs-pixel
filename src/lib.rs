mod error;
mod response;
mod types;

use error::RsPixelError;
use response::{boosters_response::BoostersResponse, key_response::KeyResponse};
use serde_json::{json, Value};
use std::time::Duration;
use surf::Client;

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
    client: Client,
    key: Key,
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
        };

        rs_pixel
            .simple_get("key".to_string())
            .await
            .map(|_| rs_pixel)
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
}
