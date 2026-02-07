use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::{
    faction::Faction,
    mission_type::MissionType,
    node::Node,
    worldstate_types::counted_item::CountedItem,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub mission_info: AlertMissionInfo,

    pub tag: String,

    pub icon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AlertMissionInfo {
    pub mission_type: MissionType,

    pub faction: Faction,

    pub node: Option<Node>,

    pub level_override: Option<String>,

    pub enemy_spec: Option<String>,

    pub extra_enemy_spec: Option<String>,

    pub min_enemy_level: i64,

    pub max_enemy_level: i64,

    pub difficulty: i64,

    pub seed: Option<i64>,

    pub mission_reward: AlertMissionReward,

    pub desc_text: String,

    pub quest_req: Option<String>,

    pub leaders_always_allowed: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct AlertMissionReward {
    pub credits: Option<i64>,

    pub items: Vec<String>,

    pub counted_items: Vec<CountedItem>,
}
