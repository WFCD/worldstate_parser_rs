use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct BountyRewards {
    #[serde(rename(deserialize = "cetusBountyRewards"))]
    pub cetus: Vec<Bounty>,

    #[serde(rename(deserialize = "solarisBountyRewards"))]
    pub solaris: Vec<Bounty>,

    #[serde(rename(deserialize = "deimosRewards"))]
    pub deimos: Vec<Bounty>,

    #[serde(rename(deserialize = "zarimanRewards"))]
    pub zariman: Vec<Bounty>,

    #[serde(rename(deserialize = "entratiLabRewards"))]
    pub cavia: Vec<Bounty>,

    #[serde(rename(deserialize = "hexRewards"))]
    pub hex: Vec<Bounty>,

    #[serde(rename(deserialize = "sortieRewards"))]
    pub sortie_rewards: Vec<DropItem>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bounty {
    #[serde(rename(deserialize = "_id"), skip_serializing)]
    pub id: String,

    pub bounty_level: String,

    pub rewards: HashMap<String, Vec<DropItem>>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DropItem {
    pub item_name: String,

    pub rarity: String,

    pub chance: f64,
}
