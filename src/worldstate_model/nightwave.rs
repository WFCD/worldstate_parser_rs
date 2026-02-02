use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolve_with},
    target_types::worldstate_types::nightwave::{
        Nightwave,
        NightwaveChallenge,
        NightwaveChallengeInfo,
        NightwaveChallengeType,
    },
    worldstate_model::{Id, deserialize_mongo_date},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NightwaveUnmapped {
    #[serde(deserialize_with = "deserialize_mongo_date")]
    activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    expiry: DateTime<Utc>,

    affiliation_tag: String,

    season: u32,

    phase: u32,

    params: String,

    active_challenges: Vec<ActiveChallengeUnmapped>,
}

impl Resolve<ContextRef<'_>> for NightwaveUnmapped {
    type Output = Nightwave;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        Nightwave {
            activation: self.activation,
            expiry: self.expiry,
            affiliation_tag: self.affiliation_tag,
            season: self.season,
            phase: self.phase,
            params: self.params,
            active_challenges: self.active_challenges.resolve(ctx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActiveChallengeUnmapped {
    #[serde(rename = "_id")]
    id: Id,

    #[serde(default)]
    daily: bool,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    expiry: DateTime<Utc>,

    challenge: InternalPath<resolve_with::LanguageItemWithDesc>,
}

impl Resolve<ContextRef<'_>> for ActiveChallengeUnmapped {
    type Output = NightwaveChallenge;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        let challenge_info =
            NightwaveChallengeType::from_path(&self.challenge.path).map(|r#type| {
                NightwaveChallengeInfo {
                    standing_awarded: r#type.standing_awarded(),
                    challenge_type: r#type,
                }
            });

        NightwaveChallenge {
            id: self.id.oid,
            challenge_info,
            activation: self.activation,
            expiry: self.expiry,
            challenge: self.challenge.resolve(ctx),
        }
    }
}
