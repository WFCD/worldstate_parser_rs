use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::node::Node;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum FissureTier {
    Lith,
    Meso,
    Neo,
    Axi,
    Requiem,
    Omnia,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Fissure {
    pub id: String,

    pub node: Option<Node>,

    pub seed: usize,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub tier: FissureTier,

    pub is_steel_path: bool,
}
