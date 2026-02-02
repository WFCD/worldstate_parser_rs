use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    core::{ContextRef, InternalPath, Resolve, resolve_with},
    target_types::worldstate_types::calendar::{Calendar, CalendarDay, CalendarEvent, CalendarSeason},
    worldstate_model::deserialize_mongo_date,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CalendarUnmapped {
    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub activation: DateTime<Utc>,

    #[serde(deserialize_with = "deserialize_mongo_date")]
    pub expiry: DateTime<Utc>,

    pub days: Vec<CalendarDayUnmapped>,

    pub season: CalendarSeasonUnmapped,

    pub year_iteration: u32,

    pub version: u32,
}

impl Resolve<ContextRef<'_>> for CalendarUnmapped {
    type Output = Calendar;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        Calendar {
            activation: self.activation,
            expiry: self.expiry,
            days: self.days.resolve(ctx),
            season: self.season.resolve(()),
            year_iteration: self.year_iteration,
            version: self.version,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CalendarSeasonUnmapped {
    #[serde(rename = "CST_SUMMER")]
    Summer,

    #[serde(rename = "CST_WINTER")]
    Winter,

    #[serde(rename = "CST_SPRING")]
    Spring,

    #[serde(rename = "CST_FALL")]
    Fall,
}

impl Resolve<()> for CalendarSeasonUnmapped {
    type Output = CalendarSeason;

    fn resolve(self, _ctx: ()) -> Self::Output {
        match self {
            CalendarSeasonUnmapped::Summer => CalendarSeason::Summer,
            CalendarSeasonUnmapped::Winter => CalendarSeason::Winter,
            CalendarSeasonUnmapped::Spring => CalendarSeason::Spring,
            CalendarSeasonUnmapped::Fall => CalendarSeason::Fall,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarDayUnmapped {
    pub day: u32,

    pub events: Vec<CalendarEventUnmapped>,
}

impl Resolve<ContextRef<'_>> for CalendarDayUnmapped {
    type Output = CalendarDay;

    fn resolve(self, ctx: ContextRef<'_>) -> Self::Output {
        let day = NaiveDate::from_yo_opt(1999, self.day)
            .and_then(|d| d.and_hms_opt(0, 0, 0))
            .map(|d| d.and_utc());

        let event = match self.events.first() {
            Some(CalendarEventUnmapped::Challenge { .. }) => {
                self.events.into_iter().next().and_then(|e| match e {
                    CalendarEventUnmapped::Challenge { challenge } => {
                        Some(CalendarEvent::Challenge(challenge.resolve(ctx)))
                    },
                    _ => None,
                })
            },
            Some(CalendarEventUnmapped::Reward { .. }) => self
                .events
                .into_iter()
                .map(|e| match e {
                    CalendarEventUnmapped::Reward { reward } => Some(reward.resolve(ctx)),
                    _ => None,
                })
                .collect::<Option<Vec<_>>>()
                .and_then(|v| v.try_into().ok())
                .map(CalendarEvent::Rewards),
            Some(CalendarEventUnmapped::Upgrade { .. }) => self
                .events
                .into_iter()
                .map(|e| match e {
                    CalendarEventUnmapped::Upgrade { upgrade } => Some(upgrade.resolve(ctx)),
                    _ => None,
                })
                .collect::<Option<Vec<_>>>()
                .and_then(|v| v.try_into().ok())
                .map(CalendarEvent::Upgrades),
            None => None,
        };

        CalendarDay { day, event }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CalendarEventUnmapped {
    #[serde(rename = "CET_CHALLENGE")]
    Challenge {
        challenge: InternalPath<resolve_with::LanguageItemWithDesc>,
    },

    #[serde(rename = "CET_REWARD")]
    Reward {
        reward: InternalPath<resolve_with::CalendarReward>,
    },

    #[serde(rename = "CET_UPGRADE")]
    Upgrade {
        upgrade: InternalPath<resolve_with::LanguageItemWithDesc>,
    },
}
