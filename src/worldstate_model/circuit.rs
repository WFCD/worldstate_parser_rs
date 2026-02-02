use serde::{Deserialize, Serialize};

use crate::{
    core::{Resolve, resolvable_string::ResolvableString, resolve_with},
    target_types::worldstate_types::circuit::Circuit,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CategoryUnmapped {
    #[serde(rename = "EXC_NORMAL")]
    Normal,
    #[serde(rename = "EXC_HARD")]
    Hard,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CircuitUnmapped {
    category: CategoryUnmapped,

    choices: Vec<ResolvableString<resolve_with::TitleCase>>,
}

impl Resolve<()> for [CircuitUnmapped; 2] {
    type Output = Circuit;

    fn resolve(self, _ctx: ()) -> Self::Output {
        let mut normal = None;
        let mut steel = None;

        for circuit in self {
            match circuit.category {
                CategoryUnmapped::Normal => normal = Some(circuit.choices.resolve(())),
                CategoryUnmapped::Hard => steel = Some(circuit.choices.resolve(())),
            }
        }

        Circuit {
            // Unwrapping here is safe because you "know" they are both there
            normal_choices: normal.expect("Missing Normal"),
            steel_path_choices: steel.expect("Missing Hard"),
        }
    }
}
