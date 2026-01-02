use serde::{Deserialize, Serialize};

use crate::{
    core::Mappable,
    custom_maps::CustomMaps,
    manifests::Exports,
    worldstate_model::fissure::{Fissure, FissureUnmapped},
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldStateUnmapped {
    #[serde(rename = "ActiveMissions")]
    pub fissures: Vec<FissureUnmapped>,
}

impl WorldStateUnmapped {
    pub fn map_worldstate(self, exports: Exports) -> Option<WorldState> {
        let custom_maps = CustomMaps::from_exports(&exports);

        let fissures = self
            .fissures
            .into_iter()
            .map(|unmapped_fissure| unmapped_fissure.map(&exports, &custom_maps))
            .collect::<Option<Vec<_>>>()?;

        Some(WorldState { fissures })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldState {
    pub fissures: Vec<Fissure>,
}
