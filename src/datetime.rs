// datetime.rs
//
// Copyright © 2025 DateTime (DTT) library.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! DateTime module for managing dates, times, and timezones in Rust.
//!
//! # Overview
//!
//! This module provides a comprehensive datetime manipulation API that includes:
//! - Fixed offset timezone support
//! - Date and time creation and parsing
//! - Format conversion (RFC 3339, ISO 8601)
//! - Date arithmetic and comparison operations  
//! - Validation utilities
//!
//! **Note**: Daylight Saving Time (DST) is **not automatically handled**. Users must
//! manually manage DST transitions by selecting appropriate timezone offsets.
//!
//! # Examples
//!
//! ```rust
//! use dtt::datetime::DateTime;
//!
//! // Create current UTC time
//! let now = DateTime::new();
//!
//! // Parse specific datetime
//! let maybe_dt = DateTime::parse("2024-01-01T12:00:00Z");
//! if let Ok(dt) = maybe_dt {
//!     // Convert timezone
//!     let est = dt.convert_to_tz("EST");
//!     if let Ok(est_dt) = est {
//!         // ...
//!     }
//! }
//! ```

#![deny(
    missing_docs,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::result_unit_err,
    clippy::clone_on_ref_ptr
)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

use crate::error::DateTimeError;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt,
    hash::{Hash, Hasher},
    ops::{Add, Sub},
    str::FromStr,
};
use time::{
    format_description, Date, Duration, Month, OffsetDateTime,
    PrimitiveDateTime, Time, UtcOffset, Weekday,
};

/// Maximum valid hour value (0-23)
const MAX_HOUR: u8 = 23;

/// Maximum valid minute/second value (0-59)
const MAX_MIN_SEC: u8 = 59;

/// Maximum valid day value (1-31)
const MAX_DAY: u8 = 31;

/// Maximum valid month value (1-12)
const MAX_MONTH: u8 = 12;

/// Maximum valid microsecond value (0-999_999)
const MAX_MICROSECOND: u32 = 999_999;

/// Maximum valid ISO week number (1-53)
const MAX_ISO_WEEK: u8 = 53;

/// Maximum valid ordinal day (1-366)
const MAX_ORDINAL_DAY: u16 = 366;

/// Represents a date and time with timezone offset support.
///
/// This struct combines a UTC datetime with a timezone offset, allowing for
/// timezone-aware datetime operations. While it supports fixed offsets,
/// it does **not** automatically handle DST transitions.
///
/// # Examples
///
/// ```
/// use dtt::datetime::DateTime;
///
/// let utc = DateTime::new();
/// let maybe_est = utc.convert_to_tz("EST");
/// if let Ok(est) = maybe_est {
///     // ...
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DateTime {
    /// The date and time in UTC (when offset = `UtcOffset::UTC`) or a
    /// user-chosen offset if `offset != UtcOffset::UTC`.
    pub datetime: PrimitiveDateTime,
    /// The timezone offset from UTC.
    pub offset: UtcOffset,
}

