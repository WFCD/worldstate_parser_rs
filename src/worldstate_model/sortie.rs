use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{
    core::{
        Context,
        InternalPath,
        Resolve,
        resolvable_string::ResolvableString,
        resolve_with,
        sol_node::SolNode,
    },
    target_types::worldstate::sortie::{Sortie, SortieVariant},
    worldstate_model::{Id, WorldstateMissionType, deserialize_mongo_date},
};

impl Resolve<Context<'_>> for SortieUnmapped {
    type Output = Sortie;

    fn resolve(self, ctx: Context<'_>) -> Self::Output {
        Sortie {
            activation: self.activation,
            expiry: self.expiry,
            boss: self.boss.resolve(ctx).cloned().map(|boss| boss.name),
            extra_drops: self.extra_drops,
            id: self.id.oid,
            reward: self.reward.resolve(()),
            seed: self.seed,
            twitter: self.twitter,
            variants: self.variants.resolve(ctx),
        }
    }
}

impl Resolve<Context<'_>> for SortieVariantUnmapped {
    type Output = SortieVariant;

    fn resolve(self, ctx: Context<'_>) -> Self::Output {
        SortieVariant {
            mission_type: self.mission_type.resolve(()),
            modifier_type: self.modifier_type.resolve(ctx),
            node: self.node.resolve(ctx).cloned(),
            tileset: self.tileset,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SortieUnmapped {
    #[serde(rename = "_id")]
    pub id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub reward: InternalPath<resolve_with::LastSegment>,

    pub seed: u64,

    pub boss: ResolvableString<resolve_with::sortie::Boss>,

    #[serde(default)]
    pub extra_drops: Vec<serde_json::Value>,

    pub variants: Vec<SortieVariantUnmapped>,

    pub twitter: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortieVariantUnmapped {
    pub mission_type: WorldstateMissionType,

    pub modifier_type: ResolvableString<resolve_with::sortie::Modifier>,

    pub node: SolNode,

    pub tileset: String,
}
