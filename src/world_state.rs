use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, Resolve},
    target_types::worldstate::{
        alert::Alert,
        archon_hunt::ArchonHunt,
        event::Event,
        fissure::Fissure,
        flash_sale::FlashSale,
        goal::Goal,
        invasion::Invasion,
        sortie::Sortie,
        syndicate_mission::SyndicateMission,
        void_trader::VoidTrader,
    },
    worldstate_model::{
        alert::AlertUnmapped,
        archon_hunt::ArchonHuntUnmapped,
        event::EventUnmapped,
        fissure::FissureUnmapped,
        flash_sale::FlashSaleUnmapped,
        goal::GoalUnmapped,
        invasion::InvasionUnmapped,
        sortie::SortieUnmapped,
        syndicate_mission::SyndicateMissionUnmapped,
        void_trader::VoidTraderUnmapped,
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

    pub syndicate_missions: Vec<SyndicateMissionUnmapped>,

    pub flash_sales: Vec<FlashSaleUnmapped>,

    pub invasions: Vec<InvasionUnmapped>,

    pub void_traders: Vec<VoidTraderUnmapped>,
}

impl WorldStateUnmapped {
    pub fn map_worldstate(self, ctx: ContextRef<'_>) -> Option<WorldState> {
        let events = self.events.resolve(());
        let fissures = self.fissures.resolve(ctx);
        let alerts = self.alerts.resolve(ctx);
        let sorties = self.sorties.resolve(ctx);
        let goals = self.goals.resolve(ctx);
        let archon_hunt = self.archon_hunt.resolve(ctx);
        let syndicate_missions = self.syndicate_missions.resolve(ctx);
        let flash_sales = self.flash_sales.resolve(());
        let invasions = self.invasions.resolve(ctx);
        let void_trader = self.void_traders.resolve(ctx).into_iter().next();

        Some(WorldState {
            archon_hunt,
            goals,
            events,
            fissures,
            alerts,
            sorties,
            syndicate_missions,
            flash_sales,
            invasions,
            void_trader,
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

    pub syndicate_missions: Vec<SyndicateMission>,

    pub flash_sales: Vec<FlashSale>,

    pub invasions: Vec<Invasion>,

    pub void_trader: Option<VoidTrader>,
}
