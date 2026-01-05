use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::node::Node;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Tier {
    #[serde(rename(deserialize = "VoidT1"))]
    Lith,
    #[serde(rename(deserialize = "VoidT2"))]
    Meso,
    #[serde(rename(deserialize = "VoidT3"))]
    Neo,
    #[serde(rename(deserialize = "VoidT4"))]
    Axi,
    #[serde(rename(deserialize = "VoidT5"))]
    Requiem,
    #[serde(rename(deserialize = "VoidT6"))]
    Omnia,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
pub struct Fissure {
    pub id: String,

    pub node: Option<Node>,

    pub seed: usize,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub tier: Tier,

    pub is_steel_path: bool,
}
