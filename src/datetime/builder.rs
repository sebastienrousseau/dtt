// Copyright © 2025 DateTime (DTT) library.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Fluent builder for [`DateTime`].
//!
//! Lives in its own module so the `mod.rs` file can focus on the
//! `DateTime` type and its core impls.

use super::DateTime;
use crate::error::DateTimeError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use time::UtcOffset;

/// A builder for [`DateTime`] objects, allowing more ergonomic creation of
/// datetimes with customized year, month, day, hour, minute, second, and offset.
///
/// # Examples
///
/// ```
/// use dtt::datetime::{DateTime, DateTimeBuilder};
/// use time::UtcOffset;
///
/// let builder = DateTimeBuilder::new()
///     .year(2024)
///     .month(1)
///     .day(1)
///     .hour(12)
///     .minute(30)
///     .second(45)
///     .offset(UtcOffset::UTC);
///
/// let dt = builder.build();
/// assert!(dt.is_ok());
///
/// let dt_unwrapped = dt.unwrap();
/// assert_eq!(dt_unwrapped.year(), 2024);
/// assert_eq!(dt_unwrapped.month().to_string(), "January");
/// assert_eq!(dt_unwrapped.day(), 1);
/// assert_eq!(dt_unwrapped.hour(), 12);
/// assert_eq!(dt_unwrapped.minute(), 30);
/// assert_eq!(dt_unwrapped.second(), 45);
/// assert_eq!(dt_unwrapped.offset(), UtcOffset::UTC);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DateTimeBuilder {
    /// Calendar year, e.g. 2024.
    year: i32,
    /// Month (1-12).
    month: u8,
    /// Day of the month (1-31, depends on month).
    day: u8,
    /// Hour of the day (0-23).
    hour: u8,
    /// Minute of the hour (0-59).
    minute: u8,
    /// Second of the minute (0-59).
    second: u8,
    /// The time zone offset from UTC.
    offset: UtcOffset,
}

impl Default for DateTimeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DateTimeBuilder {
    /// Creates a new `DateTimeBuilder` with default values set to
    /// midnight, January 1, 1970 (UTC).
    #[must_use]
    pub const fn new() -> Self {
        Self {
            year: 1970,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            offset: UtcOffset::UTC,
        }
    }

    /// Sets the year component.
    #[must_use]
    pub const fn year(mut self, year: i32) -> Self {
        self.year = year;
        self
    }

    /// Sets the month component.
    #[must_use]
    pub const fn month(mut self, month: u8) -> Self {
        self.month = month;
        self
    }

    /// Sets the day component.
    #[must_use]
    pub const fn day(mut self, day: u8) -> Self {
        self.day = day;
        self
    }

    /// Sets the hour component.
    #[must_use]
    pub const fn hour(mut self, hour: u8) -> Self {
        self.hour = hour;
        self
    }

    /// Sets the minute component.
    #[must_use]
    pub const fn minute(mut self, minute: u8) -> Self {
        self.minute = minute;
        self
    }

    /// Sets the second component.
    #[must_use]
    pub const fn second(mut self, second: u8) -> Self {
        self.second = second;
        self
    }

    /// Sets the time zone offset component.
    #[must_use]
    pub const fn offset(mut self, offset: UtcOffset) -> Self {
        self.offset = offset;
        self
    }

    /// Builds the final [`DateTime`] from the builder state.
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if any of the date components are invalid
    /// (e.g., `month = 13` or `day = 32`).
    pub fn build(&self) -> Result<DateTime, DateTimeError> {
        DateTime::from_components(
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.offset,
        )
    }
}
