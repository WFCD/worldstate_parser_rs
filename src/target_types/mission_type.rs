use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MissionType {
    Assassination,

    Exterminate,

    Survival,

    Rescue,

    Sabotage,

    Capture,

    Spy,

    Defense,

    #[serde(rename = "Mobile Defense")]
    MobileDefense,

    Interception,

    Hijack,

    #[serde(rename = "Hive Sabotage")]
    HiveSabotage,

    Excavation,

    #[serde(rename = "Infested Salvage")]
    InfestedSalvage,

    Rathuum,

    Pursuit,

    Rush,

    Assault,

    Defection,

    /// Free Roam
    Landscape,

    Circuit,

    Disruption,

    #[serde(rename = "Void Flood")]
    VoidFlood,

    #[serde(rename = "Void Cascade")]
    VoidCascade,

    #[serde(rename = "Void Armaggedon")]
    VoidArmaggedon,

    /// I don't really what this is
    #[serde(rename = "Void Armaggedon 2")]
    VoidArmageddon2,

    Alchemy,

    #[serde(rename = "Legacyte Harvest")]
    LegacyteHarvest,

    #[serde(rename = "Shrine Defense")]
    ShrineDefense,

    Faceoff,

    Descendia,

    /// The Missions from the Perita Rebellion
    Recall,

    #[serde(rename = "Sanctuary Onslaught")]
    SanctuaryOnslaught,

    #[serde(rename = "Elite Sanctuary Onslaught")]
    EliteSanctuaryOnslaught,

    Conclave,

    Skirmish,

    #[serde(rename = "Free Flight")]
    FreeFlight,

    Orphix,

    Volatile,

    #[serde(rename = "Follie's Hunt")]
    FolliesHunt,

    Unknown,
}
