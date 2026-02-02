use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::{display_info::DisplayInfo, faction::Faction, mission_type::MissionType};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ArchimedeaRoot {
    pub deep: Option<Archimedea>,
    pub elite_deep: Option<Archimedea>,
    pub temporal: Option<Archimedea>,
    pub elite_temporal: Option<Archimedea>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Archimedea {
    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub missions: Vec<ArchimedeaMission>,

    pub variables: Vec<DisplayInfo>,

    pub random_seed: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ArchimedeaMission {
    pub faction: Faction,

    pub mission_type: MissionType,

    pub difficulties: Vec<ArchimedeaDifficulties>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ArchimedeaDifficulties {
    pub deviation: DisplayInfo,

    pub risks: Vec<DisplayInfo>,
}
