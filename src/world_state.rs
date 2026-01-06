use serde::{Deserialize, Serialize};

use crate::{
    core::{Context, Resolve},
    target_types::worldstate::{
        alert::Alert,
        archon_hunt::ArchonHunt,
        event::Event,
        fissure::Fissure,
        goal::Goal,
        sortie::Sortie,
    },
    worldstate_model::{
        alert::AlertUnmapped,
        archon_hunt::ArchonHuntUnmapped,
        event::EventUnmapped,
        fissure::FissureUnmapped,
        goal::GoalUnmapped,
        sortie::SortieUnmapped,
    },
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct WorldStateUnmapped {
    pub events: Vec<EventUnmapped>,

    #[serde(rename = "ActiveMissions")]
    pub fissures: Vec<FissureUnmapped>,

    pub alerts: Vec<AlertUnmapped>,

    pub sorties: Vec<SortieUnmapped>,

    pub goals: Vec<GoalUnmapped>,

    #[serde(rename = "LiteSorties")]
    pub archon_hunt: Vec<ArchonHuntUnmapped>,
}

impl WorldStateUnmapped {
    pub fn map_worldstate(self, ctx: Context<'_>) -> Option<WorldState> {
        let events = self.events.resolve(());
        let fissures = self.fissures.resolve(ctx);
        let alerts = self.alerts.resolve(ctx);
        let sorties = self.sorties.resolve(ctx);
        let goals = self.goals.resolve(ctx);
        let archon_hunt = self.archon_hunt.resolve(ctx);

        Some(WorldState {
            archon_hunt,
            goals,
            events,
            fissures,
            alerts,
            sorties,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WorldState {
    pub events: Vec<Event>,

    pub fissures: Vec<Fissure>,

    pub alerts: Vec<Alert>,

    pub sorties: Vec<Sortie>,

    pub goals: Vec<Goal>,

    pub archon_hunt: Vec<ArchonHunt>,
}
