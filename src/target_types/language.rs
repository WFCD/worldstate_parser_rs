use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    English,
    French,
    Italian,
    German,
    Spanish,
    Portuguese,
    Russian,
    Polish,
    Ukrainian,
    Turkish,
    Japanese,
    ChineseSimplified,
    Korean,
    ChineseTraditional,
    Thai,
}

impl Language {
    pub fn as_language_code(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::French => "fr",
            Language::Italian => "it",
            Language::German => "de",
            Language::Spanish => "es",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Polish => "pl",
            Language::Ukrainian => "uk",
            Language::Turkish => "tr",
            Language::Japanese => "ja",
            Language::ChineseSimplified => "zh",
            Language::Korean => "ko",
            Language::ChineseTraditional => "tc",
            Language::Thai => "th",
        }
    }
}
