use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::{
    faction::Faction,
    node::Node,
    worldstate_types::counted_item::CountedItem,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Invasion {
    pub id: String,

    pub attacking_faction: Faction,

    pub defending_faction: Faction,

    pub node: Option<Node>,

    pub count: i64,

    pub goal: u64,

    pub loc_tag: String,

    pub completed: bool,

    pub chain_id: String,

    pub attacker_reward: Vec<CountedItem>,

    pub defender_reward: Vec<CountedItem>,

    pub activation: DateTime<Utc>,
}
