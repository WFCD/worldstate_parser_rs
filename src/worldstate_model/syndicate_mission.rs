use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolve_with, sol_node::SolNode},
    target_types::worldstate::syndicate_mission::{Job, MissionDetails, SyndicateMission},
    worldstate_model::{
        Id,
        RotationalRewardContext,
        WorldstateSyndicateType,
        deserialize_mongo_date,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SyndicateMissionUnmapped {
    #[serde(rename = "_id")]
    pub id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub seed: i64,

    pub tag: WorldstateSyndicateType,

    #[serde(flatten)]
    pub details: MissionDetailsUnmapped,
}

impl Resolve<ContextRef<'_>> for SyndicateMissionUnmapped {
    type Output = SyndicateMission;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        SyndicateMission {
            id: self.id.oid,
            activation: self.activation,
            expiry: self.expiry,
            seed: self.seed,
            syndicate_type: self.tag.resolve(()),
            details: self.details.resolve((ctx, self.tag)),
        }
    }
}

#[derive(Debug)]
pub enum MissionDetailsUnmapped {
    Bounties { jobs: Vec<JobUnmapped> },
    Nodes { nodes: Vec<SolNode> },
    Empty,
}

impl<'de> Deserialize<'de> for MissionDetailsUnmapped {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct MissionHelper {
            pub nodes: Vec<SolNode>,
            pub jobs: Option<Vec<JobUnmapped>>,
        }

        let helper = MissionHelper::deserialize(deserializer)?;

        if let Some(jobs) = helper.jobs
            && !jobs.is_empty()
        {
            return Ok(MissionDetailsUnmapped::Bounties { jobs });
        }

        if helper.nodes.is_empty() {
            return Ok(MissionDetailsUnmapped::Empty);
        }

        Ok(MissionDetailsUnmapped::Nodes {
            nodes: helper.nodes,
        })
    }
}

impl Resolve<(ContextRef<'_>, WorldstateSyndicateType)> for MissionDetailsUnmapped {
    type Output = MissionDetails;

    fn resolve(
        self,
        (ctx, syndicate_type): (ContextRef<'_>, WorldstateSyndicateType),
    ) -> Self::Output {
        match self {
            MissionDetailsUnmapped::Bounties { jobs } => MissionDetails::Bounties(
                jobs.into_iter()
                    .map(|job| {
                        let ctx = RotationalRewardContext {
                            inner_ctx: ctx,
                            syndicate_type,
                            is_vault: job.is_vault,
                            resource: job.rewards.path.clone(),
                            min_level: job.min_enemy_level,
                            max_level: job.max_enemy_level,
                        };

                        job.resolve(ctx)
                    })
                    .collect(),
            ),
            MissionDetailsUnmapped::Nodes { nodes } => MissionDetails::Nodes(
                nodes
                    .resolve(ctx)
                    .iter()
                    .map(|node| node.cloned())
                    .collect(),
            ),
            MissionDetailsUnmapped::Empty => MissionDetails::Empty,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobUnmapped {
    pub job_type: Option<InternalPath<resolve_with::LanguageItems>>,

    pub rewards: InternalPath<resolve_with::RotationalReward>,

    pub mastery_req: u64,

    pub min_enemy_level: u64,

    pub max_enemy_level: u64,

    pub xp_amounts: Vec<u64>,

    #[serde(default)]
    pub endless: bool,

    pub location_tag: Option<String>,

    #[serde(default)]
    pub is_vault: bool,
}

impl Resolve<RotationalRewardContext<'_>> for JobUnmapped {
    type Output = Job;

    fn resolve(self, ctx: RotationalRewardContext<'_>) -> Self::Output {
        Job {
            job_type: self.job_type.resolve(ctx.inner_ctx),
            rewards: self.rewards.resolve(ctx).unwrap_or_default(),
            mastery_req: self.mastery_req,
            min_enemy_level: self.min_enemy_level,
            max_enemy_level: self.max_enemy_level,
            xp_amounts: self.xp_amounts,
            endless: self.endless,
            location_tag: self.location_tag,
            is_vault: self.is_vault,
        }
    }
}
