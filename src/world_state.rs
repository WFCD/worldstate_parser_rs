use serde::{Deserialize, Serialize};

use crate::{
    core::{Context, Resolve},
    worldstate_model::{
        alert::{Alert, unmapped::AlertUnmapped},
        fissure::{Fissure, unmapped::FissureUnmapped},
    },
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct WorldStateUnmapped {
    #[serde(rename = "ActiveMissions")]
    pub fissures: Vec<FissureUnmapped>,

    pub alerts: Vec<AlertUnmapped>,
}

impl WorldStateUnmapped {
    pub fn map_worldstate(self, ctx: Context<'_>) -> Option<WorldState> {
        let fissures = self.fissures.resolve(ctx);

        let alerts = self.alerts.resolve(ctx);

        Some(WorldState { fissures, alerts })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorldState {
    pub fissures: Vec<Fissure>,

    pub alerts: Vec<Alert>,
}
