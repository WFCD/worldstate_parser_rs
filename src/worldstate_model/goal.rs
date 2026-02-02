use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolve_with, sol_node::SolNode},
    target_types::worldstate_types::goal::Goal,
    worldstate_model::{Id, deserialize_mongo_date, deserialize_mongo_date_opt},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GoalUnmapped {
    #[serde(rename = "_id")]
    id: Id,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    expiry: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date_opt", default)]
    grace_period: Option<DateTime<Utc>>,

    count: u64,

    goal: u64,

    success: Option<u64>,

    personal: bool,

    desc: InternalPath<resolve_with::LanguageItems>,

    tool_tip: InternalPath<resolve_with::LanguageItems>,

    icon: Option<String>,

    tag: String,

    node: SolNode,
}

impl Resolve<ContextRef<'_>> for GoalUnmapped {
    type Output = Goal;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        Goal {
            id: self.id.oid,
            activation: self.activation,
            expiry: self.expiry,
            grace_period: self.grace_period,
            count: self.count,
            goal: self.goal,
            success: self.success,
            personal: self.personal,
            desc: self.desc.resolve(ctx),
            tool_tip: self.tool_tip.resolve(ctx),
            icon: self.icon,
            tag: self.tag,
            node: self.node.resolve(ctx).cloned(),
        }
    }
}
