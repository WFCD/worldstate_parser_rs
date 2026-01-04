use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::custom_maps::solnode_to_region::Region;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Tier {
    #[serde(rename(deserialize = "VoidT1"))]
    Lith,
    #[serde(rename(deserialize = "VoidT2"))]
    Meso,
    #[serde(rename(deserialize = "VoidT3"))]
    Neo,
    #[serde(rename(deserialize = "VoidT4"))]
    Axi,
    #[serde(rename(deserialize = "VoidT5"))]
    Requiem,
    #[serde(rename(deserialize = "VoidT6"))]
    Omnia,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Fissure {
    pub id: String,

    pub region: Option<Region>,

    pub seed: usize,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub tier: Tier,

    pub is_steel_path: bool,
}

pub(crate) mod unmapped {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use crate::{
        core::{Context, Resolve, SolNode},
        worldstate_model::{
            Id,
            deserialize_mongo_date,
            fissure::{Fissure, Tier},
        },
    };
    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub(crate) struct FissureUnmapped {
        #[serde(rename = "_id")]
        pub id: Id,

        pub seed: usize,

        pub node: SolNode,

        #[serde(deserialize_with = "deserialize_mongo_date")]
        pub activation: DateTime<Utc>,

        #[serde(deserialize_with = "deserialize_mongo_date")]
        pub expiry: DateTime<Utc>,

        pub modifier: Tier,

        #[serde(default)]
        pub hard: bool,
    }

    impl Resolve<Context<'_>> for FissureUnmapped {
        type Output = Fissure;

        fn resolve(self, ctx: Context) -> Self::Output {
            Fissure {
                id: self.id.oid,
                region: self.node.resolve(ctx).cloned(),
                seed: self.seed,
                activation: self.activation,
                expiry: self.expiry,
                tier: self.modifier,
                is_steel_path: self.hard,
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::worldstate_model::fissure::unmapped::FissureUnmapped;

    #[test]
    fn test_deserialization() {
        let json_data = r#"
        {
            "_id": { "$oid": "603f8a0a1a8c1234567890ab" },
            "Region": 1,
            "Seed": 12345,
            "Activation": {
                "$date": {kkkkkk
                    "$numberLong": "1614776842000"
                }
            },
            "Expiry": {
                "$date": {
                    "$numberLong": "1614780442000"
                }
            },
            "Node": "node_name",
            "MissionType": "mission_type",
            "Modifier": "modifier"
        }
        "#;

        let fissure: FissureUnmapped =
            serde_json::from_str(json_data).expect("Failed to deserialize");

        assert_eq!(fissure.activation.timestamp_millis(), 1614776842000);
        assert_eq!(fissure.expiry.timestamp_millis(), 1614780442000);
        assert_eq!(fissure.id.oid, "603f8a0a1a8c1234567890ab");
    }
}
