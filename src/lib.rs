#![warn(clippy::all, clippy::pedantic, clippy::cargo)]

pub mod response;
pub mod types;
pub mod util;

use response::{
    boosters_response::BoostersResponse,
    counts_response::CountsResponse,
    friends_response::FriendsResponse,
    guild_response::GuildResponse,
    key_response::KeyResponse,
    leaderboards_response::LeaderboardsResponse,
    player_response::PlayerResponse,
    punishment_stats_response::PunishmentStatsResponse,
    recent_games_response::RecentGamesResponse,
    skyblock::{
        skyblock_auctions_response::SkyblockAuctionsResponse,
        skyblock_bazaar_response::SkyblockBazaarResponse,
        skyblock_bingo_response::SkyblockBingoResponse,
        skyblock_ended_auctions_response::SkyblockEndedAuctionsResponse,
        skyblock_fire_sales_response::SkyblockFireSalesResponse,
        skyblock_news_response::SkyblockNewsResponse,
        skyblock_profile_response::SkyblockProfileResponse,
        skyblock_profiles_response::SkyblockProfilesResponse,
    },
    status_response::StatusResponse,
};
use serde_json::{json, Value};
use std::{cmp::max, time::Duration};
use surf::Client;
use util::{
    error::Error,
    minecraft::{self, ApiType, Response},
    utils::get_timestamp_millis,
};

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
        self.time = get_timestamp_millis();
    }

    pub fn update_time_till_reset(&mut self, time_till_reset: i64) {
        self.time_till_reset = time_till_reset;
        self.time = get_timestamp_millis();
    }

    pub fn is_rate_limited(&self) -> bool {
        self.remaining_limit <= 1
            && self.time_till_reset > 0
            && self.time + self.time_till_reset * 1000 > get_timestamp_millis()
    }

    pub fn get_time_till_reset(&self) -> i64 {
        max(
            0,
            ((self.time + self.time_till_reset * 1000) - get_timestamp_millis()) / 1000,
        )
    }
}

#[derive(PartialEq)]
pub enum RateLimitStrategy {
    Delay,
    Error,
}

pub struct Config {
    pub client: Client,
    pub minecraft_api_type: ApiType,
    pub rate_limit_strategy: RateLimitStrategy,
}

#[derive(Default)]
pub struct ConfigBuilder {
    client: Option<Client>,
    minecraft_api_type: Option<ApiType>,
    rate_limit_strategy: Option<RateLimitStrategy>,
}

impl ConfigBuilder {
    pub fn client(mut self, client: Client) -> ConfigBuilder {
        self.client = Some(client);
        self
    }

    pub fn minecraft_api_type(mut self, minecraft_api_type: ApiType) -> ConfigBuilder {
        self.minecraft_api_type = Some(minecraft_api_type);
        self
    }

    pub fn rate_limit_strategy(mut self, rate_limit_strategy: RateLimitStrategy) -> ConfigBuilder {
        self.rate_limit_strategy = Some(rate_limit_strategy);
        self
    }
}

impl From<ConfigBuilder> for Config {
    fn from(v: ConfigBuilder) -> Self {
        Config {
            client: v.client.unwrap_or_else(|| {
                surf::Config::new()
                    .set_timeout(Some(Duration::from_secs(15)))
                    .try_into()
                    .unwrap()
            }),
            minecraft_api_type: v.minecraft_api_type.unwrap_or(ApiType::Ashcon),
            rate_limit_strategy: v.rate_limit_strategy.unwrap_or(RateLimitStrategy::Delay),
        }
    }
}

pub struct RsPixel {
    pub config: Config,
    key: Key,
}

impl RsPixel {
    pub async fn new(key: &str) -> Result<RsPixel, Error> {
        let mut rs_pixel = RsPixel {
            config: ConfigBuilder::default().into(),
            key: Key::new(key),
        };

        rs_pixel.simple_get("key").await.map(|_| rs_pixel)
    }

    pub async fn from_config(key: &str, config: Config) -> Result<RsPixel, Error> {
        let mut rs_pixel = RsPixel {
            config,
            key: Key::new(key),
        };

        rs_pixel.simple_get("key").await.map(|_| rs_pixel)
    }

    pub async fn username_to_uuid(&self, username: &str) -> Result<Response, Error> {
        minecraft::username_to_uuid(self, username).await
    }

    pub async fn uuid_to_username(&self, uuid: &str) -> Result<Response, Error> {
        minecraft::uuid_to_username(self, uuid).await
    }

    pub async fn simple_get(&mut self, path: &str) -> Result<Value, Error> {
        self.get(path, json!({})).await
    }

