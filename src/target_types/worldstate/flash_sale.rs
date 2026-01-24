use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FlashSale {
    pub item: String,

    pub is_shown_in_market: bool,

    pub is_hidden_from_market: bool,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub regular_override: Option<i64>,

    pub premium_override: Option<i64>,

    pub discount: Option<i64>,

    pub daily_sale_generated: bool,

    pub is_featured: bool,

    pub is_popular: bool,
}
