use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolve_with},
    target_types::worldstate_types::archimedea::{
        Archimedea,
        ArchimedeaDifficulties,
        ArchimedeaMission,
        ArchimedeaRoot,
    },
    worldstate_model::{WorldstateFaction, WorldstateMissionType, deserialize_mongo_date},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArchimedeaUnmapped {
    #[serde(deserialize_with = "deserialize_mongo_date")]
    activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    expiry: DateTime<Utc>,

    #[serde(rename = "Type")]
    archimedea_type: ArchimedeaTypeUnmapped,

    missions: Vec<ArchimedeaMissionUnmapped>,

    variables: Vec<InternalPath<resolve_with::LanguageItemWithDesc>>,

    random_seed: u64,
}

impl ArchimedeaUnmapped {
    fn resolve_for_difficulty(
        &self,
        ctx: ContextRef<'_>,
        difficulty_type: ArchimedeaDifficultyTypeUnmapped,
    ) -> Archimedea {
        Archimedea {
            activation: self.activation,
            expiry: self.expiry,
            missions: self
                .missions
                .iter()
                .map(|m| ArchimedeaMission {
                    faction: m.faction.resolve(()),
                    mission_type: m.mission_type.resolve(()),
                    difficulties: m
                        .difficulties
                        .iter()
                        .filter(|d| d.difficulty_type == difficulty_type)
                        .map(|d| ArchimedeaDifficulties {
                            deviation: d.deviation.clone().resolve(ctx),
                            risks: d.risks.clone().resolve(ctx),
                        })
                        .collect(),
                })
                .collect(),
            variables: self.variables.clone().resolve(ctx),
            random_seed: self.random_seed,
        }
    }
}

impl Resolve<ContextRef<'_>> for Vec<ArchimedeaUnmapped> {
    type Output = ArchimedeaRoot;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        let mut deep = None;
        let mut elite_deep = None;
        let mut temporal = None;
        let mut elite_temporal = None;

        for archimedea in self {
            let is_entrati = matches!(archimedea.archimedea_type, ArchimedeaTypeUnmapped::Entrati);
            let is_hex = matches!(archimedea.archimedea_type, ArchimedeaTypeUnmapped::Hex);

            if is_entrati {
                deep = Some(
                    archimedea
                        .resolve_for_difficulty(ctx, ArchimedeaDifficultyTypeUnmapped::Normal),
                )
            };
            if is_entrati {
                elite_deep = Some(
                    archimedea
                        .resolve_for_difficulty(ctx, ArchimedeaDifficultyTypeUnmapped::SteelPath),
                )
            };
            if is_hex {
                temporal = Some(
                    archimedea
                        .resolve_for_difficulty(ctx, ArchimedeaDifficultyTypeUnmapped::Normal),
                )
            };
            if is_hex {
                elite_temporal = Some(
                    archimedea
                        .resolve_for_difficulty(ctx, ArchimedeaDifficultyTypeUnmapped::SteelPath),
                )
            };
        }
        ArchimedeaRoot {
            deep,
            elite_deep,
            temporal,
            elite_temporal,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchimedeaMissionUnmapped {
    faction: WorldstateFaction,

    mission_type: WorldstateMissionType,

    difficulties: Vec<ArchimedeaDifficultyUnmapped>,
}

impl Resolve<ContextRef<'_>> for ArchimedeaMissionUnmapped {
    type Output = ArchimedeaMission;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        ArchimedeaMission {
            faction: self.faction.resolve(()),
            mission_type: self.mission_type.resolve(()),
            difficulties: self.difficulties.resolve(ctx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchimedeaDifficultyUnmapped {
    #[serde(rename = "type")]
    difficulty_type: ArchimedeaDifficultyTypeUnmapped,

    deviation: InternalPath<resolve_with::LanguageItemWithDesc>,

    risks: Vec<InternalPath<resolve_with::LanguageItemWithDesc>>,
}

impl Resolve<ContextRef<'_>> for ArchimedeaDifficultyUnmapped {
    type Output = ArchimedeaDifficulties;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        ArchimedeaDifficulties {
            deviation: self.deviation.resolve(ctx),
            risks: self.risks.resolve(ctx),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ArchimedeaDifficultyTypeUnmapped {
    #[serde(rename = "CD_NORMAL")]
    Normal,

    #[serde(rename = "CD_HARD")]
    SteelPath,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ArchimedeaTypeUnmapped {
    #[serde(rename = "CT_LAB")]
    Entrati,

    #[serde(rename = "CT_HEX")]
    Hex,
}
