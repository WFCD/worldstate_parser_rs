use serde::{Deserialize, Serialize};

use crate::{
    core::{Context, Resolve},
    target_types::worldstate::{alert::Alert, fissure::Fissure, sortie::Sortie},
    worldstate_model::{alert::AlertUnmapped, fissure::FissureUnmapped, sortie::SortieUnmapped},
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct WorldStateUnmapped {
    #[serde(rename = "ActiveMissions")]
    pub fissures: Vec<FissureUnmapped>,

    pub alerts: Vec<AlertUnmapped>,

    pub sorties: Vec<SortieUnmapped>,
}

impl WorldStateUnmapped {
    pub fn map_worldstate(self, ctx: Context<'_>) -> Option<WorldState> {
        let fissures = self.fissures.resolve(ctx);
        let alerts = self.alerts.resolve(ctx);
        let sorties = self.sorties.resolve(ctx);

        Some(WorldState {
            fissures,
            alerts,
            sorties,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldState {
    pub fissures: Vec<Fissure>,

    pub alerts: Vec<Alert>,

    pub sorties: Vec<Sortie>,
}
