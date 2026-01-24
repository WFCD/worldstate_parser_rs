pub mod bounty_rewards;
pub mod language_item;
pub mod sortie_data;

use std::{collections::HashMap, fs, io, path::Path};

use serde::de::DeserializeOwned;

use crate::{
    core::TranslationLanguage,
    wfcd_data::{
        bounty_rewards::BountyRewards,
        language_item::LanguageItemMap,
        sortie_data::SortieData,
    },
};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum WorldstateDataError {
    Io(#[from] io::Error),
    Deserialize(#[from] serde_json::Error),
}

fn init<T: DeserializeOwned>(
    language: TranslationLanguage,
    data_dir: impl AsRef<Path>,
    file: impl AsRef<Path>,
    translations_available: bool,
) -> Result<T, WorldstateDataError> {
    let mut path = data_dir.as_ref().to_owned();

    if translations_available && let Some(code) = language.as_code() {
        path = path.join(code);
    }

    path = path.join(file.as_ref().with_extension("json"));

    Ok(serde_json::from_str(fs::read_to_string(path)?.as_str())?)
}

#[derive(Debug, Clone, PartialEq)]
pub struct WorldstateData {
    pub language_items: LanguageItemMap,
    pub sortie_data: SortieData,
    pub bounty_rewards: BountyRewards,
    pub hubs: HashMap<String, String>,
}

impl WorldstateData {
    pub fn new(
        language: TranslationLanguage,
        data_dir: impl AsRef<Path>,
        drop_dir: impl AsRef<Path>,
        assets_dir: impl AsRef<Path>,
    ) -> Result<Self, WorldstateDataError> {
        let data_dir = data_dir.as_ref();

        Ok(Self {
            language_items: init(language, data_dir, "languages", true)?,
            sortie_data: init(language, data_dir, "sortieData", true)?,
            bounty_rewards: init(language, drop_dir, "data", false)?,
            hubs: init(language, assets_dir, "relays", false)?,
        })
    }
}
