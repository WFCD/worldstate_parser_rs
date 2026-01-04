use crate::{
    custom_maps::solnode_to_region::{Region, SolNodeToRegionMap},
    manifests::Exports,
};

pub mod solnode_to_region;

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
