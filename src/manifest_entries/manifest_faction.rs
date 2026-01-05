use serde::Serialize;
use serde_repr::Deserialize_repr;

use crate::{core::Resolve, target_types::faction::Faction};

#[derive(Debug, Deserialize_repr, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
#[repr(u8)]
pub enum ManifestFaction {
    Grineer = 0,
    Corpus = 1,
    Infested = 2,
    Orokin = 3,
    Sentient = 5,
    Murmur = 7,
    Scaldra = 8,
    Techrot = 9,
    Duviri = 10,
}

impl Resolve<()> for ManifestFaction {
    type Output = Faction;

    fn resolve(self, _: ()) -> Self::Output {
        match self {
            ManifestFaction::Grineer => Faction::Grineer,
            ManifestFaction::Corpus => Faction::Corpus,
            ManifestFaction::Infested => Faction::Infested,
            ManifestFaction::Orokin => Faction::Orokin,
            ManifestFaction::Sentient => Faction::Sentient,
            ManifestFaction::Murmur => Faction::Murmur,
            ManifestFaction::Scaldra => Faction::Scaldra,
            ManifestFaction::Techrot => Faction::Techrot,
            ManifestFaction::Duviri => Faction::Duviri,
        }
    }
}
