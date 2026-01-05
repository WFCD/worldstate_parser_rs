use serde::{Deserialize, Serialize};

use crate::target_types::{faction::Faction, mission_type::MissionType};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Node {
    pub system_index: usize,

    pub name: String,

    pub planet: String,

    pub mastery_req: usize,

    pub mission_type: MissionType,

    pub faction: Faction,

    pub min_enemy_level: usize,

    pub max_enemy_level: usize,

    pub is_dark_sector: bool,
}
