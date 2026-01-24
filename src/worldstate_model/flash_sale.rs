use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{InternalPath, Resolve, resolve_with},
    target_types::worldstate::flash_sale::FlashSale,
    worldstate_model::deserialize_mongo_date,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FlashSaleUnmapped {
    pub type_name: InternalPath<resolve_with::LastSegment>,

    #[serde(default, rename = "ShownInMarket")]
    pub is_shown_in_market: bool,

    #[serde(default, rename = "HideFromMarket")]
    pub is_hidden_from_market: bool,

    #[serde(rename = "StartDate", deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(rename = "EndDate", deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub regular_override: Option<i64>,
    pub premium_override: Option<i64>,

    pub discount: Option<i64>,

    #[serde(default)]
    pub daily_sale_generated: bool,

    #[serde(default)]
    pub is_featured: bool,

    #[serde(default)]
    pub is_popular: bool,
}

impl Resolve<()> for FlashSaleUnmapped {
    type Output = FlashSale;

    fn resolve(self, _ctx: ()) -> Self::Output {
        FlashSale {
            item: self.type_name.resolve(()),
            is_shown_in_market: self.is_shown_in_market,
            is_hidden_from_market: self.is_hidden_from_market,
            activation: self.activation,
            expiry: self.expiry,
            regular_override: self.regular_override,
            premium_override: self.premium_override,
            discount: self.discount,
            daily_sale_generated: self.daily_sale_generated,
            is_featured: self.is_featured,
            is_popular: self.is_popular,
        }
    }
}
