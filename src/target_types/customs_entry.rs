use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CustomsEntry {
    pub name: String,

    pub codex_secret: bool,

    pub description: Option<String>,

    pub exclude_from_codex: bool,
}
