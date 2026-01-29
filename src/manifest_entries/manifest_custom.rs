use serde::{Deserialize, Serialize};

use crate::{core::Resolve, target_types::customs_entry::CustomsEntry};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestCustom {
    pub unique_name: String,

    pub name: String,

    pub codex_secret: bool,

    pub description: Option<String>,

    #[serde(default)]
    pub exclude_from_codex: bool,
}

impl Resolve<()> for ManifestCustom {
    type Output = CustomsEntry;

    fn resolve(self, _ctx: ()) -> Self::Output {
        CustomsEntry {
            name: self.name,
            codex_secret: self.codex_secret,
            description: self.description,
            exclude_from_codex: self.exclude_from_codex,
        }
    }
}
