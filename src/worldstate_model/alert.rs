use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    custom_maps::solnode_to_region::Region,
    manifest_entries::{faction::Faction, region::MissionType},
    worldstate_model::deserialize_mongo_date,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct Alert {
    pub id: String,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub mission_info: MissionInfo,

    pub tag: String,

    pub icon: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MissionInfo {
    pub mission_type: MissionType,

    pub faction: Faction,

    pub location: Option<Region>,

    pub level_override: Option<String>,

    pub enemy_spec: Option<String>,

    pub extra_enemy_spec: Option<String>,

    pub min_enemy_level: i64,

    pub max_enemy_level: i64,

    pub difficulty: i64,

    pub seed: i64,

    pub mission_reward: MissionReward,

    pub desc_text: String,

    pub quest_req: Option<String>,

    pub leaders_always_allowed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct MissionReward {
    pub credits: Option<i64>,

    pub counted_items: Vec<CountedItem>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct CountedItem {
    pub item_type: String,

    pub item_count: i64,
}

pub(crate) mod unmapped {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use crate::{
        core::{Context, InternalPath, Resolve, SolNode, resolve_with},
        worldstate_model::{
            Id,
            WorldstateFaction,
            WorldstateMissionType,
            alert::{Alert, CountedItem, MissionInfo, MissionReward},
            deserialize_mongo_date,
        },
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
                mission_type: self.mission_type.into(),
                faction: self.faction.into(),
                location: self.location.resolve(ctx).cloned(),
                level_override: self.level_override.resolve(ctx),
                enemy_spec: self.enemy_spec.resolve(ctx),
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

        leaders_always_allowed: bool,
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
}
