use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    target_types::{mission_type::MissionType, node::Node},
    wfcd_data::bounty_rewards::DropItem,
};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sortie {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub reward: String,

    pub seed: u64,

    pub boss: Option<String>,

    pub reward_pool: Vec<DropItem>,

    pub extra_drops: Vec<serde_json::Value>,

    pub variants: Vec<SortieVariant>,

    pub twitter: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SortieVariant {
    pub mission_type: MissionType,

    pub modifier_type: String,

    pub node: Option<Node>,

    pub tileset: String,
}
