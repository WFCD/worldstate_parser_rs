use serde::{Deserialize, Serialize};

use crate::{
    ContextProvider,
    core::{ContextRef, Resolve},
    target_types::worldstate_types::{
        alert::Alert,
        archimedea::ArchimedeaRoot,
        archon_hunt::ArchonHunt,
        calendar::Calendar,
        circuit::Circuit,
        daily_deal::DailyDeal,
        event::Event,
        fissure::Fissure,
        flash_sale::FlashSale,
        goal::Goal,
        invasion::Invasion,
        nightwave::Nightwave,
        sortie::Sortie,
        syndicate_mission::SyndicateMission,
        vault_trader::VaultTrader,
        void_storm::VoidStorm,
        void_trader::VoidTraderState,
    },
    worldstate_model::{
        alert::AlertUnmapped,
        archimedea::ArchimedeaUnmapped,
        archon_hunt::ArchonHuntUnmapped,
        calendar::CalendarUnmapped,
        circuit::CircuitUnmapped,
        daily_deal::DailyDealUnmapped,
        event::EventUnmapped,
        fissure::FissureUnmapped,
        flash_sale::FlashSaleUnmapped,
        goal::GoalUnmapped,
        invasion::InvasionUnmapped,
        nightwave::NightwaveUnmapped,
        sortie::SortieUnmapped,
        syndicate_mission::SyndicateMissionUnmapped,
        vault_trader::VaultTraderUnmapped,
        void_storms::VoidStormUnmapped,
        void_trader::VoidTraderStateUnmapped,
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

    pub void_traders: Vec<VoidTraderStateUnmapped>,

    pub prime_vault_traders: Vec<VaultTraderUnmapped>,

    pub void_storms: Vec<VoidStormUnmapped>,

    pub daily_deals: Vec<DailyDealUnmapped>,

    #[serde(rename = "EndlessXpChoices")]
    pub circuit: [CircuitUnmapped; 2],

    #[serde(rename = "SeasonInfo")]
    pub nightwave: NightwaveUnmapped,

    #[serde(rename = "KnownCalendarSeasons")]
    pub calendars: Vec<CalendarUnmapped>,

    #[serde(rename = "Conquests")]
    pub archimedea: Vec<ArchimedeaUnmapped>,
}

impl WorldStateUnmapped {
    pub fn map(self, ctx: ContextRef<'_>) -> WorldState {
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
        let vault_trader = self.prime_vault_traders.resolve(ctx).into_iter().next();
        let void_storms = self.void_storms.resolve(ctx);
        let daily_deals = self.daily_deals.resolve(ctx);
        let circuit = self.circuit.resolve(());
        let nightwave = self.nightwave.resolve(ctx);
        let calendar = self.calendars.resolve(ctx).into_iter().next();
        let archimedea = self.archimedea.resolve(ctx);

        WorldState {
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
            vault_trader,
            void_storms,
            daily_deals,
            circuit,
            nightwave,
            calendar,
            archimedea,
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub enum WorldstateError {
    WorldstateDeserialization(#[from] serde_json::Error),

    Provider(Box<dyn std::error::Error + Send + Sync>),
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

    pub void_trader: Option<VoidTraderState>,

    pub vault_trader: Option<VaultTrader>,

    pub void_storms: Vec<VoidStorm>,

    pub daily_deals: Vec<DailyDeal>,

    pub circuit: Circuit,

    pub nightwave: Nightwave,

    pub calendar: Option<Calendar>,

    pub archimedea: ArchimedeaRoot,
}

impl WorldState {
    pub async fn from_str<C>(s: &str, provider: C) -> Result<WorldState, WorldstateError>
    where
        C: ContextProvider,
        C::Err: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        let ws_unmapped = serde_json::from_str::<WorldStateUnmapped>(s)?;

        let ctx = provider
            .get_ctx()
            .await
            .map_err(|err| WorldstateError::Provider(err.into()))?;

        Ok(ws_unmapped.map(ctx.as_ref()))
    }
}
