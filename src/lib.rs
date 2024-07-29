#![warn(clippy::all)]

pub mod response;
pub mod types;
pub mod util;

use moka::{future::Cache, Expiry};
use response::{
    boosters_response::BoostersResponse,
    counts_response::CountsResponse,
    guild_response::GuildResponse,
    key_response::KeyResponse,
    leaderboards_response::LeaderboardsResponse,
    player_response::PlayerResponse,
    punishment_stats_response::PunishmentStatsResponse,
    recent_games_response::RecentGamesResponse,
    skyblock::{
        skyblock_auction_response::SkyblockAuctionResponse,
        skyblock_auctions_ended_response::SkyblockAuctionsEndedResponse,
        skyblock_auctions_response::SkyblockAuctionsResponse,
        skyblock_bazaar_response::SkyblockBazaarResponse,
        skyblock_bingo_response::SkyblockBingoResponse,
        skyblock_fire_sales_response::SkyblockFireSalesResponse,
        skyblock_news_response::SkyblockNewsResponse,
        skyblock_profile_response::SkyblockProfileResponse,
        skyblock_profiles_response::SkyblockProfilesResponse,
    },
    status_response::StatusResponse,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    any::Any,
    cmp::max,
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use surf::Client;
use util::{
    error::Error,
    minecraft::{self, ApiType, Response},
    utils::get_timestamp_millis,
};

pub struct RsPixel {
    pub config: Config,
    key: Key,
}

impl RsPixel {
    pub async fn new(key: impl Into<String>) -> Result<RsPixel, Error> {
        RsPixel::from_config(key, ConfigBuilder::default().into()).await
    }

    pub async fn from_config(key: impl Into<String>, config: Config) -> Result<RsPixel, Error> {
        let rs_pixel = RsPixel {
            config,
            key: Key::new(key),
        };

        Ok(rs_pixel)
    }

    pub async fn username_to_uuid(&self, username: &str) -> Result<Response, Error> {
        minecraft::username_to_uuid(self, username).await
    }

    pub async fn uuid_to_username(&self, uuid: &str) -> Result<Response, Error> {
        minecraft::uuid_to_username(self, uuid).await
    }

    pub fn to_params(&self, key: &str, value: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        params.insert(key.to_string(), value.to_string());
        params
    }

    pub fn is_cached(&mut self, path: &str, params: HashMap<String, String>) -> bool {
        if let Some(cache) = &self.config.cache {
            cache.contains_key(&format!("{}-{:?}", path, params))
        } else {
            false
        }
    }

    pub async fn get<T>(
        &mut self,
        endpoint: HypixelEndpoint,
        params: HashMap<String, String>,
    ) -> Result<Arc<T>, Error>
    where
        for<'a> T: DeserializeOwned + Send + Sync + 'a,
    {
        let cache_key = format!("{}-{:?}", endpoint.0, params);

        if let Some(cache) = &self.config.cache {
            if let Some(cached) = cache.get(&cache_key) {
                if let Ok(cached_downcasted) = cached.1.downcast::<T>() {
                    return Ok(cached_downcasted);
                }
            }
        }

        if self.key.is_rate_limited() {
            let time_till_reset = self.key.get_time_till_reset();
            match self.config.rate_limit_strategy {
                RateLimitStrategy::Delay => {
                    println!("Sleeping for {time_till_reset} seconds");
                    std::thread::sleep(Duration::from_secs(time_till_reset as u64));
                }
                RateLimitStrategy::Error => {
                    return Err(Error::RateLimit(self.key.time_till_reset));
                }
            }
        }

        let mut req = self
            .config
            .client
            .get(format!("https://api.hypixel.net/{}", endpoint.0))
            .query(&params)?;
        if endpoint.1 {
            req = req.header("API-Key", self.key.key.clone());
        }
        match req.send().await {
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

                if res_unwrap.status() == 200 {
                    match res_unwrap.body_json::<T>().await {
                        Ok(json) => {
                            let json_arc = Arc::new(json);
                            if let Some(cache) = &self.config.cache {
                                if let Some(hypixel_cache_ttl) =
                                    self.config.hypixel_cache_ttls.get(&endpoint)
                                {
                                    cache
                                        .insert(cache_key, (*hypixel_cache_ttl, json_arc.clone()))
                                        .await;
                                }
                            }
                            Ok(json_arc)
                        }
                        Err(err) => Err(Error::from(err)),
                    }
                } else {
                    match res_unwrap.body_json::<Value>().await {
                        Ok(json) => Err(Error::from((
                            res_unwrap.status(),
                            json.get("cause")
                                .and_then(serde_json::Value::as_str)
                                .unwrap_or("Unknown fail cause")
                                .to_string(),
                        ))),
                        Err(err) => Err(Error::from(err)),
                    }
                }
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    pub async fn simple_get<T>(&mut self, path: HypixelEndpoint) -> Result<Arc<T>, Error>
    where
        for<'a> T: DeserializeOwned + Send + Sync + 'a,
    {
        self.get(path, HashMap::new()).await
    }

    #[deprecated]
    pub async fn get_key(&mut self) -> Result<Arc<KeyResponse>, Error> {
        #[allow(deprecated)]
        self.simple_get(HypixelEndpoint::KEY).await
    }

    pub async fn get_boosters(&mut self) -> Result<Arc<BoostersResponse>, Error> {
        self.simple_get(HypixelEndpoint::BOOSTERS).await
    }

    pub async fn get_leaderboards(&mut self) -> Result<Arc<LeaderboardsResponse>, Error> {
        self.simple_get(HypixelEndpoint::LEADERBOARDS).await
    }

    pub async fn get_punishment_stats(&mut self) -> Result<Arc<PunishmentStatsResponse>, Error> {
        self.simple_get(HypixelEndpoint::PUNISHMENT_STATS).await
    }

    pub async fn get_player(&mut self, uuid: &str) -> Result<Arc<PlayerResponse>, Error> {
        self.get(HypixelEndpoint::PLAYER, self.to_params("uuid", uuid))
            .await
    }
    pub async fn get_guild_by_player(&mut self, player: &str) -> Result<Arc<GuildResponse>, Error> {
        self.get(HypixelEndpoint::GUILD, self.to_params("player", player))
            .await
    }

    pub async fn get_guild_by_name(&mut self, name: &str) -> Result<Arc<GuildResponse>, Error> {
        self.get(HypixelEndpoint::GUILD, self.to_params("name", name))
            .await
    }

    pub async fn get_guild_by_id(&mut self, id: &str) -> Result<Arc<GuildResponse>, Error> {
        self.get(HypixelEndpoint::GUILD, self.to_params("id", id))
            .await
    }

    pub async fn get_skyblock_auction_by_uuid(
        &mut self,
        uuid: &str,
    ) -> Result<Arc<SkyblockAuctionResponse>, Error> {
        self.get(
            HypixelEndpoint::SKYBLOCK_AUCTION,
            self.to_params("uuid", uuid),
        )
        .await
    }

    pub async fn get_skyblock_auction_by_player(
        &mut self,
        player: &str,
    ) -> Result<Arc<SkyblockAuctionResponse>, Error> {
        self.get(
            HypixelEndpoint::SKYBLOCK_AUCTION,
            self.to_params("player", player),
        )
        .await
    }

    pub async fn get_skyblock_auction_by_profile(
        &mut self,
        profile: &str,
    ) -> Result<Arc<SkyblockAuctionResponse>, Error> {
        self.get(
            HypixelEndpoint::SKYBLOCK_AUCTION,
            self.to_params("profile", profile),
        )
        .await
    }

    pub async fn get_counts(&mut self) -> Result<Arc<CountsResponse>, Error> {
        self.simple_get(HypixelEndpoint::COUNTS).await
    }

    pub async fn get_status(&mut self, uuid: &str) -> Result<Arc<StatusResponse>, Error> {
        self.get(HypixelEndpoint::STATUS, self.to_params("uuid", uuid))
            .await
    }

    pub async fn get_recent_games(
        &mut self,
        uuid: &str,
    ) -> Result<Arc<RecentGamesResponse>, Error> {
        self.get(HypixelEndpoint::RECENT_GAMES, self.to_params("uuid", uuid))
            .await
    }

    pub async fn get_skyblock_profiles(
        &mut self,
        uuid: &str,
    ) -> Result<Arc<SkyblockProfilesResponse>, Error> {
        self.get(
            HypixelEndpoint::SKYBLOCK_PROFILES,
            self.to_params("uuid", uuid),
        )
        .await
    }

    pub async fn get_skyblock_profile(
        &mut self,
        profile: &str,
    ) -> Result<Arc<SkyblockProfileResponse>, Error> {
        self.get(
            HypixelEndpoint::SKYBLOCK_PROFILE,
            self.to_params("profile", profile),
        )
        .await
    }

    pub async fn get_skyblock_bingo(
        &mut self,
        uuid: &str,
    ) -> Result<Arc<SkyblockBingoResponse>, Error> {
        self.get(
            HypixelEndpoint::SKYBLOCK_BINGO,
            self.to_params("uuid", uuid),
        )
        .await
    }

    pub async fn get_skyblock_news(&mut self) -> Result<Arc<SkyblockNewsResponse>, Error> {
        self.simple_get(HypixelEndpoint::SKYBLOCK_NEWS).await
    }

    pub async fn get_skyblock_auctions(
        &mut self,
        page: i64,
    ) -> Result<Arc<SkyblockAuctionsResponse>, Error> {
        self.get(
            HypixelEndpoint::SKYBLOCK_AUCTIONS,
            self.to_params("page", &page.to_string()),
        )
        .await
    }

    pub async fn get_skyblock_auctions_ended(
        &mut self,
    ) -> Result<Arc<SkyblockAuctionsEndedResponse>, Error> {
        self.simple_get(HypixelEndpoint::SKYBLOCK_AUCTIONS_ENDED)
            .await
    }

    pub async fn get_skyblock_bazaar(&mut self) -> Result<Arc<SkyblockBazaarResponse>, Error> {
        self.simple_get(HypixelEndpoint::SKYBLOCK_BAZAAR).await
    }

    pub async fn get_skyblock_fire_sales(
        &mut self,
    ) -> Result<Arc<SkyblockFireSalesResponse>, Error> {
        self.simple_get(HypixelEndpoint::SKYBLOCK_FIRESALES).await
    }

    pub async fn get_resources(&mut self, resource: HypixelEndpoint) -> Result<Arc<Value>, Error> {
        if !resource.0.starts_with("resources/") {
            Err(Error::UnknownResource)
        } else {
            self.simple_get(resource).await
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct HypixelEndpoint(&'static str, bool);

impl HypixelEndpoint {
    pub fn get_path(&self) -> String {
        self.0.to_string()
    }

    #[deprecated]
    pub const KEY: Self = Self("key", true);
    pub const BOOSTERS: Self = Self("boosters", true);
    pub const LEADERBOARDS: Self = Self("leaderboards", true);
    pub const PUNISHMENT_STATS: Self = Self("punishmentstats", true);
    pub const PLAYER: Self = Self("player", true);
    pub const GUILD: Self = Self("guild", true);
    pub const COUNTS: Self = Self("counts", true);
    pub const STATUS: Self = Self("status", true);
    pub const RECENT_GAMES: Self = Self("recentGames", true);
    pub const SKYBLOCK_PROFILES: Self = Self("skyblock/profiles", true);
    pub const SKYBLOCK_PROFILE: Self = Self("skyblock/profile", true);
    pub const SKYBLOCK_BINGO: Self = Self("skyblock/bingo", true);
    pub const SKYBLOCK_NEWS: Self = Self("skyblock/news", true);
    pub const SKYBLOCK_AUCTION: Self = Self("skyblock/auction", true);
    pub const SKYBLOCK_AUCTIONS: Self = Self("skyblock/auctions", false);
    pub const SKYBLOCK_AUCTIONS_ENDED: Self = Self("skyblock/auctions_ended", false);
    pub const SKYBLOCK_BAZAAR: Self = Self("skyblock/bazaar", false);
    pub const SKYBLOCK_FIRESALES: Self = Self("skyblock/firesales", false);
    pub const RESOURCES_GAMES: Self = Self("resources/games", false);
    pub const RESOURCES_ACHIEVEMENTS: Self = Self("resources/achievements", false);
    pub const RESOURCES_CHALLENGES: Self = Self("resources/challenges", false);
    pub const RESOURCES_QUESTS: Self = Self("resources/quests", false);
    pub const RESOURCES_GUILD_ACHIEVEMENTS: Self = Self("resources/guild/achievements", false);
    pub const RESOURCES_VANITY_PETS: Self = Self("resources/vanity/pets", false);
    pub const RESOURCES_VANITY_COMPANIONS: Self = Self("resources/vanity/companions", false);
    pub const RESOURCES_SKYBLOCK_COLLECTIONS: Self = Self("resources/skyblock/collections", false);
    pub const RESOURCES_SKYBLOCK_SKILLS: Self = Self("resources/skyblock/skills", false);
    pub const RESOURCES_SKYBLOCK_ITEMS: Self = Self("resources/skyblock/items", false);
    pub const RESOURCES_SKYBLOCK_ELECTION: Self = Self("resources/skyblock/election", false);
    pub const RESOURCES_SKYBLOCK_BINGO: Self = Self("resources/skyblock/bingo", false);
}

struct Key {
    pub key: String,
    remaining_limit: i64,
    time_till_reset: i64,
    time: i64,
}

impl Key {
    pub fn new(key: impl Into<String>) -> Key {
        Key {
            key: key.into(),
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

#[derive(Default)]
pub enum RateLimitStrategy {
    #[default]
    Delay,
    Error,
}

pub struct Config {
    pub client: Client,
    pub minecraft_api_type: ApiType,
    pub rate_limit_strategy: RateLimitStrategy,
    pub uuid_to_username_cache: Option<Cache<String, String>>,
    pub cache: Option<Cache<String, (Duration, Arc<dyn Any + Send + Sync>)>>,
    pub hypixel_cache_ttls: HashMap<HypixelEndpoint, Duration>,
}

#[derive(Default)]
pub struct ConfigBuilder {
    client: Option<Client>,
    minecraft_api_type: Option<ApiType>,
    rate_limit_strategy: Option<RateLimitStrategy>,
    minecraft_cache_ttl: Option<Duration>,
    hypixel_cache_ttls: HashMap<HypixelEndpoint, Duration>,
}

impl ConfigBuilder {
    /// Set the Surf client to use for HTTP requests. Defaults to
    pub fn client(mut self, client: Client) -> ConfigBuilder {
        self.client = Some(client);
        self
    }

    /// Set API for username and uuid conversions. Defaults to `ApiType::Mojang`.
    pub fn minecraft_api_type(mut self, minecraft_api_type: ApiType) -> ConfigBuilder {
        self.minecraft_api_type = Some(minecraft_api_type);
        self
    }

    /// Set how Hypixle API rate limits should be handled. Defaults to `RateLimitStrategy::Delay`.
    pub fn rate_limit_strategy(mut self, rate_limit_strategy: RateLimitStrategy) -> ConfigBuilder {
        self.rate_limit_strategy = Some(rate_limit_strategy);
        self
    }

    /// Set the time to live for uuid and username caches. A TTL must be set to enable caching.
    pub fn minecraft_cache_ttl(mut self, minecraft_cache_ttl: Duration) -> ConfigBuilder {
        self.minecraft_cache_ttl = Some(minecraft_cache_ttl);
        self
    }

    /// Set the time to live for Hypixel API caching. Only endpoints with a TTL set will be cached.
    pub fn add_hypixel_cache_ttl(
        mut self,
        endpoint: HypixelEndpoint,
        ttl: Duration,
    ) -> ConfigBuilder {
        self.hypixel_cache_ttls.insert(endpoint, ttl);
        self
    }
}

impl From<ConfigBuilder> for Config {
    fn from(c: ConfigBuilder) -> Self {
        Config {
            client: c.client.unwrap_or_default(),
            minecraft_api_type: c.minecraft_api_type.unwrap_or_default(),
            rate_limit_strategy: c.rate_limit_strategy.unwrap_or_default(),
            uuid_to_username_cache: c
                .minecraft_cache_ttl
                .map(|ttl| Cache::builder().time_to_live(ttl).build()),
            cache: if !c.hypixel_cache_ttls.is_empty() {
                Some(Cache::builder().expire_after(HypixelCacheExpiry).build())
            } else {
                None
            },
            hypixel_cache_ttls: c.hypixel_cache_ttls,
        }
    }
}

struct HypixelCacheExpiry;

impl Expiry<String, (Duration, Arc<dyn Any + Send + Sync>)> for HypixelCacheExpiry {
    fn expire_after_create(
        &self,
        _key: &String,
        value: &(Duration, Arc<dyn Any + Send + Sync>),
        _current_time: Instant,
    ) -> Option<Duration> {
        Some(value.0)
    }
}
