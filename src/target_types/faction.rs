use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, derive_more::Display)]
pub enum Faction {
    Grineer,
    Corpus,
    Infested,
    Orokin,
    Sentient,
    Stalker,
    Narmer,
    Murmur,
    Scaldra,
    Techrot,
    Anarchs,

    /// Not really a faction, but can occur as "Faction"
    Duviri,

    Tenno,
}
