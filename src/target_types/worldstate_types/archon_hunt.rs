use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    target_types::{mission_type::MissionType, node::Node},
    wfcd_data::bounty_rewards::DropItem,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArchonHunt {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub reward_pool: Vec<DropItem>,

    pub seed: i64,

    pub boss: Option<String>,

    pub missions: Vec<ArchonHuntMission>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ArchonHuntMission {
    pub mission_type: MissionType,

    pub node: Option<Node>,
}
