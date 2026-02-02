use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::Resolve,
    target_types::worldstate_types::event::{Event, EventLink, EventMessage},
    worldstate_model::{Id, WorldstateLanguage, deserialize_mongo_date_opt},
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventUnmapped {
    #[serde(rename = "_id")]
    id: Id,

    messages: Vec<EventMessageUnmapped>,

    prop: String,

    icon: Option<String>,

    priority: bool,

    mobile_only: bool,

    community: Option<bool>,

    image_url: Option<String>,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_mongo_date_opt")]
    date: Option<DateTime<Utc>>,

    hide_end_date_modifier: Option<bool>,

    #[serde(default)]
    links: Vec<EventLinkUnmapped>,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_mongo_date_opt")]
    event_end_date: Option<DateTime<Utc>>,
}

impl Resolve<()> for EventUnmapped {
    type Output = Event;

    fn resolve(self, _: ()) -> Self::Output {
        Event {
            id: self.id.oid,
            messages: self.messages.resolve(()),
            prop: self.prop,
            icon: self.icon,
            priority: self.priority,
            mobile_only: self.mobile_only,
            community: self.community,
            image_url: self.image_url,
            date: self.date,
            hide_end_date_modifier: self.hide_end_date_modifier,
            links: self.links.resolve(()),
            event_end_date: self.event_end_date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventLinkUnmapped {
    language_code: WorldstateLanguage,

    link: String,
}

impl Resolve<()> for EventLinkUnmapped {
    type Output = EventLink;

    fn resolve(self, _: ()) -> Self::Output {
        EventLink {
            language: self.language_code.resolve(()),
            link: self.link,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventMessageUnmapped {
    language_code: WorldstateLanguage,

    message: String,
}

impl Resolve<()> for EventMessageUnmapped {
    type Output = EventMessage;

    fn resolve(self, _: ()) -> Self::Output {
        EventMessage {
            language: self.language_code.resolve(()),
            message: self.message,
        }
    }
}
