use std::collections::HashMap;

use serde::Deserialize;

pub type LanguageItemMap = HashMap<String, LanguageItem>;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct LanguageItem {
    pub value: String,
    pub desc: Option<String>,
}
