use std::{fmt, sync::LazyLock};

use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    de::{self, Visitor},
};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolve_with},
    target_types::{
        faction::Faction,
        language::Language,
        mission_type::MissionType,
        worldstate_types::syndicate::SyndicateType,
    },
    wfcd_data::bounty_rewards::{Bounty, DropItem},
};

pub mod alert;
pub mod archimedea;
pub mod archon_hunt;
pub mod calendar;
pub mod circuit;
pub mod counted_item;
pub mod daily_deal;
pub mod event;
pub mod fissure;
pub mod flash_sale;
pub mod goal;
pub mod invasion;
pub mod nightwave;
pub mod sortie;
pub mod syndicate_mission;
pub mod vault_trader;
pub mod void_storms;
pub mod void_trader;

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
    #[serde(rename = "MT_ALCHEMY")]
    Alchemy,
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
            WorldstateMissionType::Alchemy => MissionType::Alchemy,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum WorldstateSyndicateType {
    ArbitersSyndicate,
    NecraloidSyndicate,
    EventSyndicate,
    CephalonSudaSyndicate,
    KahlSyndicate,
    NewLokaSyndicate,
    NightcapJournalSyndicate,
    QuillsSyndicate,
    RadioLegionSyndicate,
    RadioLegion2Syndicate,
    RadioLegion3Syndicate,
    PerrinSyndicate,
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

    RadioLegionIntermission(u8),
}

impl Resolve<()> for WorldstateSyndicateType {
    type Output = SyndicateType;

    fn resolve(self, _ctx: ()) -> Self::Output {
        match self {
            WorldstateSyndicateType::ArbitersSyndicate => SyndicateType::Arbiters,
            WorldstateSyndicateType::NecraloidSyndicate => SyndicateType::Necraloid,
            WorldstateSyndicateType::EventSyndicate => SyndicateType::Event,
            WorldstateSyndicateType::CephalonSudaSyndicate => SyndicateType::CephalonSuda,
            WorldstateSyndicateType::KahlSyndicate => SyndicateType::Kahl,
            WorldstateSyndicateType::NewLokaSyndicate => SyndicateType::NewLoka,
            WorldstateSyndicateType::NightcapJournalSyndicate => SyndicateType::NightcapJournal,
            WorldstateSyndicateType::QuillsSyndicate => SyndicateType::Quills,
            WorldstateSyndicateType::RadioLegionSyndicate => SyndicateType::RadioLegion,
            WorldstateSyndicateType::RadioLegion2Syndicate => SyndicateType::RadioLegion2,
            WorldstateSyndicateType::RadioLegion3Syndicate => SyndicateType::RadioLegion3,
            WorldstateSyndicateType::PerrinSyndicate => SyndicateType::Perrin,
            WorldstateSyndicateType::VoxSyndicate => SyndicateType::Vox,
            WorldstateSyndicateType::RedVeilSyndicate => SyndicateType::RedVeil,
            WorldstateSyndicateType::VentKidsSyndicate => SyndicateType::VentKids,
            WorldstateSyndicateType::SteelMeridianSyndicate => SyndicateType::SteelMeridian,
            WorldstateSyndicateType::EntratiLabSyndicate => SyndicateType::Cavia,
            WorldstateSyndicateType::HexSyndicate => SyndicateType::Hex,
            WorldstateSyndicateType::EntratiSyndicate => SyndicateType::Entrati,
            WorldstateSyndicateType::CetusSyndicate => SyndicateType::Ostrons,
            WorldstateSyndicateType::SolarisSyndicate => SyndicateType::SolarisUnited,
            WorldstateSyndicateType::ZarimanSyndicate => SyndicateType::Zariman,
            WorldstateSyndicateType::RadioLegionIntermission(i) => SyndicateType::Nightwave(i),
        }
    }
}

