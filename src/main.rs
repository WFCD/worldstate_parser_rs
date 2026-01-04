pub mod core;
pub mod custom_maps;
pub mod manifest_entries;
pub mod manifests;
pub mod target_types;
pub mod wfcd_worldstate_data;
pub mod world_state;
pub mod worldstate_model;

use std::{error::Error, fs, path::Path};

use reqwest::blocking::get;
use serde::de::DeserializeOwned;

use crate::{
    core::Context,
    custom_maps::CustomMaps,
    manifests::Exports,
    wfcd_worldstate_data::WorldstateData,
    world_state::WorldStateUnmapped,
};

type BoxDynError = Box<dyn Error>;

pub const CACHE_DIR: &str = "./cache";

fn get_from_cache_or_fetch<T: DeserializeOwned>(manifest: &str) -> Result<T, BoxDynError> {
    let path = Path::new(CACHE_DIR)
        .join(manifest)
        .with_added_extension("json");

    if let Ok(cached) = fs::read_to_string(&path) {
        println!("Using cache at {}", path.to_str().unwrap());
        return Ok(serde_json::from_str(&cached)?);
    }

    let item_json = get(format!(
        "http://content.warframe.com/PublicExport/Manifest/{}",
        manifest
    ))?
    .text()?;

    for file in fs::read_dir(CACHE_DIR)? {
        let file_name = file?.file_name().into_string().unwrap();

        if file_name.starts_with(
            manifest
                .split_once('!')
                .expect("Manifest should be valid")
                .0,
        ) {
            fs::remove_file(file_name)?;
        }
    }

    fs::write(path, &item_json)?;

    Ok(serde_json::from_str(&item_json)?)
}

fn get_export() -> Result<Exports, BoxDynError> {
    let file = get("https://origin.warframe.com/PublicExport/index_en.txt.lzma")?
        .bytes()?
        .to_vec();

    let mut buffer: Vec<u8> = Vec::new();

    lzma_rs::lzma_decompress(&mut file.as_slice(), &mut buffer).unwrap();

    let data = String::from_utf8(buffer)?;

    let export: manifests::PublicExportIndex = data.parse()?;

    let exports = Exports {
        export_regions: get_from_cache_or_fetch(&export.regions)?,
    };

    Ok(exports)
}

fn main() -> Result<(), BoxDynError> {
    let exports = get_export()?;
    let custom_maps = CustomMaps::from_exports(&exports);
    let worldstate_data = WorldstateData::new("data/")?;

    let ctx = Context {
        custom_maps: &custom_maps,
        exports: &exports,
        worldstate_data: &worldstate_data,
    };

    let fissures =
        serde_json::from_str::<WorldStateUnmapped>(&fs::read_to_string("worldstate.json")?)?
            .map_worldstate(ctx)
            .ok_or("Failed to map worldstate")?;

    fs::write(
        "worldstate_parsed.json",
        serde_json::to_string_pretty(&fissures)?,
    )?;

    Ok(())
}
