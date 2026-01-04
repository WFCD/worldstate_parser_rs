use std::{collections::HashMap, str::FromStr};

use crate::manifest_entries::region::RegionManifestEntry;

#[derive(Debug)]
pub struct PublicExportIndex {
    pub customs: String,
    pub drones: String,
    pub flavour: String,
    pub fusion_bundles: String,
    pub gear: String,
    pub keys: String,
    pub recipes: String,
    pub regions: String,
    pub relic_arcane: String,
    pub resources: String,
    pub sentinels: String,
    pub sortie_rewards: String,
    pub upgrades: String,
    pub warframes: String,
    pub weapons: String,
    pub manifest: String,
}

impl FromStr for PublicExportIndex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: HashMap<&str, &str> = HashMap::new();

        for line in s.lines() {
            if let Some((key, _)) = line.split_once('!') {
                values.insert(key, line);
            }
        }

        let get_val = |key: &str| {
            values
                .get(key)
                .map(|&v| v.to_owned())
                .ok_or_else(|| format!("Missing key: {}", key))
        };

        Ok(PublicExportIndex {
            customs: get_val("ExportCustoms_en.json")?,
            drones: get_val("ExportDrones_en.json")?,
            flavour: get_val("ExportFlavour_en.json")?,
            fusion_bundles: get_val("ExportFusionBundles_en.json")?,
            gear: get_val("ExportGear_en.json")?,
            keys: get_val("ExportKeys_en.json")?,
            recipes: get_val("ExportRecipes_en.json")?,
            regions: get_val("ExportRegions_en.json")?,
            relic_arcane: get_val("ExportRelicArcane_en.json")?,
            resources: get_val("ExportResources_en.json")?,
            sentinels: get_val("ExportSentinels_en.json")?,
            sortie_rewards: get_val("ExportSortieRewards_en.json")?,
            upgrades: get_val("ExportUpgrades_en.json")?,
            warframes: get_val("ExportWarframes_en.json")?,
            weapons: get_val("ExportWeapons_en.json")?,
            manifest: get_val("ExportManifest.json")?,
        })
    }
}

all_the_exports! {
    struct ExportRegions(RegionManifestEntry);

}

macro_rules! all_the_exports {
    (
        $( struct $ident:ident( $inner_type:ty ); )*
    ) => {
        paste::paste! {
            $(
                #[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq)]
                #[serde(rename_all = "PascalCase")]
                pub struct $ident {
                    pub [<$ident:snake>]: Vec<$inner_type>,
                }
            )*

            #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
            pub struct Exports {
                $(
                    pub [<$ident:snake>]: $ident,
                )*
            }
        }
    };
}

use all_the_exports;
