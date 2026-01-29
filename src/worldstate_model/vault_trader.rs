use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolvable_string::ResolvableString, resolve_with},
    target_types::worldstate::vault_trader::{
        Currency,
        Price,
        ScheduleInfo,
        VaultTrader,
        VaultTraderManifest,
    },
    worldstate_model::{Id, deserialize_mongo_date, deserialize_mongo_date_opt},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VaultTraderUnmapped {
    #[serde(rename = "_id")]
    id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    expiry: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    initial_start_date: DateTime<Utc>,

    node: ResolvableString<resolve_with::Hubs>,

    manifest: Vec<VaultTraderManifestUnmapped>,

    evergreen_manifest: Vec<VaultTraderManifestUnmapped>,

    schedule_info: Vec<ScheduleInfoUnmapped>,
}

impl Resolve<ContextRef<'_>> for VaultTraderUnmapped {
    type Output = VaultTrader;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        VaultTrader {
            id: self.id.oid,
            activation: self.activation,
            expiry: self.expiry,
            initial_start_date: self.initial_start_date,
            node: self.node.resolve(ctx),
            manifest: self.manifest.resolve(ctx),
            evergreen_manifest: self.evergreen_manifest.resolve(ctx),
            schedule_info: self.schedule_info.resolve(ctx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PriceUnmapped {
    #[serde(rename = "PrimePrice")]
    RegalAya(u64),

    #[serde(rename = "RegularPrice")]
    Aya(u64),
}

impl Resolve<()> for PriceUnmapped {
    type Output = Price;

    fn resolve(self, _ctx: ()) -> Self::Output {
        match self {
            PriceUnmapped::RegalAya(amount) => Price {
                currency: Currency::RegalAya,
                amount,
            },
            PriceUnmapped::Aya(amount) => Price {
                currency: Currency::Aya,
                amount,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VaultTraderManifestUnmapped {
    item_type: InternalPath<resolve_with::VaultTraderItem>,

    #[serde(flatten)]
    price: PriceUnmapped,
}

impl Resolve<ContextRef<'_>> for VaultTraderManifestUnmapped {
    type Output = VaultTraderManifest;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        VaultTraderManifest {
            item_type: self.item_type.resolve(ctx),
            price: self.price.resolve(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScheduleInfoUnmapped {
    #[serde(deserialize_with = "deserialize_mongo_date")]
    expiry: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date_opt", default)]
    preview_hidden_until: Option<DateTime<Utc>>,

    featured_item: InternalPath<resolve_with::VaultTraderItem>,
}

impl Resolve<ContextRef<'_>> for ScheduleInfoUnmapped {
    type Output = ScheduleInfo;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        ScheduleInfo {
            expiry: self.expiry,
            preview_hidden_until: self.preview_hidden_until,
            featured_item: self.featured_item.resolve(ctx),
        }
    }
}
