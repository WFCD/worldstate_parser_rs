use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolve_with},
    target_types::worldstate_types::daily_deal::DailyDeal,
    worldstate_model::deserialize_mongo_date,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DailyDealUnmapped {
    pub store_item: InternalPath<resolve_with::LanguageItems>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub discount: u32,

    pub original_price: u32,

    pub sale_price: u32,

    pub amount_total: u32,

    pub amount_sold: u32,
}

impl Resolve<ContextRef<'_>> for DailyDealUnmapped {
    type Output = DailyDeal;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        DailyDeal {
            item: self.store_item.resolve(ctx),
            activation: self.activation,
            expiry: self.expiry,
            discount_percentage: self.discount,
            original_price: self.original_price,
            sale_price: self.sale_price,
            stock: self.amount_total,
            amount_sold: self.amount_sold,
        }
    }
}
