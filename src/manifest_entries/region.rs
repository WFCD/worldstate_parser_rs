use serde::Serialize;
use serde_repr::Deserialize_repr;

use crate::target_types::{faction::Faction, mission_type::MissionType};

#[derive(Deserialize_repr, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum NodeType {
    Default = 0,
    DarkSector = 4,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegionManifestEntry {
    pub unique_name: String,

    pub name: String,

    pub system_index: usize,

    pub system_name: String,

    pub node_type: NodeType,

    pub mastery_req: usize,

    pub mission_index: MissionType,

    pub faction_index: Faction,

    pub min_enemy_level: usize,

    pub max_enemy_level: usize,
}
