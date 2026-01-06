use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{Context, InternalPath, Resolve, resolve_with, sol_node::SolNode},
    target_types::worldstate::alert::{Alert, CountedItem, MissionInfo, MissionReward},
    worldstate_model::{Id, WorldstateFaction, WorldstateMissionType, deserialize_mongo_date},
};

impl Resolve<Context<'_>> for AlertUnmapped {
    type Output = Alert;

    fn resolve(self, ctx: Context) -> Self::Output {
        Alert {
            id: self.id.oid,
            activation: self.activation,
            expiry: self.expiry,
            mission_info: self.mission_info.resolve(ctx),
            tag: self.tag,
            icon: self.icon,
        }
    }
}

impl Resolve<Context<'_>> for MissionInfoUnmapped {
    type Output = MissionInfo;

    fn resolve(self, ctx: Context) -> Self::Output {
        MissionInfo {
            mission_type: self.mission_type.resolve(()),
            faction: self.faction.resolve(()),
            node: self.location.resolve(ctx).cloned(),
            level_override: self.level_override.resolve(()),
            enemy_spec: self.enemy_spec.resolve(()),
            extra_enemy_spec: self.extra_enemy_spec.and_then(|spec| spec.to_title_case()),
            min_enemy_level: self.min_enemy_level,
            max_enemy_level: self.max_enemy_level,
            difficulty: self.difficulty,
            seed: self.seed,
            mission_reward: self.mission_reward.resolve(ctx),
            desc_text: self.desc_text.resolve(ctx),
            quest_req: self.quest_req.map(|quest_req| quest_req.resolve(ctx)),
            leaders_always_allowed: self.leaders_always_allowed,
        }
    }
}

impl Resolve<Context<'_>> for MissionRewardUnmapped {
    type Output = MissionReward;

    fn resolve(self, ctx: Context) -> Self::Output {
        MissionReward {
            credits: self.credits,
            counted_items: self
                .counted_items
                .into_iter()
                .map(|item| item.resolve(ctx))
                .collect(),
        }
    }
}

impl Resolve<Context<'_>> for CountedItemUnmapped {
    type Output = CountedItem;

    fn resolve(self, ctx: Context) -> Self::Output {
        CountedItem {
            item_count: self.item_count,
            item_type: self.item_type.resolve(ctx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AlertUnmapped {
    #[serde(rename = "_id")]
    id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    mission_info: MissionInfoUnmapped,

    tag: String,

    icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionInfoUnmapped {
    mission_type: WorldstateMissionType,

    faction: WorldstateFaction,

    /// Sol Node
    location: SolNode,

    level_override: Option<InternalPath<resolve_with::LastSegment>>,

    enemy_spec: Option<InternalPath<resolve_with::LastSegment>>,

    extra_enemy_spec: Option<InternalPath>,

    min_enemy_level: i64,

    max_enemy_level: i64,

    difficulty: i64,

    seed: i64,

    mission_reward: MissionRewardUnmapped,

    desc_text: InternalPath<resolve_with::LanguageItems>,

    quest_req: Option<InternalPath<resolve_with::LanguageItems>>,

    leaders_always_allowed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionRewardUnmapped {
    credits: Option<i64>,

    #[serde(default)]
    counted_items: Vec<CountedItemUnmapped>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CountedItemUnmapped {
    item_type: InternalPath<resolve_with::LanguageItems>,

    item_count: i64,
}
