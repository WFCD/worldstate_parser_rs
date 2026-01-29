use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolvable_string::ResolvableString, resolve_with},
    target_types::worldstate::void_trader::{
        ArrivedVoidTrader,
        DepartedVoidTrader,
        ShopItem,
        VoidTraderState,
    },
    worldstate_model::{Id, deserialize_mongo_date},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoidTraderStateUnmapped {
    Arrived(ArrivedVoidTraderUnmapped),
    Departed(DepartedVaultTraderUnmapped),
}

impl Resolve<ContextRef<'_>> for VoidTraderStateUnmapped {
    type Output = VoidTraderState;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        match self {
            VoidTraderStateUnmapped::Arrived(arrived_void_trader_unmapped) => {
                VoidTraderState::Arrived(arrived_void_trader_unmapped.resolve(ctx))
            },
            VoidTraderStateUnmapped::Departed(departed_vault_trader_unmapped) => {
                VoidTraderState::Departed(departed_vault_trader_unmapped.resolve(ctx))
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DepartedVaultTraderUnmapped {
    #[serde(rename = "_id")]
    pub id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub character: String,

    pub node: ResolvableString<resolve_with::Hubs>,
}

impl Resolve<ContextRef<'_>> for DepartedVaultTraderUnmapped {
    type Output = DepartedVoidTrader;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        DepartedVoidTrader {
            id: self.id.oid,
            activation: self.activation,
            expiry: self.expiry,
            character: self.character,
            next_location: self.node.resolve(ctx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArrivedVoidTraderUnmapped {
    #[serde(rename = "_id")]
    pub id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub node: ResolvableString<resolve_with::Hubs>,

    pub character: String,

    #[serde(rename = "Manifest")]
    pub shop: Vec<ShopItemUnmapped>,
}

impl Resolve<ContextRef<'_>> for ArrivedVoidTraderUnmapped {
    type Output = ArrivedVoidTrader;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        ArrivedVoidTrader {
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
