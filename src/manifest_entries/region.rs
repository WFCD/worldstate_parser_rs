use serde::Serialize;
use serde_repr::Deserialize_repr;

use crate::manifest_entries::faction::Faction;

#[derive(Deserialize_repr, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum NodeType {
    Default = 0,
    DarkSector = 4,
}

#[derive(Serialize, Deserialize_repr, Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MissionType {
    Assassination = 0,
    Exterminate = 1,
    Survival = 2,
    Rescue = 3,
    Sabotage = 4,
    Capture = 5,
    Spy = 7,
    Defense = 8,
    MobileDefense = 9,
    Interception = 13,
    Hijack = 14,
    HiveSabotage = 15,
    Excavation = 17,
    InfestedSalvage = 21,
    Rathuum = 22,
    Pursuit = 24,
    Rush = 25,
    Assault = 26,
    Defection = 27,
    /// Free Roam
    Landscape = 28,
    Circuit = 31,
    Disruption = 32,
    VoidFlood = 33,
    VoidCascade = 34,
    VoidArmaggedon = 35,
    VoidArmageddon2 = 36,
    Alchemy = 38,
    LegacyteHarvest = 40,
    ShrineDefense = 41,
    Faceoff = 42,
    Descendia = 43,
    /// The Missions from the Perita Rebellion
    Recall = 44,
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