    pub async fn get(&mut self, path: &str, params: Value) -> Result<Value, Error> {
        if self.key.is_rate_limited() {
            let time_till_reset = self.key.get_time_till_reset();
            match self.config.rate_limit_strategy {
                RateLimitStrategy::Delay => {
                    println!("Sleeping for {} seconds", time_till_reset);
                    std::thread::sleep(Duration::from_secs(time_till_reset.try_into().unwrap()));
                }
                RateLimitStrategy::Error => {
                    return Err(Error::RateLimit(self.key.time_till_reset));
                }
            }
        }

        match self
            .config
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
                    .and_then(|header| header.as_str().parse::<i64>().ok())
                {
                    self.key.update_remaining_limit(remaining_limit);
                }
                if let Some(time_till_reset) = res_unwrap
                    .header("RateLimit-Reset")
                    .and_then(|header| header.as_str().parse::<i64>().ok())
                {
                    self.key.update_time_till_reset(time_till_reset);
                }

                let json = res_unwrap.body_json::<Value>().await.map_err(Error::from);

                if res_unwrap.status() == 200 {
                    return json;
                }

                Err(Error::from((
                    res_unwrap.status(),
                    json.ok()
                        .as_ref()
                        .and_then(|json_unwrap| json_unwrap.get("cause"))
                        .and_then(serde_json::Value::as_str)
                        .unwrap_or("Unknown fail cause")
                        .to_string(),
                )))
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    pub async fn get_key(&mut self) -> Result<KeyResponse, Error> {
        self.simple_get("key")
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_boosters(&mut self) -> Result<BoostersResponse, Error> {
        self.simple_get("boosters")
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_leaderboards(&mut self) -> Result<LeaderboardsResponse, Error> {
        self.simple_get("leaderboards")
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_punishment_stats(&mut self) -> Result<PunishmentStatsResponse, Error> {
        self.simple_get("punishmentstats")
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_player_by_uuid(&mut self, uuid: &str) -> Result<PlayerResponse, Error> {
        self.get("player", json!({ "uuid": uuid }))
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_player_by_username(
        &mut self,
        username: &str,
    ) -> Result<PlayerResponse, Error> {
        let minecraft_response = self.username_to_uuid(username).await?;

        self.get_player_by_uuid(minecraft_response.uuid.as_str())
            .await
    }

    pub async fn get_friends(&mut self, uuid: &str) -> Result<FriendsResponse, Error> {
        self.get("friends", json!({ "uuid": uuid }))
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    async fn get_guild(&mut self, key: &str, value: &str) -> Result<GuildResponse, Error> {
        self.get("guild", json!({ key: value }))
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_guild_by_player(&mut self, uuid: &str) -> Result<GuildResponse, Error> {
        self.get_guild("player", uuid).await
    }

    pub async fn get_guild_by_name(&mut self, name: &str) -> Result<GuildResponse, Error> {
        self.get_guild("name", name).await
    }

    pub async fn get_guild_by_id(&mut self, id: &str) -> Result<GuildResponse, Error> {
        self.get_guild("id", id).await
    }

    pub async fn get_counts(&mut self) -> Result<CountsResponse, Error> {
        self.simple_get("counts")
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_status(&mut self, uuid: &str) -> Result<StatusResponse, Error> {
        self.get("status", json!({ "uuid": uuid }))
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_recent_games(&mut self, uuid: &str) -> Result<RecentGamesResponse, Error> {
        self.get("recentGames", json!({ "uuid": uuid }))
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_skyblock_profiles_by_uuid(
        &mut self,
        uuid: &str,
    ) -> Result<SkyblockProfilesResponse, Error> {
        self.get("skyblock/profiles", json!({ "uuid": uuid }))
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
            .map(|mut response: SkyblockProfilesResponse| {
                response.set_uuid(uuid);
                response
            })
    }

    pub async fn get_skyblock_profiles_by_name(
        &mut self,
        username: &str,
    ) -> Result<SkyblockProfilesResponse, Error> {
        let minecraft_response = self.username_to_uuid(username).await?;

        self.get_skyblock_profiles_by_uuid(minecraft_response.uuid.as_str())
            .await
    }

    pub async fn get_skyblock_profile(
        &mut self,
        profile: &str,
    ) -> Result<SkyblockProfileResponse, Error> {
        self.get("skyblock/profile", json!({ "profile": profile }))
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_skyblock_bingo(&mut self, uuid: &str) -> Result<SkyblockBingoResponse, Error> {
        self.get("skyblock/bingo", json!({ "uuid": uuid }))
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_skyblock_news(&mut self) -> Result<SkyblockNewsResponse, Error> {
        self.simple_get("skyblock/news")
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_skyblock_auctions(
        &mut self,
        page: i64,
    ) -> Result<SkyblockAuctionsResponse, Error> {
        self.get("skyblock/auctions", json!({ "page": page }))
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_skyblock_ended_auctions(
        &mut self,
    ) -> Result<SkyblockEndedAuctionsResponse, Error> {
        self.simple_get("skyblock/auctions_ended")
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_skyblock_bazaar(&mut self) -> Result<SkyblockBazaarResponse, Error> {
        self.simple_get("skyblock/bazaar")
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_skyblock_fire_sales(&mut self) -> Result<SkyblockFireSalesResponse, Error> {
        self.simple_get("skyblock/firesales")
            .await
            .and_then(|response| serde_json::from_value(response).map_err(Error::from))
    }

    pub async fn get_resource(&mut self, resource: &str) -> Result<Value, Error> {
        self.simple_get(format!("resources/{}", resource).as_str())
            .await
    }
}
