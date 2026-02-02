pub(crate) mod core;
pub(crate) mod custom_maps;
#[cfg(feature = "default_provider")]
pub mod default_context_provider;
#[cfg(feature = "default_fetcher")]
pub mod default_data_fetcher;
pub(crate) mod manifest_entries;
pub(crate) mod manifests;
pub mod target_types;
pub(crate) mod wfcd_data;
pub mod worldstate;
pub(crate) mod worldstate_model;

pub use crate::{
    target_types::{
        display_info::DisplayInfo,
        faction::Faction,
        language::Language,
        mission_type::MissionType,
        node::Node,
        relic::*,
        worldstate_types::{
            alert::*,
            archimedea::*,
            archon_hunt::*,
            calendar::*,
            circuit::*,
            counted_item::*,
            daily_deal::*,
            event::*,
            fissure::*,
            flash_sale::*,
            goal::*,
            invasion::*,
            nightwave::*,
            sortie::*,
            syndicate::SyndicateType,
            syndicate_mission::*,
            vault_trader::*,
            void_storm::VoidStorm,
            void_trader::*,
        },
        *,
    },
    worldstate::*,
};

pub trait ContextProvider {
    type Err;
    fn get_ctx(&self) -> impl Future<Output = Result<crate::core::Context, Self::Err>> + Send;
}
