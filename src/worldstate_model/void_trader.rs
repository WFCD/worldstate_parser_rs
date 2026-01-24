use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolvable_string::ResolvableString, resolve_with},
    target_types::worldstate::void_trader::{ShopItem, VoidTrader},
    worldstate_model::{Id, deserialize_mongo_date},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VoidTraderUnmapped {
    #[serde(rename = "_id")]
    pub id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub character: String,

    pub node: ResolvableString<resolve_with::Hubs>,

    #[serde(rename = "Manifest")]
    pub shop: Vec<ShopItemUnmapped>,
}

impl Resolve<ContextRef<'_>> for VoidTraderUnmapped {
    type Output = VoidTrader;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        VoidTrader {
            id: self.id.oid,
            activation: self.activation,
            expiry: self.expiry,
            character: self.character,
            node: self.node.resolve(ctx),
            shop: self.shop.resolve(ctx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShopItemUnmapped {
    pub item_type: InternalPath<resolve_with::LanguageItems>,

    pub prime_price: u64,

    pub regular_price: u64,

    /// Limited per-user. Such as Baro's loot box
    pub limit: Option<u64>,
}

impl Resolve<ContextRef<'_>> for ShopItemUnmapped {
    type Output = ShopItem;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        ShopItem {
            item_type: self.item_type.resolve(ctx),
            prime_price: self.prime_price,
            regular_price: self.regular_price,
            limit: self.limit,
        }
    }
}
