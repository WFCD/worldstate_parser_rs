use serde::{Deserialize, Serialize};

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

    #[serde(rename(serialize = "Mobile Defense"))]
    MobileDefense,

    Interception,

    Hijack,

    #[serde(rename(serialize = "Hive Sabotage"))]
    HiveSabotage,

    Excavation,

    #[serde(rename(serialize = "Infested Salvage"))]
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

    #[serde(rename(serialize = "Void Flood"))]
    VoidFlood,

    #[serde(rename(serialize = "Void Cascade"))]
    VoidCascade,

    #[serde(rename(serialize = "Void Armaggedon"))]
    VoidArmaggedon,

    /// I don't really what this is
    #[serde(rename(serialize = "Void Armaggedon 2"))]
    VoidArmageddon2,

    Alchemy,

    #[serde(rename(serialize = "Legacyte Harvest"))]
    LegacyteHarvest,

    #[serde(rename(serialize = "Shrine Defense"))]
    ShrineDefense,

    Faceoff,

    Descendia,

    /// The Missions from the Perita Rebellion
    Recall,

    SanctuaryOnslaught,
    EliteSanctuaryOnslaught,

    Conclave,

    Unknown,
}
