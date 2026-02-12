use chrono::{DateTime, Duration, NaiveDate, Timelike, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::target_types::cycles::Cycle;

/// AKA "Mood"
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Display)]
pub enum DuviriState {
    Joy,
    Anger,
    Envy,
    Sorrow,
    Fear,
}

impl DuviriState {
    pub fn next(self) -> Self {
        match self {
            DuviriState::Joy => DuviriState::Anger,
            DuviriState::Anger => DuviriState::Envy,
            DuviriState::Envy => DuviriState::Sorrow,
            DuviriState::Sorrow => DuviriState::Fear,
            DuviriState::Fear => DuviriState::Joy,
        }
    }

    pub fn from_index(index: usize) -> Self {
        match index % 5 {
            0 => DuviriState::Joy,
            1 => DuviriState::Anger,
            2 => DuviriState::Envy,
            3 => DuviriState::Sorrow,
            4 => DuviriState::Fear,
            _ => unreachable!(),
        }
    }
}

pub type DuviriCycle = Cycle<DuviriState>;

impl DuviriCycle {
    pub const MOOD_DURATION: Duration = Duration::hours(2);
    const TOTAL_CYCLE: Duration = Duration::hours(Self::MOOD_DURATION.num_hours() * 5);

    const KNOWN_JOY_START: DateTime<Utc> = NaiveDate::from_ymd_opt(2026, 2, 4)
        .unwrap()
        .and_hms_opt(22, 0, 0)
        .unwrap()
        .and_utc();

    pub fn now() -> Self {
        Self::at(Utc::now().with_nanosecond(0).unwrap())
    }

    pub fn at(time: DateTime<Utc>) -> Self {
        let elapsed_secs = time.timestamp() - Self::KNOWN_JOY_START.timestamp();
        let cycle_offset = elapsed_secs.rem_euclid(Self::TOTAL_CYCLE.num_seconds());

        let mood_idx = (cycle_offset / Self::MOOD_DURATION.num_seconds()) as usize;
        let state = DuviriState::from_index(mood_idx);

        let time_into_current_mood = cycle_offset % Self::MOOD_DURATION.num_seconds();
        let activation = time - Duration::seconds(time_into_current_mood);
        let expiry = activation + Self::MOOD_DURATION;

        Self {
            state,
            activation,
            expiry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DuviriCycle;

    #[test]
    fn test() {
        dbg!(DuviriCycle::now().time_left());
        dbg!(DuviriCycle::now());
    }
}
