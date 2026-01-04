pub mod language_item;
pub mod sortie_data;

use std::{fs, io, path::Path};

use serde::{Deserialize, Deserializer, de::DeserializeOwned};

use crate::{
    core::InternalPath,
    wfcd_worldstate_data::{language_item::LanguageItemMap, sortie_data::SortieData},
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
    Ok(serde_json::from_str(
        fs::read_to_string(data_dir.as_ref().join(file.as_ref().with_extension("json")))?.as_str(),
    )?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldstateData {
    pub language_items: LanguageItemMap,
    pub sortie_data: SortieData,
}

impl WorldstateData {
    pub fn new(data_dir: impl AsRef<Path>) -> Result<Self, WorldstateDataError> {
        let data_dir = data_dir.as_ref();
        Ok(Self {
            language_items: init(data_dir, "languages")?,
            sortie_data: init(data_dir, "sortieData")?,
        })
    }
}

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<InternalPath>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let has_content = !s.is_empty();

    Ok(has_content.then(|| InternalPath::from(s)))
}
