use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::node::Node;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: String,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub grace_period: Option<DateTime<Utc>>,

    pub count: u64,

    pub goal: u64,

    pub success: Option<u64>,

    pub personal: bool,

    pub desc: String,

    pub tool_tip: String,

    pub icon: Option<String>,

    pub tag: String,

    pub node: Option<Node>,
}
