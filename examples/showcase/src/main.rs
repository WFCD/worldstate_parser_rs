use std::{fs, path::Path};

use worldstate_parser::{
    PathContext,
    default_context_provider::DefaultContextProvider,
    worldstate,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let world_state_str = reqwest::get("https://api.warframe.com/cdn/worldState.php")
        .await?
        .text()
        .await?;

    let world_state = worldstate::from_str(
        &world_state_str,
        DefaultContextProvider,
        PathContext {
            data_dir: Path::new("data/"),
            drops_dir: Path::new("drops/"),
            assets_dir: Path::new("assets/"),
        },
    )
    .await?;

    fs::write(
        "showcase/worldstate_parsed.json",
        serde_json::to_string_pretty(&world_state)?,
    )?;

    Ok(())
}
