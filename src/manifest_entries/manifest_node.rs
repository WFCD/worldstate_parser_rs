use serde::Serialize;
use serde_repr::Deserialize_repr;

use crate::{
    core::Resolve,
    manifest_entries::{
        manifest_faction::ManifestFaction,
        manifest_mission_type::ManifestMissionType,
    },
    target_types::node::Node,
};

#[derive(Deserialize_repr, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum NodeType {
    Default = 0,
    DarkSector = 4,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ManifestNode {
    pub unique_name: String,

    pub name: String,

    pub system_index: usize,

    pub system_name: String,

    pub node_type: NodeType,

    pub mastery_req: usize,

    pub mission_index: ManifestMissionType,

    pub faction_index: ManifestFaction,

    pub min_enemy_level: usize,

    pub max_enemy_level: usize,
}

impl Resolve<()> for ManifestNode {
    type Output = Node;

    fn resolve(self, _: ()) -> Self::Output {
        Node {
            system_index: self.system_index,
            node: self.name,
            planet: self.system_name,
            mastery_req: self.mastery_req,
            mission_type: self.mission_index.resolve(()),
            faction: self.faction_index.resolve(()),
            min_enemy_level: self.min_enemy_level,
            max_enemy_level: self.max_enemy_level,
            is_dark_sector: self.node_type == NodeType::DarkSector,
        }
    }
}
