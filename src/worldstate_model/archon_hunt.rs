use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        ContextRef,
        InternalPath,
        Resolve,
        resolvable_string::ResolvableString,
        resolve_with,
        sol_node::SolNode,
    },
    target_types::worldstate::archon_hunt::{ArchonHunt, Mission},
    worldstate_model::{Id, WorldstateMissionType, deserialize_mongo_date},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArchonHuntUnmapped {
    #[serde(rename = "_id")]
    pub id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub reward: InternalPath<resolve_with::LastSegment>,

    pub seed: i64,

    pub boss: ResolvableString<resolve_with::sortie::Boss>,

    pub missions: Vec<MissionUnmapped>,
}

impl Resolve<ContextRef<'_>> for ArchonHuntUnmapped {
    type Output = ArchonHunt;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        ArchonHunt {
            id: self.id.oid,
            activation: self.activation,
            expiry: self.expiry,
            reward: self.reward.resolve(()),
            seed: self.seed,
            boss: self.boss.resolve(ctx).map(|boss| &boss.name).cloned(),
            missions: self.missions.resolve(ctx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionUnmapped {
    pub mission_type: WorldstateMissionType,

    pub node: SolNode,
}

impl Resolve<ContextRef<'_>> for MissionUnmapped {
    type Output = Mission;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        Mission {
            mission_type: self.mission_type.resolve(()),
            node: self.node.resolve(ctx).cloned(),
        }
    }
}
