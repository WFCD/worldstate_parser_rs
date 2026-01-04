use serde::Serialize;
use serde_repr::Deserialize_repr;

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
    #[serde(rename(serialize = "Mobile Defense"))]
    MobileDefense = 9,
    Interception = 13,
    Hijack = 14,
    #[serde(rename(serialize = "Hive Sabotage"))]
    HiveSabotage = 15,
    Excavation = 17,
    #[serde(rename(serialize = "Infested Salvage"))]
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
    #[serde(rename(serialize = "Void Flood"))]
    VoidFlood = 33,
    #[serde(rename(serialize = "Void Cascade"))]
    VoidCascade = 34,
    #[serde(rename(serialize = "Void Armaggedon"))]
    VoidArmaggedon = 35,
    #[serde(rename(serialize = "Void Armaggedon 2"))]
    VoidArmageddon2 = 36,
    Alchemy = 38,
    #[serde(rename(serialize = "Legacyte Harvest"))]
    LegacyteHarvest = 40,
    #[serde(rename(serialize = "Shrine Defense"))]
    ShrineDefense = 41,
    Faceoff = 42,
    Descendia = 43,
    /// The Missions from the Perita Rebellion
    Recall = 44,

    Unknown = 255,
}
