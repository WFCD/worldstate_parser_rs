use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::target_types::language::Language;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,

    pub messages: Vec<EventMessage>,

    pub prop: String,

    pub icon: Option<String>,

    pub priority: bool,

    pub mobile_only: bool,

    pub community: Option<bool>,

    pub image_url: Option<String>,

    pub date: Option<DateTime<Utc>>,

    pub hide_end_date_modifier: Option<bool>,

    #[serde(default)]
    pub links: Vec<EventLink>,

    pub event_end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EventLink {
    pub language: Language,

    pub link: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EventMessage {
    pub language: Language,

    pub message: String,
}
