use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{core::Resolve, target_types::faction::Faction};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortieData {
    pub modifier_types: HashMap<String, String>,

    pub modifier_descriptions: HashMap<String, String>,

    pub bosses: HashMap<String, Boss>,

    pub modifiers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Boss {
    pub name: String,

    pub faction: SortieBossFaction,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SortieBossFaction {
    Corpus,
    Grineer,
    Infestation,
    Corrupted,
    Narmer,
}

impl Resolve<()> for SortieBossFaction {
    type Output = Faction;

    fn resolve(self, _: ()) -> Self::Output {
        match self {
            SortieBossFaction::Corpus => Faction::Corpus,
            SortieBossFaction::Grineer => Faction::Grineer,
            SortieBossFaction::Infestation => Faction::Infested,
            SortieBossFaction::Corrupted => Faction::Orokin,
            SortieBossFaction::Narmer => Faction::Narmer,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::wfcd_worldstate_data::sortie_data::SortieData;

    #[test]
    fn test_deserialize() {
        let _: SortieData =
            serde_json::from_str(&read_to_string("./data/sortieData.json").unwrap()).unwrap();
    }
}
