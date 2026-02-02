use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolve_with, sol_node::SolNode},
    target_types::worldstate_types::{counted_item::CountedItem, invasion::Invasion},
    worldstate_model::{
        Id,
        WorldstateFaction,
        counted_item::CountedItemUnmapped,
        deserialize_mongo_date,
    },
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InvasionUnmapped {
    #[serde(rename = "_id")]
    id: Id,

    faction: WorldstateFaction,

    defender_faction: WorldstateFaction,

    node: SolNode,

    count: i64,

    goal: u64,

    loc_tag: InternalPath<resolve_with::LanguageItems>,

    completed: bool,

    #[serde(rename = "ChainID")]
    chain_id: Id,

    attacker_reward: AttackerRewardUnmapped,

    defender_reward: InvasionRewardUnmapped,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    activation: DateTime<Utc>,
}

impl Resolve<ContextRef<'_>> for InvasionUnmapped {
    type Output = Invasion;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        Invasion {
            id: self.id.oid,
            attacking_faction: self.faction.resolve(()),
            defending_faction: self.defender_faction.resolve(()),
            node: self.node.resolve(ctx).cloned(),
            count: self.count,
            goal: self.goal,
            loc_tag: self.loc_tag.resolve(ctx),
            completed: self.completed,
            chain_id: self.chain_id.oid,
            attacker_reward: self.attacker_reward.resolve(ctx),
            defender_reward: self.defender_reward.counted_items.resolve(ctx),
            activation: self.activation,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttackerRewardUnmapped {
    Array(Vec<Option<serde_json::Value>>),

    Reward(InvasionRewardUnmapped),
}

impl Resolve<ContextRef<'_>> for AttackerRewardUnmapped {
    type Output = Vec<CountedItem>;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        match self {
            AttackerRewardUnmapped::Array(_) => vec![],
            AttackerRewardUnmapped::Reward(reward_unmapped) => {
                reward_unmapped.counted_items.resolve(ctx)
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvasionRewardUnmapped {
    counted_items: Vec<CountedItemUnmapped>,
}
