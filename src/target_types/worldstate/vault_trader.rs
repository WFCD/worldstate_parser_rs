use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VaultTrader {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub initial_start_date: DateTime<Utc>,

    pub node: String,

    pub manifest: Vec<VaultTraderManifest>,

    pub evergreen_manifest: Vec<VaultTraderManifest>,

    pub schedule_info: Vec<ScheduleInfo>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Currency {
    Aya,
    #[serde(rename = "Regal Aya")]
    RegalAya,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Price {
    pub currency: Currency,
    pub amount: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct VaultTraderManifest {
    pub item_type: String,

    pub price: Price,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleInfo {
    pub expiry: DateTime<Utc>,

    pub preview_hidden_until: Option<DateTime<Utc>>,

    pub featured_item: String,
}