impl<'de> Deserialize<'de> for WorldstateSyndicateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyndicateVisitor;

        impl<'de> Visitor<'de> for SyndicateVisitor {
            type Value = WorldstateSyndicateType;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid syndicate string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use WorldstateSyndicateType::*;

                // 1. Check exact matches for standard syndicates
                match value {
                    "ArbitersSyndicate" => Ok(ArbitersSyndicate),
                    "NecraloidSyndicate" => Ok(NecraloidSyndicate),
                    "EventSyndicate" => Ok(EventSyndicate),
                    "CephalonSudaSyndicate" => Ok(CephalonSudaSyndicate),
                    "KahlSyndicate" => Ok(KahlSyndicate),
                    "NewLokaSyndicate" => Ok(NewLokaSyndicate),
                    "NightcapJournalSyndicate" => Ok(NightcapJournalSyndicate),
                    "QuillsSyndicate" => Ok(QuillsSyndicate),
                    "RadioLegionSyndicate" => Ok(RadioLegionSyndicate),
                    "RadioLegion2Syndicate" => Ok(RadioLegion2Syndicate),
                    "RadioLegion3Syndicate" => Ok(RadioLegion3Syndicate),
                    "PerrinSyndicate" => Ok(PerrinSyndicate),
                    "VoxSyndicate" => Ok(VoxSyndicate),
                    "RedVeilSyndicate" => Ok(RedVeilSyndicate),
                    "VentKidsSyndicate" => Ok(VentKidsSyndicate),
                    "SteelMeridianSyndicate" => Ok(SteelMeridianSyndicate),
                    "EntratiLabSyndicate" => Ok(EntratiLabSyndicate),
                    "HexSyndicate" => Ok(HexSyndicate),
                    "EntratiSyndicate" => Ok(EntratiSyndicate),
                    "CetusSyndicate" => Ok(CetusSyndicate),
                    "SolarisSyndicate" => Ok(SolarisSyndicate),
                    "ZarimanSyndicate" => Ok(ZarimanSyndicate),

                    // 2. Handle Intermission logic
                    _ => {
                        if let Some(inner) = value.strip_prefix("RadioLegionIntermission")
                            && let Some(number_part) = inner.strip_suffix("Syndicate")
                        {
                            // If empty, it's the first one (implied 1)
                            if number_part.is_empty() {
                                return Ok(RadioLegionIntermission(1));
                            }
                            // Otherwise parse the number (e.g., "12")
                            if let Ok(n) = number_part.parse::<u8>() {
                                return Ok(RadioLegionIntermission(n));
                            }
                        }
                        Err(E::custom(format!("Unknown syndicate type: {}", value)))
                    },
                }
            }
        }

        deserializer.deserialize_str(SyndicateVisitor)
    }
}

static BOUNTY_REWARD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("Tier(?:(?:[ABCDE])|Narmer)Table([ABC])Rewards").unwrap());

static GHOUL_REWARD_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("GhoulBountyTable([AB])Rewards").unwrap());

#[derive(Debug, Clone)]
pub struct RotationalRewardContext<'a> {
    pub inner_ctx: ContextRef<'a>,
    pub syndicate_type: WorldstateSyndicateType,
    pub resource: String,
    pub min_level: u64,
    pub max_level: u64,
    pub is_vault: bool,
}

fn lookup_bounty(
    level_string: &str,
    bounty_tier: &str,
    bounty_collection: &[Bounty],
) -> Option<Vec<DropItem>> {
    bounty_collection
        .iter()
        .find(|bounty| bounty.bounty_level == level_string)
        .and_then(|bounty| bounty.rewards.get(bounty_tier))
        .cloned()
}

impl Resolve<RotationalRewardContext<'_>> for InternalPath<resolve_with::RotationalReward> {
    type Output = Option<Vec<DropItem>>;

    fn resolve(
        self,
        RotationalRewardContext {
            inner_ctx: ctx,
            syndicate_type,
            resource,
            min_level,
            max_level,
            is_vault,
        }: RotationalRewardContext<'_>,
    ) -> Self::Output {
        let table = resource.split('/').next_back()?;

        let level_range_string = format!("{} - {}", min_level, max_level);

        let bounty_tier = BOUNTY_REWARD_REGEX
            .captures(table)
            .and_then(|cap| cap.get(1))
            .map(|group| group.as_str())?;

        match syndicate_type {
            WorldstateSyndicateType::CetusSyndicate => {
                let level_string;
                let tier;

                if let Some(ghoul_tier) = GHOUL_REWARD_REGEX
                    .captures(table)
                    .and_then(|cap| cap.get(1))
                    .map(|group| group.as_str())
                {
                    level_string = format!("Level {level_range_string} Ghoul Bounty");
                    tier = ghoul_tier;
                } else {
                    level_string = format!("Level {level_range_string} Cetus Bounty");
                    tier = bounty_tier;
                }

                lookup_bounty(&level_string, tier, &ctx.worldstate_data.rewards.cetus)
            },

            WorldstateSyndicateType::EntratiSyndicate => {
                let variant = match is_vault {
                    true => "Isolation Vault",
                    false => "Cambion Drift Bounty",
                };

                let level_string = format!("Level {level_range_string} {variant}");

                lookup_bounty(
                    &level_string,
                    bounty_tier,
                    &ctx.worldstate_data.rewards.deimos,
                )
            },

            WorldstateSyndicateType::SolarisSyndicate => {
                let level_string = format!("Level {level_range_string} Orb Vallis Bounty");

                lookup_bounty(
                    &level_string,
                    bounty_tier,
                    &ctx.worldstate_data.rewards.solaris,
                )
            },

            _ => None,
        }
    }
}
