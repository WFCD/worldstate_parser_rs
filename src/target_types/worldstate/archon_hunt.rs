use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::{mission_type::MissionType, node::Node};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct ArchonHunt {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub reward: String,

    pub seed: i64,

    pub boss: Option<String>,

    pub missions: Vec<Mission>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    pub mission_type: MissionType,

    pub node: Option<Node>,
}
