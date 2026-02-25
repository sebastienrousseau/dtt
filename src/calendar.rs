// SPDX-License-Identifier: MIT OR Apache-2.0

//! Calendar-level duration mathematics.
//!
//! Provides `CalendarDuration` to execute complex logic when adding months or years
//! (e.g. leap year caps, mapping Jan 31 + 1 month safely to Feb 28/29).

use crate::{error::DateTimeError, DateTime};
use time::Duration;

/// Represents a calendar-aware duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CalendarDuration {
    months: i32,
    days: i32,
}

impl CalendarDuration {
    /// Creates a duration of standard months.
    #[must_use]
    pub const fn months(count: i32) -> Self {
        Self {
            months: count,
            days: 0,
        }
    }

    /// Creates a duration of standard calendar years.
    #[must_use]
    pub const fn years(count: i32) -> Self {
        Self {
            months: count * 12,
            days: 0,
        }
    }
}

impl DateTime {
    /// Applies a `CalendarDuration` to the DateTime.
    /// Safely handles month boundary clamping.
    ///
    /// # Errors
    /// Returns `DateTimeError` if the resulting computation overflows calendar boundaries.
    pub fn add_calendar(
        &self,
        duration: CalendarDuration,
    ) -> Result<Self, DateTimeError> {
        let mut year = self.year();
        let mut month = self.month() as i32;

        month += duration.months;

        while month > 12 {
            year += 1;
            month -= 12;
        }
        while month < 1 {
            year -= 1;
            month += 12;
        }

        let is_leap =
            (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let max_days = match month {
            4 | 6 | 9 | 11 => 30,
            2 => {
                if is_leap {
                    29
                } else {
                    28
                }
            }
            _ => 31,
        };

        let next_day = std::cmp::min(self.day(), max_days);

        // Add residual calendar days natively using standard bounds
        let intermediate = DateTime::from_components(
            year,
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            { month as u8 },
            next_day,
            self.hour(),
            self.minute(),
            self.second(),
            self.offset(),
        )?;

        intermediate.plus(Duration::days(i64::from(duration.days)))
    }
}
