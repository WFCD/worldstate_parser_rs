use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::manifest_entries::{faction::Faction, region::MissionType};

pub mod alert;
pub mod fissure;

pub fn deserialize_mongo_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct MongoDate {
        #[serde(rename = "$date")]
        date: MongoDateInner,
    }

    #[derive(Deserialize)]
    struct MongoDateInner {
        #[serde(rename = "$numberLong")]
        number_long: String,
    }

    let v = MongoDate::deserialize(deserializer)?;
    let millis = v
        .date
        .number_long
        .parse::<i64>()
        .map_err(serde::de::Error::custom)?;

    Utc.timestamp_millis_opt(millis)
        .single()
        .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Id {
    #[serde(rename = "$oid")]
    pub oid: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldstateFaction {
    #[serde(rename = "FC_CORPUS")]
    Corpus,
    #[serde(rename = "FC_CORRUPTED")]
    Corrupted,
    #[serde(rename = "FC_GRINEER")]
    Grineer,
    #[serde(rename = "FC_INFESTATION")]
    Infestation,
    #[serde(rename = "FC_MITW")]
    TheMurmur,
    #[serde(rename = "FC_SCALDRA")]
    Scaldra,
    #[serde(rename = "FC_SENTIENT")]
    Sentient,
    #[serde(rename = "FC_TECHROT")]
    Techrot,
    #[serde(rename = "FC_OROKIN")]
    Orokin,
}

impl From<WorldstateFaction> for Faction {
    fn from(value: WorldstateFaction) -> Self {
        match value {
            WorldstateFaction::Corpus => Self::Corpus,
            WorldstateFaction::Corrupted => Self::Orokin,
            WorldstateFaction::Grineer => Self::Grineer,
            WorldstateFaction::Infestation => Self::Infested,
            WorldstateFaction::TheMurmur => Self::Murmur,
            WorldstateFaction::Scaldra => Self::Scaldra,
            WorldstateFaction::Sentient => Self::Sentient,
            WorldstateFaction::Techrot => Self::Techrot,
            WorldstateFaction::Orokin => Self::Orokin,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorldstateMissionType {
    #[serde(rename = "MT_ARENA")]
    Rathuum,
    #[serde(rename = "MT_ARMAGEDDON")]
    VoidArmageddon,
    #[serde(rename = "MT_ARTIFACT")]
    Disruption,
    #[serde(rename = "MT_ASSAULT")]
    Assault,
    #[serde(rename = "MT_ASSASSINATION")]
    Assassination,
    #[serde(rename = "MT_CAPTURE")]
    Capture,
    #[serde(rename = "MT_CORRUPTION")]
    VoidFlood,
    #[serde(rename = "MT_DEFAULT")]
    Unknown,
    #[serde(rename = "MT_DEFENSE")]
    Defense,
    #[serde(rename = "MT_ENDLESS_CAPTURE")]
    LegacyteHarvest,
    #[serde(rename = "MT_ENDLESS_EXTERMINATION")]
    SanctuaryOnslaught,
    #[serde(rename = "MT_EVACUATION")]
    Defection,
    #[serde(rename = "MT_EXCAVATE")]
    Excavation,
    #[serde(rename = "MT_EXTERMINATION")]
    Exterminate,
    #[serde(rename = "MT_HIVE")]
    HiveSabotage,
    #[serde(rename = "MT_INTEL")]
    Spy,
    #[serde(rename = "MT_LANDSCAPE")]
    Landscape,
    #[serde(rename = "MT_MOBILE_DEFENSE")]
    MobileDefense,
    #[serde(rename = "MT_PURIFY")]
    InfestedSalvage,
    #[serde(rename = "MT_PVP")]
    Conclave,
    #[serde(rename = "MT_RACE")]
    Rush,
    #[serde(rename = "MT_RESCUE")]
    Rescue,
    #[serde(rename = "MT_RETRIEVAL")]
    Hijack,
    #[serde(rename = "MT_SABOTAGE")]
    Sabotage,
    #[serde(rename = "MT_SURVIVAL")]
    Survival,
    #[serde(rename = "MT_TERRITORY")]
    Interception,
    #[serde(rename = "MT_VOID_CASCADE")]
    VoidCascade,
}

impl From<WorldstateMissionType> for MissionType {
    fn from(value: WorldstateMissionType) -> Self {
        match value {
            WorldstateMissionType::Rathuum => Self::Rathuum,
            WorldstateMissionType::VoidArmageddon => Self::VoidArmaggedon,
            WorldstateMissionType::Disruption => Self::Disruption,
            WorldstateMissionType::Assault => Self::Assault,
            WorldstateMissionType::Assassination => Self::Assassination,
            WorldstateMissionType::Capture => Self::Capture,
            WorldstateMissionType::VoidFlood => Self::VoidFlood,
            WorldstateMissionType::Defense => Self::Defense,
            WorldstateMissionType::LegacyteHarvest => Self::LegacyteHarvest,
            WorldstateMissionType::Defection => Self::Defection,
            WorldstateMissionType::Excavation => Self::Excavation,
            WorldstateMissionType::Exterminate => Self::Exterminate,
            WorldstateMissionType::HiveSabotage => Self::HiveSabotage,
            WorldstateMissionType::Spy => Self::Spy,
            WorldstateMissionType::Landscape => Self::Landscape,
            WorldstateMissionType::MobileDefense => Self::MobileDefense,
            WorldstateMissionType::InfestedSalvage => Self::InfestedSalvage,
            WorldstateMissionType::Rush => Self::Rush,
            WorldstateMissionType::Rescue => Self::Rescue,
            WorldstateMissionType::Hijack => Self::Hijack,
            WorldstateMissionType::Sabotage => Self::Sabotage,
            WorldstateMissionType::Survival => Self::Survival,
            WorldstateMissionType::Interception => Self::Interception,
            WorldstateMissionType::VoidCascade => Self::VoidCascade,
            _ => Self::Unknown,
        }
    }
}
