use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    pub unique_name: String,

    pub name: String,

    pub codex_secret: bool,

    pub description: String,

    pub relic_rewards: Vec<RelicReward>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelicReward {
    pub reward_name: String,

    pub rarity: RelicRewardRarity,

    pub tier: i64,

    pub item_count: i64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
pub enum RelicRewardRarity {
    Common,
    Uncommon,
    Rare,
}
