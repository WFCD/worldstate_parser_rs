use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, Resolve, sol_node::SolNode},
    target_types::worldstate_types::void_storm::VoidStorm,
    worldstate_model::{Id, deserialize_mongo_date, fissure::TierUnmapped},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VoidStormUnmapped {
    #[serde(rename = "_id")]
    id: Id,

    node: SolNode,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    expiry: DateTime<Utc>,

    active_mission_tier: TierUnmapped,
}

impl Resolve<ContextRef<'_>> for VoidStormUnmapped {
    type Output = VoidStorm;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        VoidStorm {
            id: self.id.oid,
            node: self.node.resolve(ctx).cloned(),
            activation: self.activation,
            expiry: self.expiry,
            tier: self.active_mission_tier.resolve(()),
        }
    }
}
