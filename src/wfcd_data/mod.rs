pub mod bounty_rewards;
pub mod language_item;
pub mod sortie_data;

use std::{collections::HashMap, fs, io, path::Path};

use serde::de::DeserializeOwned;

use crate::wfcd_data::{
    bounty_rewards::{BountyRewards, DropItem},
    language_item::LanguageItemMap,
    sortie_data::SortieData,
};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum WorldstateDataError {
    Io(#[from] io::Error),
    Deserialize(#[from] serde_json::Error),
}

fn init<T: DeserializeOwned>(
    data_dir: impl AsRef<Path>,
    file: impl AsRef<Path>,
) -> Result<T, WorldstateDataError> {
    let path = data_dir.as_ref().join(file.as_ref().with_extension("json"));

    Ok(serde_json::from_str(fs::read_to_string(path)?.as_str())?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldstateData {
    pub language_items: LanguageItemMap,
    pub sortie_data: SortieData,
    pub rewards: BountyRewards,
    pub archon_hunt_rewards: Vec<DropItem>,
    pub hubs: HashMap<String, String>,
}

impl WorldstateData {
    pub fn new(
        data_dir: impl AsRef<Path>,
        drop_dir: impl AsRef<Path>,
        assets_dir: impl AsRef<Path>,
        manual_assets_dir: impl AsRef<Path>,
    ) -> Result<Self, WorldstateDataError> {
        let data_dir = data_dir.as_ref();

        Ok(Self {
            language_items: init(data_dir, "languages")?,
            sortie_data: init(data_dir, "sortieData")?,
            rewards: init(drop_dir, "data")?,
            hubs: init(assets_dir, "relays")?,
            archon_hunt_rewards: init(manual_assets_dir, "archon_hunt_rewards")?,
        })
    }
}
