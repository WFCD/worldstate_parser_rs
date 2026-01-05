use std::collections::HashMap;

use crate::{core::Resolve, manifests::Exports, target_types::node::Node};

pub type SolNodeToRegionMap = HashMap<String, Node>;

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
            .map(|region| (region.unique_name.clone(), region.resolve(())))
            .collect::<SolNodeToRegionMap>();

        Self {
            solnode_to_region: region,
        }
    }
}
