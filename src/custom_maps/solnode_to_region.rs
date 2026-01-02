use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::manifest_entries::{
    faction::Faction,
    region::{MissionType, NodeType, RegionManifestEntry},
};

pub type SolNodeToRegionMap = HashMap<String, Region>;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Region {
    pub system_index: usize,

    pub node: String,

    pub planet: String,

    pub mastery_req: usize,

    pub mission_type: MissionType,

    pub faction: Faction,

    pub min_enemy_level: usize,

    pub max_enemy_level: usize,

    pub is_dark_sector: bool,
}

impl From<RegionManifestEntry> for Region {
    fn from(manifest_region: RegionManifestEntry) -> Self {
        Self {
            system_index: manifest_region.system_index,
            node: manifest_region.name,
            planet: manifest_region.system_name,
            mastery_req: manifest_region.mastery_req,
            mission_type: manifest_region.mission_index,
            faction: manifest_region.faction_index,
            min_enemy_level: manifest_region.min_enemy_level,
            max_enemy_level: manifest_region.max_enemy_level,
            is_dark_sector: manifest_region.node_type == NodeType::DarkSector,
        }
    }
}
