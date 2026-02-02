use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Circuit {
    pub normal_choices: Vec<String>,

    pub steel_path_choices: Vec<String>,
}
