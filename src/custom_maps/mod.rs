use std::collections::HashMap;

use crate::{
    core::Resolve,
    manifest_entries::manifest_relic_arcane::RelicArcane,
    manifests::Exports,
    target_types::{customs_entry::CustomsEntry, node::Node, relic::Relic},
};

pub type SolNodeToRegionMap = HashMap<String, Node>;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomMaps {
    pub solnode_to_region: SolNodeToRegionMap,
    pub relic_uniq_to_relic: HashMap<String, Relic>,
    pub unique_to_customs_entry: HashMap<String, CustomsEntry>,
}

impl CustomMaps {
    pub fn new(exports: &Exports) -> Self {
        let region = exports
            .export_regions
            .export_regions
            .iter()
            .cloned()
            .map(|region| (region.unique_name.clone(), region.resolve(())))
            .collect::<SolNodeToRegionMap>();

        let relic_map = exports
            .export_relic_arcane
            .export_relic_arcane
            .iter()
            .cloned()
            .filter_map(|relic_arcane| match relic_arcane {
                RelicArcane::Arcane(_a) => None,
                RelicArcane::Relic(manifest_relic) => Some((
                    manifest_relic.unique_name.clone(),
                    manifest_relic.resolve(()),
                )),
            })
            .collect();

        let customs_map = exports
            .export_customs
            .export_customs
            .iter()
            .cloned()
            .map(|entry| (entry.unique_name.clone(), entry.resolve(())))
            .collect();

        Self {
            solnode_to_region: region,
            relic_uniq_to_relic: relic_map,
            unique_to_customs_entry: customs_map,
        }
    }
}
