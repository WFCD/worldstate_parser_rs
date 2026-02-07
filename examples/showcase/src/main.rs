use std::{fs, path::Path};

use tracing_subscriber::filter::LevelFilter;
use worldstate_parser::{
    WorldState,
    default_context_provider::{DefaultContextProvider, PathContext},
    default_data_fetcher::CacheStrategy,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();

    worldstate_parser::default_data_fetcher::fetch_all(CacheStrategy::Basic).await?;

    let world_state_str = reqwest::get("https://api.warframe.com/cdn/worldState.php")
        .await?
        .text()
        .await?;

    let provider = DefaultContextProvider(
        PathContext {
            data_dir: Path::new("data/"),
            drops_dir: Path::new("drops/"),
            assets_dir: Path::new("assets/"),
        },
        &reqwest::Client::new(),
    );

    let world_state = WorldState::from_str(&world_state_str, provider).await?;

    fs::write(
        "worldstate_parsed.json",
        serde_json::to_string_pretty(&world_state)?,
    )?;

    Ok(())
}
