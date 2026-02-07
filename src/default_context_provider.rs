use std::{collections::HashMap, io, path::Path, string::FromUtf8Error, sync::LazyLock};

use regex::Regex;
use reqwest::get;
use serde::de::DeserializeOwned;
use tokio::fs;
use tracing::debug;

use crate::{
    ContextProvider,
    core::Context,
    custom_maps::CustomMaps,
    manifest_entries::manifest_node::ManifestNode,
    manifests::{self, ExportRegions, Exports, MissingManifestKeyError},
    wfcd_data::{WorldstateData, language_item::LanguageItemMap},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PathContext<'a> {
    pub data_dir: &'a Path,
    pub drops_dir: &'a Path,
    pub assets_dir: &'a Path,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum WorldstateDataError {
    Io(#[from] io::Error),
    Deserialize(#[from] serde_json::Error),
}

const CACHE_DIR: &str = "./cache";

static SOLNODES_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(.*) \(.*\)").unwrap());

async fn init<T: DeserializeOwned>(
    data_dir: impl AsRef<Path>,
    file: impl AsRef<Path>,
) -> Result<T, WorldstateDataError> {
    let path = data_dir.as_ref().join(file.as_ref().with_extension("json"));

    Ok(serde_json::from_str(
        fs::read_to_string(path).await?.as_str(),
    )?)
}

async fn create_worldstate_data(
    PathContext {
        data_dir,
        assets_dir,
        drops_dir,
    }: PathContext<'_>,
) -> Result<WorldstateData, WorldstateDataError> {
    let mut language_items: LanguageItemMap = init(data_dir, "languages").await?;
    let archimedea_ext: LanguageItemMap = init(assets_dir, "languageItemsExt").await?;
    language_items.extend(archimedea_ext);

    #[derive(serde::Deserialize)]
    pub struct SolNodeItem {
        value: String,
    }

    let sol_nodes: HashMap<String, SolNodeItem> = init(data_dir, "solNodes").await?;

    let hubs = sol_nodes
        .into_iter()
        .filter_map(|(key, value)| {
            if !key.contains("HUB") {
                return None;
            }

            let relay_name = SOLNODES_REGEX
                .captures(&value.value)
                .and_then(|cap| cap.get(1))
                .map(|r#match| r#match.as_str().to_owned())
                .unwrap_or_else(|| value.value);

            Some((key, relay_name))
        })
        .collect();

    Ok(WorldstateData {
        language_items,
        sortie_data: init(data_dir, "sortieData").await?,
        rewards: init(drops_dir, "data").await?,
        hubs,
        archon_hunt_rewards: init(assets_dir, "archonHuntRewards").await?,
        archon_shards_store_item: init(assets_dir, "archonShardsStoreItem").await?,
    })
}

#[derive(Debug, Clone, Copy)]
pub struct DefaultContextProvider<'a>(pub PathContext<'a>, pub &'a reqwest::Client);

impl ContextProvider for DefaultContextProvider<'_> {
    type Err = DefaultContextProviderError;

    async fn get_ctx(&self) -> Result<Context, Self::Err> {
        let exports = get_export(&self.0.assets_dir.join("crewBattleNodes.json"), self.1).await?;
        let custom_maps = CustomMaps::new(&exports);
        let worldstate_data = create_worldstate_data(self.0).await?;

        Ok(Context {
            custom_maps,
            exports,
            worldstate_data,
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum DefaultContextProviderError {
    Io(#[from] io::Error),
    Json(#[from] serde_json::Error),
    Reqwest(#[from] reqwest::Error),
    MissingKey(#[from] MissingManifestKeyError),
    Utf8Error(#[from] FromUtf8Error),
    DataError(#[from] WorldstateDataError),
}

async fn get_from_cache_or_fetch<T: DeserializeOwned>(
    manifest: &str,
) -> Result<T, DefaultContextProviderError> {
    fs::create_dir_all(CACHE_DIR).await?;

    let path = Path::new(CACHE_DIR)
        .join(manifest)
        .with_added_extension("json");

    if let Ok(cached) = fs::read_to_string(&path).await {
        debug!("Using cache at {}", path.to_str().unwrap());
        return Ok(serde_json::from_str(&cached)?);
    }

    let item_json = get(format!(
        "http://content.warframe.com/PublicExport/Manifest/{manifest}"
    ))
    .await?
    .text()
    .await?;

    let mut files = fs::read_dir(CACHE_DIR).await?;

    while let Ok(Some(file)) = files.next_entry().await {
        let file_name = file.file_name().into_string().unwrap();

        if file_name.starts_with(
            manifest
                .split_once('!')
                .expect("Manifest should be valid")
                .0,
        ) {
            fs::remove_file(file_name).await?;
        }
    }

    fs::write(path, &item_json).await?;

    Ok(serde_json::from_str(&item_json)?)
}

async fn get_export(
    ctx: &Path,
    client: &reqwest::Client,
) -> Result<Exports, DefaultContextProviderError> {
    let file = client
        .get("https://origin.warframe.com/PublicExport/index_en.txt.lzma")
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();

    let mut buffer: Vec<u8> = Vec::new();

    lzma_rs::lzma_decompress(&mut file.as_slice(), &mut buffer).unwrap();

    let data = String::from_utf8(buffer)?;

    let export: manifests::PublicExportIndex = data.parse()?;

    let crew_battle_nodes_json: Vec<ManifestNode> =
        serde_json::from_str(&fs::read_to_string(ctx).await?)?;

    let mut export_regions: ExportRegions = get_from_cache_or_fetch(&export.regions).await?;

    export_regions.export_regions.extend(crew_battle_nodes_json);

    let exports = Exports {
        export_regions,
        export_relic_arcane: get_from_cache_or_fetch(&export.relic_arcane).await?,
        export_customs: get_from_cache_or_fetch(&export.customs).await?,
    };

    Ok(exports)
}
