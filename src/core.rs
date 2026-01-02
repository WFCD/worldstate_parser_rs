use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer};

use crate::{custom_maps::CustomMaps, manifests::Exports};

pub fn deserialize_mongo_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
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

pub trait Mappable {
    type MapTo;

    fn map(self, export: &Exports, custom_maps: &CustomMaps) -> Option<Self::MapTo>;
}
