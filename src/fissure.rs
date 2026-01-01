use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

use crate::{
    manifests::ExportRegions,
    model::{mission_type::MissionType, tier::Tier},
    region::Region,
};

#[derive(Debug)]
pub struct Fissure {
    pub id: Id,

    pub region: i64,

    pub seed: i64,

    pub activation: DateTime<Utc>,

    pub expiry: DateTime<Utc>,

    pub node: Region,

    pub mission_type: MissionType,

    pub tier: Tier,

    pub is_steel_path: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FissureUnmapped {
    #[serde(rename = "_id")]
    pub id: Id,

    pub region: i64,

    pub seed: i64,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub node: String,

    pub mission_type: MissionType,

    pub modifier: Tier,

    #[serde(default)]
    pub hard: bool,
}

impl FissureUnmapped {
    pub fn map(self, export: &ExportRegions) -> Option<Fissure> {
        let Self {
            id,
            region,
            seed,
            activation,
            expiry,
            node,
            mission_type,
            modifier,
            hard,
        } = self;

        let node = export
            .export_regions
            .iter()
            .find(|region| region.unique_name == node)
            .cloned()?;

        Some(Fissure {
            id,
            region,
            seed,
            activation,
            expiry,
            mission_type,
            tier: modifier,
            node,
            is_steel_path: hard,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct Id {
    #[serde(rename = "$oid")]
    pub oid: String,
}

fn deserialize_mongo_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct MongoDate {
        #[serde(rename = "$date")]
        date: MongoDateInner,
    }

    #[derive(Deserialize)]
    struct MongoDateInner {
        #[serde(rename = "$numberLong")]
        number_long: String,
    }

    let v = MongoDate::deserialize(deserializer)?;
    let millis = v
        .date
        .number_long
        .parse::<i64>()
        .map_err(serde::de::Error::custom)?;

    Utc.timestamp_millis_opt(millis)
        .single()
        .ok_or_else(|| serde::de::Error::custom("invalid timestamp"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization() {
        let json_data = r#"
        {
            "_id": { "$oid": "603f8a0a1a8c1234567890ab" },
            "Region": 1,
            "Seed": 12345,
            "Activation": {
                "$date": {
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
