# rs-pixel
Asynchronous Rust implementation of the Hypixel Public API

# Getting started
You will need a Hypixel Api Key to access most endpoints ([official documentation](https://api.hypixel.net/))

## Creating an instance
Create a default instance
```rust
let mut api = RsPixel::new("API KEY").await.unwrap();
```
Or configure the client, minecraft username/uuid api type, and the rate limit strategy
```rust
let config = Config::new()
    .client(surf::Config::new().try_into().unwrap())
    .minecraft_api_type(MinecraftApiType::PlayerDb)
    .rate_limit_strategy(RateLimitStrategy::Error)
    .into();
let mut api = RsPixel::from_config("API KEY", config).await.unwrap();
```

## Examples
Print a player's name and rank
```rust
let response = api.get_player_by_username("USERNAME").await.unwrap();

println!(
    "{} has the {} rank",
    response.get_name().unwrap(),
    response.get_rank()
);
```

Get a skyblock player
```rust
let response = api.get_skyblock_profiles_by_name("USERNAME").await.unwrap();
let profile = response.get_last_played_profile().unwrap();

println!(
    "Enderman Slayer XP: {}\nCombat Skill Level: {}\nCatacombs LeveL: {}",
    profile.get_slayer("enderman").unwrap().current_exp,
    profile.get_skill("combat").unwrap().level,
    profile.get_catacombs().unwrap().level
);
```

Parse skyblock inventory nbt to json
```rust
let response = api.get_skyblock_profiles_by_name("USERNAME").await.unwrap();
let profile = response.get_last_played_profile().unwrap();

println!("Inventory Contents:  {}", profile.get_inventory().unwrap());
```

Get a skyblock auctions
```rust
let response = api.get_skyblock_auctions(0).await.unwrap();
let auction = response.auctions.get(0).unwrap();

println!(
    "The starting bid for a {} is {} coins",
    auction.item_name, auction.starting_bid
);
```