use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    target_types::{node::Node, worldstate_types::syndicate::SyndicateType},
    wfcd_data::bounty_rewards::DropItem,
};

fn deserialize_null_as_empty<'de, D>(deserializer: D) -> Result<SyndicateMissionDetails, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(SyndicateMissionDetails::Empty))
}

fn serialize_empty_as_null<S>(
    details: &SyndicateMissionDetails,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match details {
        SyndicateMissionDetails::Empty => serializer.serialize_none(), // Writes `null`
        _ => details.serialize(serializer),                            // Writes normal JSON object
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SyndicateMission {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub seed: i64,

    pub syndicate_type: SyndicateType,

    #[serde(
        deserialize_with = "deserialize_null_as_empty",
        serialize_with = "serialize_empty_as_null"
    )]
    pub details: SyndicateMissionDetails,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum SyndicateMissionDetails {
    Bounties(Vec<SyndicateJob>),
    Nodes(Vec<Option<Node>>),
    Empty,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SyndicateJob {
    pub job_type: Option<String>,

    pub rewards: Vec<DropItem>,

    pub mastery_req: u64,

    pub min_enemy_level: u64,

    pub max_enemy_level: u64,

    pub xp_amounts: Vec<u64>,

    pub endless: bool,

    pub location_tag: Option<String>,

    /// Whether it's an isolation vault or not.
    pub is_vault: bool,
}
