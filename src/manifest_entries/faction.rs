use serde::Serialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
#[repr(u8)]
pub enum Faction {
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
