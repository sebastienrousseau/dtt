// Copyright © 2025 DateTime (DTT) library.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! String-form validators for `DateTime` components.
//!
//! All entries are pure functions that mirror the bounds enforced by
//! `DateTime::from_components` and `DateTime::parse`, so the validators
//! and the constructors can never disagree.

use super::{
    DateTime, MAX_DAY, MAX_HOUR, MAX_ISO_WEEK, MAX_MICROSECOND,
    MAX_MIN_SEC, MAX_MONTH, MAX_ORDINAL_DAY,
};

// -----------------------------------------------------------------------------
// Validation Methods
// -----------------------------------------------------------------------------

impl DateTime {
    /// Validates whether a string represents a valid day of the month.
    #[must_use]
    pub fn is_valid_day(day: &str) -> bool {
        day.parse::<u8>().is_ok_and(|d| (1..=MAX_DAY).contains(&d))
    }

    /// Validates whether a string represents a valid hour.
    #[must_use]
    pub fn is_valid_hour(hour: &str) -> bool {
        hour.parse::<u8>().is_ok_and(|h| h <= MAX_HOUR)
    }

    /// Validates whether a string represents a valid minute.
    #[must_use]
    pub fn is_valid_minute(minute: &str) -> bool {
        minute.parse::<u8>().is_ok_and(|m| m <= MAX_MIN_SEC)
    }

    /// Validates whether a string represents a valid second.
    #[must_use]
    pub fn is_valid_second(second: &str) -> bool {
        second.parse::<u8>().is_ok_and(|s| s <= MAX_MIN_SEC)
    }

    /// Validates whether a string represents a valid month.
    #[must_use]
    pub fn is_valid_month(month: &str) -> bool {
        month.parse::<u8>().is_ok_and(|m| (1..=MAX_MONTH).contains(&m))
    }

    /// Validates whether a string represents a valid year within the
    /// supported `time::Date` range of `-9999..=9999`.
    ///
    /// This bound matches what `DateTime::from_components` can actually
    /// construct, so the validator and the builder agree.
    #[must_use]
    pub fn is_valid_year(year: &str) -> bool {
        year.parse::<i32>().is_ok_and(|y| (-9999..=9999).contains(&y))
    }

    /// Validates whether a string represents a valid microsecond.
    #[must_use]
    pub fn is_valid_microsecond(microsecond: &str) -> bool {
        microsecond
            .parse::<u32>()
            .is_ok_and(|us| us <= MAX_MICROSECOND)
    }

    /// Validates whether a string represents a valid ordinal day of the year.
    #[must_use]
    pub fn is_valid_ordinal(ordinal: &str) -> bool {
        ordinal
            .parse::<u16>()
            .is_ok_and(|o| (1..=MAX_ORDINAL_DAY).contains(&o))
    }

    /// Validates whether a string represents a valid ISO week number.
    #[must_use]
    pub fn is_valid_iso_week(week: &str) -> bool {
        week.parse::<u8>()
            .is_ok_and(|w| (1..=MAX_ISO_WEEK).contains(&w))
    }

    /// Validates whether a string represents a valid time in `HH:MM:SS` format.
    #[must_use]
    pub fn is_valid_time(time: &str) -> bool {
        let parts: Vec<&str> = time.split(':').collect();
        if parts.len() != 3 {
            return false;
        }

        Self::is_valid_hour(parts[0])
            && Self::is_valid_minute(parts[1])
            && Self::is_valid_second(parts[2])
    }
}
