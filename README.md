# rs-pixel &emsp; [![Build Status]][actions] [![Latest Version]][crates.io] [![Discord]][discord link]

[actions]: https://github.com/kr45732/rs-pixel/actions?query=branch%3Amain
[Build Status]: https://img.shields.io/github/workflow/status/kr45732/rs-pixel/Rust/main
[Latest Version]: https://img.shields.io/crates/v/rs-pixel.svg
[crates.io]: https://crates.io/crates/rs-pixel
[Discord]: https://img.shields.io/discord/796790757947867156?color=4166f5&label=discord&style=flat-square
[discord link]: https://dsc.gg/skyblock-plus

A complete, rate-limiting, asynchronous Rust implementation of the Hypixel Public API with extensive SkyBlock support.

```toml
[dependencies]
rs-pixel = "0.1.0"
```

# Getting started
You will need a Hypixel Api Key to access most endpoints ([official documentation](https://api.hypixel.net/)).

## Creating an Instance
Use the default configuration
```rust
let mut api = RsPixel::new("API KEY").await.unwrap();
```
Or configure the client, Minecraft username/UUID API type, and the rate limit strategy
```rust
let config = ConfigBuilder::default()
    .client(surf::Config::new().try_into().unwrap())
    .minecraft_api_type(ApiType::PlayerDb)
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

Print a skyblock player's statistics
```rust
let response = api.get_skyblock_profiles_by_name("USERNAME").await.unwrap();
let profile = response.get_selected_profile().unwrap();

println!(
    "Enderman Slayer XP: {}\nCombat Skill Level: {}\nCatacombs LeveL: {}",
    profile.get_slayer("enderman").unwrap().current_exp,
    profile.get_skill("combat").unwrap().level,
    profile.get_catacombs().unwrap().level
);
```

Print a skyblock player's inventory contents (NBT parsed to JSON)
```rust
let response = api.get_skyblock_profiles_by_uuid("uuid").await.unwrap();
let profile = response.get_selected_profile().unwrap();

println!("Inventory Contents:  {}", profile.get_inventory().unwrap());
```

Get the first page and print the first auction
```rust
let response = api.get_skyblock_auctions(0).await.unwrap();
let auction = response.auctions.get(0).unwrap();

println!(
    "The starting bid for a {} is {} coins",
    auction.item_name, auction.starting_bid
);
```

# Todo
- Documentation
- More examples

# License & Contributing
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.