lazy_static::lazy_static! {
    /// Static mapping of timezone abbreviations to their `UtcOffset`.
    ///
    /// # Note
    ///
    /// This is not an exhaustive list of timezones. It is a convenient subset
    /// for demonstration purposes. Real-world usage might integrate a
    /// more robust timezone library or database.
    static ref TIMEZONE_OFFSETS: HashMap<&'static str, Result<UtcOffset, DateTimeError>> = {
        let mut m = HashMap::new();
        let _ = m.insert("UTC", Ok(UtcOffset::UTC));
        let _ = m.insert("GMT", Ok(UtcOffset::UTC));

        // North American time zones
        let _ = m.insert("EST", UtcOffset::from_hms(-5, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("EDT", UtcOffset::from_hms(-4, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("CST", UtcOffset::from_hms(-6, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("CDT", UtcOffset::from_hms(-5, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("MST", UtcOffset::from_hms(-7, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("MDT", UtcOffset::from_hms(-6, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("PST", UtcOffset::from_hms(-8, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("PDT", UtcOffset::from_hms(-7, 0, 0).map_err(DateTimeError::from));

        // European time zones
        let _ = m.insert("CET", UtcOffset::from_hms(1, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("CEST", UtcOffset::from_hms(2, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("EET", UtcOffset::from_hms(2, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("EEST", UtcOffset::from_hms(3, 0, 0).map_err(DateTimeError::from));

        // Asian time zones
        let _ = m.insert("JST", UtcOffset::from_hms(9, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("IST", UtcOffset::from_hms(5, 30, 0).map_err(DateTimeError::from));
        let _ = m.insert("HKT", UtcOffset::from_hms(8, 0, 0).map_err(DateTimeError::from));

        // Australian time zones
        let _ = m.insert("AEDT", UtcOffset::from_hms(11, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert("AEST", UtcOffset::from_hms(10, 0, 0).map_err(DateTimeError::from));
        let _ = m.insert(
            "WADT",
            UtcOffset::from_hms(8, 45, 0)
                .map_err(DateTimeError::from)
        );

        m
    };
}

// -----------------------------------------------------------------------------
// Builder Pattern
// -----------------------------------------------------------------------------

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
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
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

// -----------------------------------------------------------------------------
// Core Implementations
// -----------------------------------------------------------------------------

impl DateTime {
    // -------------------------------------------------------------------------
    // Creation Methods
    // -------------------------------------------------------------------------

    /// Creates a new `DateTime` instance representing the current UTC time.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let now = DateTime::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        // Directly obtain the current UTC time.
        let now = OffsetDateTime::now_utc();
        Self {
            datetime: PrimitiveDateTime::new(now.date(), now.time()),
            offset: UtcOffset::UTC,
        }
    }

    /// Creates a new `DateTime` instance with the current time in the specified timezone.
    ///
    /// # Arguments
    ///
    /// * `tz` - A timezone abbreviation (e.g., "UTC", "EST", "PST")
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` instance or a `DateTimeError`
    /// if the timezone is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let maybe_est_time = DateTime::new_with_tz("EST");
    /// if let Ok(est_time) = maybe_est_time {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the timezone is invalid.
    ///
    pub fn new_with_tz(tz: &str) -> Result<Self, DateTimeError> {
        let offset = TIMEZONE_OFFSETS
            .get(tz)
            .ok_or(DateTimeError::InvalidTimezone)?
            .as_ref()
            .map_err(Clone::clone)?;

        let now_utc = OffsetDateTime::now_utc();
        let now_local = now_utc.to_offset(*offset);

        Ok(Self {
            datetime: PrimitiveDateTime::new(
                now_local.date(),
                now_local.time(),
            ),
            offset: *offset,
        })
    }

    /// Creates a new `DateTime` instance with a custom UTC offset.
    ///
    /// # Arguments
    ///
    /// * `hours` - Hour offset from UTC (-23 to +23)
    /// * `minutes` - Minute offset from UTC (-59 to +59)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`
    /// if the offset is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// // Create time with UTC+5:30 offset (e.g., for India)
    /// let maybe_ist = DateTime::new_with_custom_offset(5, 30);
    /// if let Ok(ist) = maybe_ist {
    ///     // ...
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the timezone is invalid.
    ///
    pub fn new_with_custom_offset(
        hours: i8,
        minutes: i8,
    ) -> Result<Self, DateTimeError> {
        // Direct numeric checks (no casts needed)
        if hours.abs() > 23 || minutes.abs() > 59 {
            return Err(DateTimeError::InvalidTimezone);
        }

        let offset = UtcOffset::from_hms(hours, minutes, 0)
            .map_err(|_| DateTimeError::InvalidTimezone)?;

        let now_utc = OffsetDateTime::now_utc();
        let now_local = now_utc.to_offset(offset);

        Ok(Self {
            datetime: PrimitiveDateTime::new(
                now_local.date(),
                now_local.time(),
            ),
            offset,
        })
    }

    /// Returns a new `DateTime` which is exactly one day earlier.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `DateTime` or a `DateTimeError`
    /// if subtracting one day would result in an invalid date.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let now = DateTime::new();
    /// let maybe_yesterday = now.previous_day();
    /// assert!(maybe_yesterday.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the resulting date would be invalid.
    ///
    pub fn previous_day(&self) -> Result<Self, DateTimeError> {
        self.add_days(-1)
    }

    /// Returns a new `DateTime` which is exactly one day later.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `DateTime` or a `DateTimeError`
    /// if adding one day would result in an invalid date.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let now = DateTime::new();
    /// let maybe_tomorrow = now.next_day();
    /// assert!(maybe_tomorrow.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the resulting date would be invalid.
    ///
    pub fn next_day(&self) -> Result<Self, DateTimeError> {
        self.add_days(1)
    }

    /// Sets the time components (hour, minute, second) while preserving the current date
    /// and timezone offset.
    ///
    /// # Arguments
    ///
    /// * `hour` - Hour (0-23)
    /// * `minute` - Minute (0-59)
    /// * `second` - Second (0-59)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`
    /// if the time components are invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// // Attempt to set the time to 10:30:45
    /// let updated_dt = dt.set_time(10, 30, 45);
    /// assert!(updated_dt.is_ok());
    /// if let Ok(new_val) = updated_dt {
    ///     assert_eq!(new_val.hour(), 10);
    ///     assert_eq!(new_val.minute(), 30);
    ///     assert_eq!(new_val.second(), 45);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the resulting time would be invalid.
    ///
    pub fn set_time(
        &self,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, DateTimeError> {
        // Construct a new time; returns an error if invalid
        let new_time = Time::from_hms(hour, minute, second)
            .map_err(|_| DateTimeError::InvalidTime)?;

        // Preserve the existing date
        Ok(Self {
            datetime: PrimitiveDateTime::new(
                self.datetime.date(),
                new_time,
            ),
            offset: self.offset,
        })
    }

    /// Subtracts a specified number of years from the `DateTime`.
    ///
    /// Handles leap year transitions appropriately (e.g., if subtracting a year from
    /// Feb 29 results in Feb 28).
    ///
    /// # Arguments
    ///
    /// * `years` - Number of years to subtract
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`
    /// if the resulting date would be invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let maybe_past = dt.sub_years(1);
    /// assert!(maybe_past.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the resulting date would be invalid.
    ///
    pub fn sub_years(&self, years: i32) -> Result<Self, DateTimeError> {
        self.add_years(-years)
    }

    /// Converts this `DateTime` to another timezone, then formats it
    /// using the provided `format_str`.
    ///
    /// # Arguments
    ///
    /// * `tz` - Target timezone abbreviation (e.g., "UTC", "EST", "PST").
    /// * `format_str` - A format description (see the `time` crate documentation
    ///   for the supported syntax).
    ///
    /// # Returns
    ///
    /// Returns a `Result<String, DateTimeError>` containing either
    /// the formatted datetime string or an error if conversion or
    /// formatting fails.
    ///
    /// # Errors
    ///
    /// This function will return a [`DateTimeError`] if:
    /// - The specified timezone is not recognized or invalid.
    /// - The formatting operation fails due to an invalid `format_str`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let result = dt.format_time_in_timezone("EST", "[hour]:[minute]:[second]");
    /// if let Ok(formatted_str) = result {
    ///     println!("Time in EST: {}", formatted_str);
    /// }
    /// ```
    pub fn format_time_in_timezone(
        &self,
        tz: &str,
        format_str: &str,
    ) -> Result<String, DateTimeError> {
        // 1. Convert this DateTime to the specified timezone
        let dt_tz = self.convert_to_tz(tz)?;

        // 2. Format the timezone-adjusted DateTime using the provided format string
        dt_tz.format(format_str)
    }

    /// Returns `true` if the input string is a valid ISO 8601 or RFC 3339–like datetime/date.
    ///
    /// # Arguments
    ///
    /// * `input` - A string that might represent a date or datetime in ISO 8601/RFC 3339 format.
    ///
    /// # Returns
    ///
    /// `true` if the string can be successfully parsed as either:
    ///   - RFC 3339 datetime (e.g., "2024-01-01T12:00:00Z"), or
    ///   - ISO 8601 date (e.g., "2024-01-01")
    ///     `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// assert!(DateTime::is_valid_iso_8601("2024-01-01T12:00:00Z"));
    /// assert!(DateTime::is_valid_iso_8601("2024-01-01"));
    /// assert!(!DateTime::is_valid_iso_8601("2024-13-01")); // invalid month
    /// assert!(!DateTime::is_valid_iso_8601("not a date"));
    /// ```
    #[must_use]
    pub fn is_valid_iso_8601(input: &str) -> bool {
        // 1. Try parsing the string as RFC 3339 (a strict subset of ISO 8601).
        if PrimitiveDateTime::parse(
            input,
            &format_description::well_known::Rfc3339,
        )
        .is_ok()
        {
            return true;
        }

        // 2. Otherwise, try parsing as just the date portion of ISO 8601 (yyyy-mm-dd).
        if Date::parse(
            input,
            &format_description::well_known::Iso8601::DATE,
        )
        .is_ok()
        {
            return true;
        }

        // 3. If both attempts fail, it's not a valid ISO 8601 or RFC 3339 datetime/date.
        false
    }

    /// Creates a `DateTime` instance from individual components.
    ///
    /// # Arguments
    ///
    /// * `year` - Calendar year
    /// * `month` - Month (1-12)
    /// * `day` - Day of month (1-31, depending on month)
    /// * `hour` - Hour (0-23)
    /// * `minute` - Minute (0-59)
    /// * `second` - Second (0-59)
    /// * `offset` - Timezone offset from UTC
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`
    /// if any component is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// use time::UtcOffset;
    ///
    /// let dt = DateTime::from_components(2024, 1, 1, 12, 0, 0, UtcOffset::UTC);
    /// assert!(dt.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if any component is invalid.
    ///
    pub fn from_components(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        offset: UtcOffset,
    ) -> Result<Self, DateTimeError> {
        let month = Month::try_from(month)
            .map_err(|_| DateTimeError::InvalidDate)?;
        let date = Date::from_calendar_date(year, month, day)
            .map_err(|_| DateTimeError::InvalidDate)?;
        let time = Time::from_hms(hour, minute, second)
            .map_err(|_| DateTimeError::InvalidTime)?;

        Ok(Self {
            datetime: PrimitiveDateTime::new(date, time),
            offset,
        })
    }

    // -------------------------------------------------------------------------
    // Getter Methods
    // -------------------------------------------------------------------------

    /// Returns the year component of the `DateTime`.
    #[must_use]
    pub const fn year(&self) -> i32 {
        self.datetime.date().year()
    }

    /// Returns the month component of the `DateTime`.
    #[must_use]
    pub const fn month(&self) -> Month {
        self.datetime.date().month()
    }

    /// Returns the day component of the `DateTime`.
    #[must_use]
    pub const fn day(&self) -> u8 {
        self.datetime.date().day()
    }

    /// Returns the hour component of the `DateTime`.
    #[must_use]
    pub const fn hour(&self) -> u8 {
        self.datetime.time().hour()
    }

    /// Returns the minute component of the `DateTime`.
    #[must_use]
    pub const fn minute(&self) -> u8 {
        self.datetime.time().minute()
    }

    /// Returns the second component of the `DateTime`.
    #[must_use]
    pub const fn second(&self) -> u8 {
        self.datetime.time().second()
    }

    /// Returns the microsecond component of the `DateTime`.
    #[must_use]
    pub const fn microsecond(&self) -> u32 {
        self.datetime.microsecond()
    }

    /// Returns the ISO week component of the `DateTime`.
    #[must_use]
    pub const fn iso_week(&self) -> u8 {
        self.datetime.iso_week()
    }

    /// Returns the ordinal day (day of year) component of the `DateTime`.
    #[must_use]
    pub const fn ordinal(&self) -> u16 {
        self.datetime.ordinal()
    }

    /// Returns the timezone offset of the `DateTime`.
    #[must_use]
    pub const fn offset(&self) -> UtcOffset {
        self.offset
    }

    /// Returns the weekday of the `DateTime`.
    #[must_use]
    pub const fn weekday(&self) -> Weekday {
        self.datetime.date().weekday()
    }

    // -------------------------------------------------------------------------
    // Parsing Methods
    // -------------------------------------------------------------------------

    /// Parses a string representation of a date and time.
    ///
    /// Supports both RFC 3339 and ISO 8601 formats.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice containing the date/time to parse
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the parsed `DateTime` or a `DateTimeError`
    /// if parsing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// // Parse RFC 3339 format
    /// let dt1 = DateTime::parse("2024-01-01T12:00:00Z");
    ///
    /// // Parse ISO 8601 date
    /// let dt2 = DateTime::parse("2024-01-01");
    /// assert!(dt1.is_ok());
    /// assert!(dt2.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the input string is not a valid date/time.
    ///
    pub fn parse(input: &str) -> Result<Self, DateTimeError> {
        // Try RFC 3339 format first
        if let Ok(dt) = PrimitiveDateTime::parse(
            input,
            &format_description::well_known::Rfc3339,
        ) {
            return Ok(Self {
                datetime: dt,
                offset: UtcOffset::UTC,
            });
        }

        // Fall back to ISO 8601 date format
        if let Ok(date) = Date::parse(
            input,
            &format_description::well_known::Iso8601::DATE,
        ) {
            return Ok(Self {
                datetime: PrimitiveDateTime::new(date, Time::MIDNIGHT),
                offset: UtcOffset::UTC,
            });
        }

        Err(DateTimeError::InvalidFormat)
    }

    /// Parses a date/time string using a custom format specification.
    ///
    /// # Arguments
    ///
    /// * `input` - The date/time string to parse
    /// * `format` - Format specification string (see `time` crate documentation)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the parsed `DateTime` or a `DateTimeError`
    /// if parsing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::parse_custom_format(
    ///     "2024-01-01 12:00:00",
    ///     "[year]-[month]-[day] [hour]:[minute]:[second]"
    /// );
    /// assert!(dt.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the input string is not a valid date/time.
    ///
    pub fn parse_custom_format(
        input: &str,
        format: &str,
    ) -> Result<Self, DateTimeError> {
        let format_desc = format_description::parse(format)
            .map_err(|_| DateTimeError::InvalidFormat)?;
        let datetime = PrimitiveDateTime::parse(input, &format_desc)
            .map_err(|_| DateTimeError::InvalidFormat)?;

        Ok(Self {
            datetime,
            offset: UtcOffset::UTC,
        })
    }

    // -------------------------------------------------------------------------
    // Formatting Methods
    // -------------------------------------------------------------------------

    /// Formats the `DateTime` according to the specified format string.
    ///
    /// # Arguments
    ///
    /// * `format_str` - Format specification string (see `time` crate documentation)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the formatted string or a `DateTimeError`
    /// if formatting fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let formatted = dt.format("[year]-[month]-[day]");
    /// assert!(formatted.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the format string is invalid.
    ///
    pub fn format(
        &self,
        format_str: &str,
    ) -> Result<String, DateTimeError> {
        let format_desc = format_description::parse(format_str)
            .map_err(|_| DateTimeError::InvalidFormat)?;
        self.datetime
            .format(&format_desc)
            .map_err(|_| DateTimeError::InvalidFormat)
    }

    /// Formats the `DateTime` as an RFC 3339 string.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the formatted RFC 3339 string
    /// or a `DateTimeError` if formatting fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let maybe_rfc3339 = dt.format_rfc3339();
    /// assert!(maybe_rfc3339.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if formatting fails.
    ///
    pub fn format_rfc3339(&self) -> Result<String, DateTimeError> {
        self.datetime
            .assume_offset(self.offset)
            .format(&format_description::well_known::Rfc3339)
            .map_err(|_| DateTimeError::InvalidFormat)
    }

    /// Formats the `DateTime` as an ISO 8601 string (YYYY-MM-DDTHH:MM:SS).
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the formatted ISO 8601 string
    /// or a `DateTimeError` if formatting fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let maybe_iso8601 = dt.format_iso8601();
    /// assert!(maybe_iso8601.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if formatting fails.
    ///
    pub fn format_iso8601(&self) -> Result<String, DateTimeError> {
        self.format("[year]-[month]-[day]T[hour]:[minute]:[second]")
    }

    /// Updates the `DateTime` to the current time while preserving the timezone offset.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the updated `DateTime` or a `DateTimeError`
    /// if the update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let dt = DateTime::new();
    /// sleep(Duration::from_secs(1));
    /// let updated_dt = dt.update();
    /// assert!(updated_dt.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the update fails.
    ///
    pub fn update(&self) -> Result<Self, DateTimeError> {
        let now = OffsetDateTime::now_utc().to_offset(self.offset);
        Ok(Self {
            datetime: PrimitiveDateTime::new(now.date(), now.time()),
            offset: self.offset,
        })
    }

    // -------------------------------------------------------------------------
    // Timezone Conversion Method
    // -------------------------------------------------------------------------

    /// Converts the current `DateTime` to another timezone.
    ///
    /// # Arguments
    ///
    /// * `new_tz` - Target timezone abbreviation (e.g., "UTC", "EST", "PST")
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the `DateTime` in the new timezone
    /// or a `DateTimeError` if the conversion fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let utc = DateTime::new();
    /// let maybe_est = utc.convert_to_tz("EST");
    /// assert!(maybe_est.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the timezone is invalid.
    ///
    pub fn convert_to_tz(
        &self,
        new_tz: &str,
    ) -> Result<Self, DateTimeError> {
        let new_offset = TIMEZONE_OFFSETS
            .get(new_tz)
            .ok_or(DateTimeError::InvalidTimezone)?
            .as_ref()
            .map_err(Clone::clone)?;

        let datetime_with_offset =
            self.datetime.assume_offset(self.offset);
        let new_datetime = datetime_with_offset.to_offset(*new_offset);

        Ok(Self {
            datetime: PrimitiveDateTime::new(
                new_datetime.date(),
                new_datetime.time(),
            ),
            offset: *new_offset,
        })
    }

    // -------------------------------------------------------------------------
    // Additional Utilities
    // -------------------------------------------------------------------------

    /// Gets the Unix timestamp (seconds since Unix epoch).
    ///
    /// # Returns
    ///
    /// Returns the number of seconds from the Unix epoch (1970-01-01T00:00:00Z).
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let ts = dt.unix_timestamp();
    /// ```
    #[must_use]
    pub const fn unix_timestamp(&self) -> i64 {
        self.datetime.assume_offset(self.offset).unix_timestamp()
    }

    /// Calculates the duration between this `DateTime` and another.
    ///
    /// The result can be negative if `other` is later than `self`.
    ///
    /// # Arguments
    ///
    /// * `other` - The `DateTime` to compare with
    ///
    /// # Returns
    ///
    /// Returns a `Duration` representing the time difference.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt1 = DateTime::new();
    /// let dt2 = dt1.add_days(1).unwrap_or(dt1);
    /// let duration = dt1.duration_since(&dt2);
    /// // duration could be negative if dt2 > dt1
    /// ```
    #[must_use]
    pub fn duration_since(&self, other: &Self) -> Duration {
        let self_offset = self.datetime.assume_offset(self.offset);
        let other_offset = other.datetime.assume_offset(other.offset);

        let seconds_diff = self_offset.unix_timestamp()
            - other_offset.unix_timestamp();
        let nanos_diff = i64::from(self_offset.nanosecond())
            - i64::from(other_offset.nanosecond());

        Duration::seconds(seconds_diff)
            + Duration::nanoseconds(nanos_diff)
    }

    // -------------------------------------------------------------------------
    // Date Arithmetic Methods
    // -------------------------------------------------------------------------

    /// Adds a specified number of days to the `DateTime`.
    ///
    /// # Arguments
    ///
    /// * `days` - Number of days to add (can be negative for subtraction)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`
    /// if the operation would result in an invalid date.
    ///
    /// # Errors
    ///
    /// This function returns a [`DateTimeError::InvalidDate`] if adding `days` results
    /// in a date overflow or otherwise invalid date.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let future = dt.add_days(7);
    /// assert!(future.is_ok());
    /// ```
    pub fn add_days(&self, days: i64) -> Result<Self, DateTimeError> {
        let new_datetime = self
            .datetime
            .checked_add(Duration::days(days))
            .ok_or(DateTimeError::InvalidDate)?;

        Ok(Self {
            datetime: new_datetime,
            offset: self.offset,
        })
    }

    /// Adds a specified number of months to the `DateTime`.
    ///
    /// Handles month-end dates and leap years appropriately.
    ///
    /// # Arguments
    ///
    /// * `months` - Number of months to add (can be negative for subtraction)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`
    /// if the operation would result in an invalid date.
    ///
    /// # Errors
    ///
    /// This function returns a [`DateTimeError`] if:
    /// - The calculated year, month, or day is invalid (e.g., out of range).
    /// - The underlying date library fails to construct a valid date.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let future = dt.add_months(3);
    /// assert!(future.is_ok());
    /// ```
    pub fn add_months(
        &self,
        months: i32,
    ) -> Result<Self, DateTimeError> {
        let current_date = self.datetime.date();
        let total_months =
            current_date.year() * 12 + current_date.month() as i32 - 1
                + months;

        let target_year = total_months / 12;
        let target_month = u8::try_from((total_months % 12) + 1);

        let target_month =
            target_month.map_err(|_| DateTimeError::InvalidDate)?;
        let days_in_target_month =
            days_in_month(target_year, target_month)?;
        let target_day = current_date.day().min(days_in_target_month);

        let new_month = Month::try_from(target_month)
            .map_err(|_| DateTimeError::InvalidDate)?;
        let new_date = Date::from_calendar_date(
            target_year,
            new_month,
            target_day,
        )
        .map_err(|_| DateTimeError::InvalidDate)?;

        Ok(Self {
            datetime: PrimitiveDateTime::new(
                new_date,
                self.datetime.time(),
            ),
            offset: self.offset,
        })
    }

    /// Subtracts a specified number of months from the `DateTime`.
    ///
    /// # Arguments
    ///
    /// * `months` - Number of months to subtract
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`
    /// if the operation would result in an invalid date.
    ///
    /// # Errors
    ///
    /// This function returns a [`DateTimeError::InvalidDate`] if:
    /// - The resulting date is out of valid range.
    /// - The underlying date library fails to construct a valid `DateTime`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let past = dt.sub_months(3);
    /// assert!(past.is_ok());
    /// ```
    pub fn sub_months(
        &self,
        months: i32,
    ) -> Result<Self, DateTimeError> {
        self.add_months(-months)
    }

    /// Adds a specified number of years to the `DateTime`.
    ///
    /// Handles leap-year transitions appropriately.
    ///
    /// # Arguments
    ///
    /// * `years` - Number of years to add (can be negative for subtraction)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`
    /// if the operation would result in an invalid date.
    ///
    /// # Errors
    ///
    /// This function returns a [`DateTimeError::InvalidDate`] if:
    /// - The resulting year is out of valid range.
    /// - A non-leap year cannot accommodate February 29th.
    /// - Any other invalid date scenario occurs during calculation.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let future = dt.add_years(5);
    /// assert!(future.is_ok());
    /// ```
    pub fn add_years(&self, years: i32) -> Result<Self, DateTimeError> {
        let current_date = self.datetime.date();
        let target_year = current_date
            .year()
            .checked_add(years)
            .ok_or(DateTimeError::InvalidDate)?;

        // Handle February 29th in leap years
        let new_day = if current_date.month() == Month::February
            && current_date.day() == 29
            && !is_leap_year(target_year)
        {
            28
        } else {
            current_date.day()
        };

        let new_date = Date::from_calendar_date(
            target_year,
            current_date.month(),
            new_day,
        )
        .map_err(|_| DateTimeError::InvalidDate)?;

        Ok(Self {
            datetime: PrimitiveDateTime::new(
                new_date,
                self.datetime.time(),
            ),
            offset: self.offset,
        })
    }

    // -------------------------------------------------------------------------
    // Range / Boundary Helper Methods
    // -------------------------------------------------------------------------

    /// Returns a new `DateTime` for the start of the current week (Monday).
    ///
    /// # Errors
    ///
    /// This function can return a [`DateTimeError`] if an overflow or
    /// invalid date calculation occurs during date arithmetic.
    pub fn start_of_week(&self) -> Result<Self, DateTimeError> {
        let days_since_monday = i64::from(
            self.datetime.weekday().number_days_from_monday(),
        );
        self.add_days(-days_since_monday)
    }

    /// Returns a new `DateTime` for the end of the current week (Sunday).
    ///
    /// # Errors
    ///
    /// This function can return a [`DateTimeError`] if an overflow or
    /// invalid date calculation occurs during date arithmetic.
    pub fn end_of_week(&self) -> Result<Self, DateTimeError> {
        let days_until_sunday = 6 - i64::from(
            self.datetime.weekday().number_days_from_monday(),
        );
        self.add_days(days_until_sunday)
    }

    /// Returns a new `DateTime` for the start of the current month.
    ///
    /// # Errors
    ///
    /// This function can return a [`DateTimeError`] if the date cannot be
    /// constructed (e.g., due to an invalid year or month).
    pub fn start_of_month(&self) -> Result<Self, DateTimeError> {
        self.set_date(
            self.datetime.year(),
            self.datetime.month() as u8,
            1,
        )
    }

    /// Returns a new `DateTime` for the end of the current month.
    ///
    /// # Errors
    ///
    /// This function can return a [`DateTimeError`] if the date cannot be
    /// constructed (e.g., `days_in_month` fails to provide a valid day).
    pub fn end_of_month(&self) -> Result<Self, DateTimeError> {
        let year = self.datetime.year();
        let month = self.datetime.month() as u8;
        let last_day = days_in_month(year, month)?;
        self.set_date(year, month, last_day)
    }

    /// Returns a new `DateTime` for the start of the current year.
    ///
    /// # Errors
    ///
    /// This function can return a [`DateTimeError`] if the date cannot
    /// be constructed (e.g., invalid year).
    pub fn start_of_year(&self) -> Result<Self, DateTimeError> {
        self.set_date(self.datetime.year(), 1, 1)
    }

    /// Returns a new `DateTime` for the end of the current year.
    ///
    /// # Errors
    ///
    /// This function can return a [`DateTimeError`] if the date cannot
    /// be constructed (e.g., invalid year).
    pub fn end_of_year(&self) -> Result<Self, DateTimeError> {
        self.set_date(self.datetime.year(), 12, 31)
    }

    // -------------------------------------------------------------------------
    // Range Validation
    // -------------------------------------------------------------------------

    /// Checks if the current `DateTime` falls within a specific date range (inclusive).
    ///
    /// # Arguments
    ///
    /// * `start` - Start of the date range (inclusive)
    /// * `end` - End of the date range (inclusive)
    ///
    /// # Returns
    ///
    /// Returns `true` if the current `DateTime` falls within the range, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let start = dt.add_days(-1).unwrap_or(dt);
    /// let end = dt.add_days(1).unwrap_or(dt);
    ///
    /// assert!(dt.is_within_range(&start, &end));
    /// ```
    #[must_use]
    pub fn is_within_range(&self, start: &Self, end: &Self) -> bool {
        self >= start && self <= end
    }

    // -------------------------------------------------------------------------
    // Mutation Helpers
    // -------------------------------------------------------------------------

    /// Sets the date components while maintaining the current time.
    ///
    /// # Arguments
    ///
    /// * `year` - Calendar year
    /// * `month` - Month (1-12)
    /// * `day` - Day of month (1-31)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`
    /// if the date is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let new_dt = dt.set_date(2024, 1, 1);
    /// assert!(new_dt.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `DateTimeError` if the resulting date would be invalid.
    ///
    pub fn set_date(
        &self,
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Self, DateTimeError> {
        let month = Month::try_from(month)
            .map_err(|_| DateTimeError::InvalidDate)?;
        let new_date = Date::from_calendar_date(year, month, day)
            .map_err(|_| DateTimeError::InvalidDate)?;

        Ok(Self {
            datetime: PrimitiveDateTime::new(
                new_date,
                self.datetime.time(),
            ),
            offset: self.offset,
        })
    }
}

// -----------------------------------------------------------------------------
// Validation Methods
// -----------------------------------------------------------------------------

impl DateTime {
    /// Validates whether a string represents a valid day of the month.
    #[must_use]
    pub fn is_valid_day(day: &str) -> bool {
        day.parse::<u8>()
            .map(|d| (1..=MAX_DAY).contains(&d))
            .unwrap_or(false)
    }

    /// Validates whether a string represents a valid hour.
    #[must_use]
    pub fn is_valid_hour(hour: &str) -> bool {
        hour.parse::<u8>().map(|h| h <= MAX_HOUR).unwrap_or(false)
    }

    /// Validates whether a string represents a valid minute.
    #[must_use]
    pub fn is_valid_minute(minute: &str) -> bool {
        minute
            .parse::<u8>()
            .map(|m| m <= MAX_MIN_SEC)
            .unwrap_or(false)
    }

    /// Validates whether a string represents a valid second.
    #[must_use]
    pub fn is_valid_second(second: &str) -> bool {
        second
            .parse::<u8>()
            .map(|s| s <= MAX_MIN_SEC)
            .unwrap_or(false)
    }

    /// Validates whether a string represents a valid month.
    #[must_use]
    pub fn is_valid_month(month: &str) -> bool {
        month
            .parse::<u8>()
            .map(|m| (1..=MAX_MONTH).contains(&m))
            .unwrap_or(false)
    }

    /// Validates whether a string represents a valid year.
    #[must_use]
    pub fn is_valid_year(year: &str) -> bool {
        year.parse::<i32>().is_ok()
    }

    /// Validates whether a string represents a valid microsecond.
    #[must_use]
    pub fn is_valid_microsecond(microsecond: &str) -> bool {
        microsecond
            .parse::<u32>()
            .map(|us| us <= MAX_MICROSECOND)
            .unwrap_or(false)
    }

    /// Validates whether a string represents a valid ordinal day of the year.
    #[must_use]
    pub fn is_valid_ordinal(ordinal: &str) -> bool {
        ordinal
            .parse::<u16>()
            .map(|o| (1..=MAX_ORDINAL_DAY).contains(&o))
            .unwrap_or(false)
    }

    /// Validates whether a string represents a valid ISO week number.
    #[must_use]
    pub fn is_valid_iso_week(week: &str) -> bool {
        week.parse::<u8>()
            .map(|w| (1..=MAX_ISO_WEEK).contains(&w))
            .unwrap_or(false)
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

// -----------------------------------------------------------------------------
// Standard Trait Implementations
// -----------------------------------------------------------------------------

impl fmt::Display for DateTime {
    /// Formats the `DateTime` using RFC 3339 format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.format_rfc3339()
            .map_or(Err(fmt::Error), |s| write!(f, "{s}"))
    }
}

impl FromStr for DateTime {
    type Err = DateTimeError;

    /// Parses a string into a `DateTime` instance (RFC 3339 or ISO 8601).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Default for DateTime {
    /// Returns the current UTC time as the default `DateTime` value.
    fn default() -> Self {
        Self::new()
    }
}

impl Add<Duration> for DateTime {
    type Output = Result<Self, DateTimeError>;

    /// Adds a Duration to the `DateTime`.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Duration to add
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`.
    fn add(self, rhs: Duration) -> Self::Output {
        let maybe_new = self.datetime.checked_add(rhs);
        maybe_new.map_or(
            Err(DateTimeError::InvalidDate),
            |new_datetime| {
                Ok(Self {
                    datetime: new_datetime,
                    offset: self.offset,
                })
            },
        )
    }
}

impl Sub<Duration> for DateTime {
    type Output = Result<Self, DateTimeError>;

    /// Subtracts a Duration from the `DateTime`.
    ///
    /// # Arguments
    ///
    /// * `rhs` - Duration to subtract
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either the new `DateTime` or a `DateTimeError`.
    fn sub(self, rhs: Duration) -> Self::Output {
        let maybe_new = self.datetime.checked_sub(rhs);
        maybe_new.map_or(
            Err(DateTimeError::InvalidDate),
            |new_datetime| {
                Ok(Self {
                    datetime: new_datetime,
                    offset: self.offset,
                })
            },
        )
    }
}

impl PartialOrd for DateTime {
    /// Compares two `DateTime` for ordering, returning `Some(Ordering)`.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    /// Compares two `DateTimes` for ordering.
    fn cmp(&self, other: &Self) -> Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl Hash for DateTime {
    /// Computes a hash value for the `DateTime` based on its components.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.datetime.hash(state);
        self.offset.hash(state);
    }
}

// -----------------------------------------------------------------------------
// Helper Functions
// -----------------------------------------------------------------------------

/// Helper function to determine the number of days in a given month and year.
///
/// # Arguments
///
/// * `year` - Calendar year
/// * `month` - Month number (1-12)
///
/// # Returns
///
/// Returns a `Result` containing either the number of days or a `DateTimeError`.
///
/// # Errors
///
/// Returns a `DateTimeError` if the day in the month is invalid.
///
pub const fn days_in_month(
    year: i32,
    month: u8,
) -> Result<u8, DateTimeError> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
        4 | 6 | 9 | 11 => Ok(30),
        2 => Ok(if is_leap_year(year) { 29 } else { 28 }),
        _ => Err(DateTimeError::InvalidDate),
    }
}

/// Helper function to determine if a year is a leap year.
///
/// # Arguments
///
/// * `year` - Calendar year to check
///
/// # Returns
///
/// Returns `true` if the year is a leap year, `false` otherwise.
///
/// # Examples
///
/// ```
/// use dtt::datetime::is_leap_year;
///
/// assert!(is_leap_year(2024));
/// assert!(!is_leap_year(2023));
/// assert!(is_leap_year(2000));
/// assert!(!is_leap_year(1900));
/// ```
#[must_use]
pub const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// -----------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_new() {
        let dt = DateTime::new();
        assert_eq!(dt.offset(), UtcOffset::UTC);
    }

    #[test]
    fn test_new_with_tz() {
        let est = DateTime::new_with_tz("EST");
        assert!(est.is_ok());
        if let Ok(est_dt) = est {
            assert_eq!(est_dt.offset().whole_hours(), -5);
        }

        let invalid = DateTime::new_with_tz("INVALID");
        assert!(matches!(invalid, Err(DateTimeError::InvalidTimezone)));
    }

    #[test]
    fn test_new_with_custom_offset() {
        let offset = DateTime::new_with_custom_offset(5, 30);
        assert!(offset.is_ok());
        if let Ok(dt) = offset {
            assert_eq!(dt.offset().whole_hours(), 5);
            assert_eq!(dt.offset().minutes_past_hour(), 30);
        }

        // Test invalid offsets
        let too_large_hours = DateTime::new_with_custom_offset(24, 0);
        assert!(too_large_hours.is_err());
        let too_large_minutes = DateTime::new_with_custom_offset(0, 60);
        assert!(too_large_minutes.is_err());
    }

    #[test]
    fn test_from_components() {
        let dt = DateTime::from_components(
            2024,
            1,
            1,
            12,
            0,
            0,
            UtcOffset::UTC,
        );
        assert!(dt.is_ok());
        if let Ok(dt_val) = dt {
            assert_eq!(dt_val.year(), 2024);
            assert_eq!(dt_val.month(), Month::January);
            assert_eq!(dt_val.day(), 1);
            assert_eq!(dt_val.hour(), 12);
            assert_eq!(dt_val.minute(), 0);
            assert_eq!(dt_val.second(), 0);
        }

        // Test invalid dates
        let invalid_month = DateTime::from_components(
            2024,
            13,
            1,
            0,
            0,
            0,
            UtcOffset::UTC,
        );
        assert!(invalid_month.is_err());

        let invalid_day = DateTime::from_components(
            2024,
            2,
            30,
            0,
            0,
            0,
            UtcOffset::UTC,
        );
        assert!(invalid_day.is_err());
    }

    #[test]
    fn test_parse() {
        // Test RFC 3339 format
        let dt = DateTime::parse("2024-01-01T12:00:00Z");
        assert!(dt.is_ok());

        // Test ISO 8601 date
        let dt = DateTime::parse("2024-01-01");
        assert!(dt.is_ok());
        if let Ok(dt_val) = dt {
            assert_eq!(dt_val.hour(), 0);
            assert_eq!(dt_val.minute(), 0);
        }

        // Test invalid formats
        let invalid1 = DateTime::parse("invalid");
        assert!(invalid1.is_err());
        let invalid2 = DateTime::parse("2024-13-01");
        assert!(invalid2.is_err());
    }

    #[test]
    fn test_format() {
        let dt = DateTime::new();
        let maybe_formatted = dt.format("[year]-[month]-[day]");
        assert!(maybe_formatted.is_ok());

        let invalid_format = dt.format("[invalid]");
        assert!(invalid_format.is_err());
    }

    #[test]
    fn test_timezone_conversion() {
        let utc = DateTime::new();
        let est = utc.convert_to_tz("EST");
        assert!(est.is_ok());
        if let Ok(est_val) = est {
            assert_eq!(est_val.offset().whole_hours(), -5);
        }

        let invalid = utc.convert_to_tz("INVALID");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_arithmetic() {
        let dt = DateTime::new();

        // Test adding days
        let future = dt.add_days(7);
        assert!(future.is_ok());

        // Test subtracting days (negative)
        let past = dt.add_days(-7);
        assert!(past.is_ok());

        // Test adding months
        let next_month = dt.add_months(1);
        assert!(next_month.is_ok());

        // Test month edge cases
        let jan31 = DateTime::from_components(
            2024,
            1,
            31,
            0,
            0,
            0,
            UtcOffset::UTC,
        );
        assert!(jan31.is_ok());
        if let Ok(jan31_dt) = jan31 {
            let feb = jan31_dt.add_months(1);
            assert!(feb.is_ok());
            if let Ok(feb_dt) = feb {
                // 2024 is a leap year => Feb has 29 days
                assert_eq!(feb_dt.day(), 29);
            }
        }
    }

    #[test]
    fn test_leap_year() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
    }

    #[test]
    fn test_validation() {
        // Test day validation
        assert!(DateTime::is_valid_day("1"));
        assert!(DateTime::is_valid_day("31"));
        assert!(!DateTime::is_valid_day("0"));
        assert!(!DateTime::is_valid_day("32"));
        assert!(!DateTime::is_valid_day("abc"));

        // Test hour validation
        assert!(DateTime::is_valid_hour("0"));
        assert!(DateTime::is_valid_hour("23"));
        assert!(!DateTime::is_valid_hour("24"));

        // Test minute validation
        assert!(DateTime::is_valid_minute("0"));
        assert!(DateTime::is_valid_minute("59"));
        assert!(!DateTime::is_valid_minute("60"));

        // Test time string validation
        assert!(DateTime::is_valid_time("00:00:00"));
        assert!(DateTime::is_valid_time("23:59:59"));
        assert!(!DateTime::is_valid_time("24:00:00"));
        assert!(!DateTime::is_valid_time("23:60:00"));
        assert!(!DateTime::is_valid_time("23:59:60"));
    }

    #[test]
    fn test_range_operations() {
        let dt = DateTime::from_components(
            2024,
            1,
            15,
            12,
            0,
            0,
            UtcOffset::UTC,
        );
        assert!(dt.is_ok());
        if let Ok(dt_val) = dt {
            // Test week ranges
            let week_start = dt_val.start_of_week();
            assert!(week_start.is_ok());
            if let Ok(ws) = week_start {
                assert_eq!(ws.weekday(), Weekday::Monday);
            }

            let week_end = dt_val.end_of_week();
            assert!(week_end.is_ok());
            if let Ok(we) = week_end {
                assert_eq!(we.weekday(), Weekday::Sunday);
            }

            // Test month ranges
            let month_start = dt_val.start_of_month();
            assert!(month_start.is_ok());
            if let Ok(ms) = month_start {
                assert_eq!(ms.day(), 1);
            }

            let month_end = dt_val.end_of_month();
            assert!(month_end.is_ok());
            if let Ok(me) = month_end {
                assert_eq!(me.day(), 31);
            }

            // Test year ranges
            let year_start = dt_val.start_of_year();
            assert!(year_start.is_ok());
            if let Ok(ys) = year_start {
                assert_eq!(ys.month(), Month::January);
                assert_eq!(ys.day(), 1);
            }

            let year_end = dt_val.end_of_year();
            assert!(year_end.is_ok());
            if let Ok(ye) = year_end {
                assert_eq!(ye.month(), Month::December);
                assert_eq!(ye.day(), 31);
            }
        }
    }

    #[test]
    fn test_ordering() {
        let dt1 = DateTime::from_components(
            2024,
            1,
            1,
            0,
            0,
            0,
            UtcOffset::UTC,
        );
        let dt2 = DateTime::from_components(
            2024,
            1,
            2,
            0,
            0,
            0,
            UtcOffset::UTC,
        );

        assert!(dt1.is_ok());
        assert!(dt2.is_ok());
        if let (Ok(a), Ok(b)) = (dt1, dt2) {
            assert!(a < b);
            assert!(b > a);
            assert_ne!(a, b);
        }
    }

    #[test]
    fn test_duration() {
        let dt1 = DateTime::from_components(
            2024,
            1,
            1,
            0,
            0,
            0,
            UtcOffset::UTC,
        );
        let dt2 = DateTime::from_components(
            2024,
            1,
            2,
            0,
            0,
            0,
            UtcOffset::UTC,
        );

        if let (Ok(a), Ok(b)) = (dt1, dt2) {
            let duration = b.duration_since(&a);
            assert_eq!(duration.whole_days(), 1);
        }
    }

    #[test]
    fn test_from_str() {
        let dt = DateTime::from_str("2024-01-01T00:00:00Z");
        assert!(dt.is_ok());
        let invalid = DateTime::from_str("invalid");
        assert!(invalid.is_err());
    }

    #[test]
    fn test_display() {
        let dt = DateTime::from_components(
            2024,
            1,
            1,
            0,
            0,
            0,
            UtcOffset::UTC,
        );
        assert!(dt.is_ok());
        if let Ok(dt_val) = dt {
            assert_eq!(dt_val.to_string(), "2024-01-01T00:00:00Z");
        }
    }

    #[test]
    fn test_hash() {
        use std::collections::HashSet;
        let dt1 = DateTime::from_components(
            2024,
            1,
            1,
            0,
            0,
            0,
            UtcOffset::UTC,
        );
        let dt2 = DateTime::from_components(
            2024,
            1,
            1,
            0,
            0,
            0,
            UtcOffset::UTC,
        );
        assert!(dt1.is_ok());
        assert!(dt2.is_ok());
        if let (Ok(a), Ok(b)) = (dt1, dt2) {
            let mut set = HashSet::new();
            assert!(
                set.insert(a),
                "The set should not have contained `a` before"
            );
            assert!(set.contains(&b));
        }
    }

    #[test]
    fn test_builder_pattern() {
        let builder = DateTimeBuilder::new()
            .year(2024)
            .month(1)
            .day(1)
            .hour(12)
            .minute(30)
            .second(45)
            .offset(UtcOffset::UTC);

        let dt = builder.build();
        assert!(dt.is_ok());
        if let Ok(value) = dt {
            assert_eq!(value.year(), 2024);
            assert_eq!(value.month(), Month::January);
            assert_eq!(value.day(), 1);
            assert_eq!(value.hour(), 12);
            assert_eq!(value.minute(), 30);
            assert_eq!(value.second(), 45);
        }
    }
}
