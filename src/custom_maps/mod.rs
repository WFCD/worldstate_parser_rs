use std::collections::HashMap;

use crate::{manifests::Exports, target_types::region::Region};

pub type SolNodeToRegionMap = HashMap<String, Region>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomMaps {
    pub solnode_to_region: SolNodeToRegionMap,
}

impl CustomMaps {
    pub fn from_exports(exports: &Exports) -> Self {
        let region = exports
            .export_regions
            .export_regions
            .iter()
            .cloned()
            .map(|region| (region.unique_name.clone(), Region::from(region)))
            .collect::<SolNodeToRegionMap>();

        Self {
            solnode_to_region: region,
        }
    }
}
