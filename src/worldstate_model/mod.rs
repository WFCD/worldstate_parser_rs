use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    core::Resolve,
    target_types::{faction::Faction, language::Language, mission_type::MissionType},
};

pub mod alert;
pub mod archon_hunt;
pub mod event;
pub mod fissure;
pub mod goal;
pub mod sortie;

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

pub fn deserialize_mongo_date_opt<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
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

    let v: Option<MongoDate> = Option::deserialize(deserializer)?;

    match v {
        Some(mongo_date) => {
            let millis = mongo_date
                .date
                .number_long
                .parse::<i64>()
                .map_err(serde::de::Error::custom)?;

            let dt = Utc
                .timestamp_millis_opt(millis)
                .single()
                .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))?;

            Ok(Some(dt))
        },
        None => Ok(None),
    }
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

impl Resolve<()> for WorldstateFaction {
    type Output = Faction;

    fn resolve(self, _: ()) -> Self::Output {
        match self {
            WorldstateFaction::Corpus => Faction::Corpus,
            WorldstateFaction::Corrupted => Faction::Orokin,
            WorldstateFaction::Grineer => Faction::Grineer,
            WorldstateFaction::Infestation => Faction::Infested,
            WorldstateFaction::TheMurmur => Faction::Murmur,
            WorldstateFaction::Scaldra => Faction::Scaldra,
            WorldstateFaction::Sentient => Faction::Sentient,
            WorldstateFaction::Techrot => Faction::Techrot,
            WorldstateFaction::Orokin => Faction::Orokin,
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

impl Resolve<()> for WorldstateMissionType {
    type Output = MissionType;

    fn resolve(self, _: ()) -> Self::Output {
        match self {
            WorldstateMissionType::Rathuum => MissionType::Rathuum,
            WorldstateMissionType::VoidArmageddon => MissionType::VoidArmaggedon,
            WorldstateMissionType::Disruption => MissionType::Disruption,
            WorldstateMissionType::Assault => MissionType::Assault,
            WorldstateMissionType::Assassination => MissionType::Assassination,
            WorldstateMissionType::Capture => MissionType::Capture,
            WorldstateMissionType::VoidFlood => MissionType::VoidFlood,
            WorldstateMissionType::Defense => MissionType::Defense,
            WorldstateMissionType::LegacyteHarvest => MissionType::LegacyteHarvest,
            WorldstateMissionType::Defection => MissionType::Defection,
            WorldstateMissionType::Excavation => MissionType::Excavation,
            WorldstateMissionType::Exterminate => MissionType::Exterminate,
            WorldstateMissionType::HiveSabotage => MissionType::HiveSabotage,
            WorldstateMissionType::Spy => MissionType::Spy,
            WorldstateMissionType::Landscape => MissionType::Landscape,
            WorldstateMissionType::MobileDefense => MissionType::MobileDefense,
            WorldstateMissionType::InfestedSalvage => MissionType::InfestedSalvage,
            WorldstateMissionType::Rush => MissionType::Rush,
            WorldstateMissionType::Rescue => MissionType::Rescue,
            WorldstateMissionType::Hijack => MissionType::Hijack,
            WorldstateMissionType::Sabotage => MissionType::Sabotage,
            WorldstateMissionType::Survival => MissionType::Survival,
            WorldstateMissionType::Interception => MissionType::Interception,
            WorldstateMissionType::VoidCascade => MissionType::VoidCascade,
            WorldstateMissionType::Unknown => MissionType::Unknown,
            WorldstateMissionType::SanctuaryOnslaught => MissionType::SanctuaryOnslaught,
            WorldstateMissionType::Conclave => MissionType::Conclave,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WorldstateLanguage {
    EN,
    FR,
    IT,
    DE,
    ES,
    PT,
    RU,
    PL,
    UK,
    TR,
    JA,
    ZH,
    KO,
    TC,
    TH,
}

impl Resolve<()> for WorldstateLanguage {
    type Output = Language;
    fn resolve(self, _: ()) -> Self::Output {
        match self {
            Self::EN => Language::English,
            Self::FR => Language::French,
            Self::IT => Language::Italian,
            Self::DE => Language::German,
            Self::ES => Language::Spanish,
            Self::PT => Language::Portuguese,
            Self::RU => Language::Russian,
            Self::PL => Language::Polish,
            Self::UK => Language::Ukrainian,
            Self::TR => Language::Turkish,
            Self::JA => Language::Japanese,
            Self::ZH => Language::ChineseSimplified,
            Self::KO => Language::Korean,
            Self::TC => Language::ChineseTraditional,
            Self::TH => Language::Thai,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum WorldstateSyndicateType {
    ArbitersSyndicate,
    NecraloidSyndicate,
    EventSyndicate,
    CephalonSudaSyndicate,
    KahlSyndicate,
    NewLokaSyndicate,
    NightcapJournalSyndicate,
    QuillsSyndicate,
    RadioLegion3Syndicate,
    RadioLegion2Syndicate,
    PerrinSyndicate,
    RadioLegionIntermission10Syndicate,
    RadioLegionIntermission11Syndicate,
    RadioLegionIntermission13Syndicate,
    RadioLegionIntermission12Syndicate,
    RadioLegionIntermission2Syndicate,
    RadioLegionIntermission3Syndicate,
    RadioLegionIntermission14Syndicate,
    RadioLegionIntermission4Syndicate,
    RadioLegionIntermission6Syndicate,
    RadioLegionIntermission5Syndicate,
    RadioLegionIntermission9Syndicate,
    RadioLegionIntermission7Syndicate,
    RadioLegionIntermission8Syndicate,
    RadioLegionSyndicate,
    RadioLegionIntermissionSyndicate,
    VoxSyndicate,
    RedVeilSyndicate,
    VentKidsSyndicate,
    SteelMeridianSyndicate,
    EntratiLabSyndicate,
    HexSyndicate,
    EntratiSyndicate,
    CetusSyndicate,
    SolarisSyndicate,
    ZarimanSyndicate,
}
