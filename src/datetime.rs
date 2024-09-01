// datetime.rs
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! A DateTime module for handling dates, times, and timezones.
//!
//! This module provides a `DateTime` struct and associated methods for
//! creating, manipulating, and formatting date and time information.
//! It supports various timezones using fixed UTC offsets, but does not
//! automatically adjust for daylight saving time (DST). Users must manually
//! manage any necessary DST adjustments by selecting the correct timezone offset.

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
    Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time,
    UtcOffset, Weekday,
};

/// A structure representing a date and time with timezone information.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DateTime {
    /// datetime: The date and time in UTC.
    pub datetime: PrimitiveDateTime,
    /// offset: The timezone offset in UTC.
    pub offset: UtcOffset,
}

// A static HashMap containing timezone abbreviations and their UTC offsets.
lazy_static::lazy_static! {
    static ref TIMEZONE_OFFSETS: HashMap<&'static str, Result<UtcOffset, DateTimeError>> = {
        let mut m = HashMap::new();
        m.insert("ACDT", UtcOffset::from_hms(10, 30, 0).map_err(DateTimeError::from));
        m.insert("ACST", UtcOffset::from_hms(9, 30, 0).map_err(DateTimeError::from));
        m.insert("ADT", UtcOffset::from_hms(-3, 0, 0).map_err(DateTimeError::from));
        m.insert("AEDT", UtcOffset::from_hms(11, 0, 0).map_err(DateTimeError::from));
        m.insert("AEST", UtcOffset::from_hms(10, 0, 0).map_err(DateTimeError::from));
        m.insert("AKDT", UtcOffset::from_hms(-8, 0, 0).map_err(DateTimeError::from));
        m.insert("AKST", UtcOffset::from_hms(-9, 0, 0).map_err(DateTimeError::from));
        m.insert("AST", UtcOffset::from_hms(-4, 0, 0).map_err(DateTimeError::from));
        m.insert("AWST", UtcOffset::from_hms(8, 0, 0).map_err(DateTimeError::from));
        m.insert("BST", UtcOffset::from_hms(1, 0, 0).map_err(DateTimeError::from));
        m.insert("CDT", UtcOffset::from_hms(-5, 0, 0).map_err(DateTimeError::from));
        m.insert("CEST", UtcOffset::from_hms(2, 0, 0).map_err(DateTimeError::from));
        m.insert("CET", UtcOffset::from_hms(1, 0, 0).map_err(DateTimeError::from));
        m.insert("CST", UtcOffset::from_hms(-6, 0, 0).map_err(DateTimeError::from));
        m.insert("ECT", UtcOffset::from_hms(-4, 0, 0).map_err(DateTimeError::from));
        m.insert("EDT", UtcOffset::from_hms(-4, 0, 0).map_err(DateTimeError::from));
        m.insert("EEST", UtcOffset::from_hms(3, 0, 0).map_err(DateTimeError::from));
        m.insert("EET", UtcOffset::from_hms(2, 0, 0).map_err(DateTimeError::from));
        m.insert("EST", UtcOffset::from_hms(-5, 0, 0).map_err(DateTimeError::from));
        m.insert("GMT", Ok(UtcOffset::UTC));
        m.insert("UTC", Ok(UtcOffset::UTC));
        m.insert("HADT", UtcOffset::from_hms(-9, 0, 0).map_err(DateTimeError::from));
        m.insert("HAST", UtcOffset::from_hms(-10, 0, 0).map_err(DateTimeError::from));
        m.insert("HKT", UtcOffset::from_hms(8, 0, 0).map_err(DateTimeError::from));
        m.insert("IST", UtcOffset::from_hms(5, 30, 0).map_err(DateTimeError::from));
        m.insert("IDT", UtcOffset::from_hms(3, 0, 0).map_err(DateTimeError::from));
        m.insert("JST", UtcOffset::from_hms(9, 0, 0).map_err(DateTimeError::from));
        m.insert("KST", UtcOffset::from_hms(9, 0, 0).map_err(DateTimeError::from));
        m.insert("MDT", UtcOffset::from_hms(-6, 0, 0).map_err(DateTimeError::from));
        m.insert("MST", UtcOffset::from_hms(-7, 0, 0).map_err(DateTimeError::from));
        m.insert("NZDT", UtcOffset::from_hms(13, 0, 0).map_err(DateTimeError::from));
        m.insert("NZST", UtcOffset::from_hms(12, 0, 0).map_err(DateTimeError::from));
        m.insert("PDT", UtcOffset::from_hms(-7, 0, 0).map_err(DateTimeError::from));
        m.insert("PST", UtcOffset::from_hms(-8, 0, 0).map_err(DateTimeError::from));
        m.insert("WADT", UtcOffset::from_hms(8, 45, 0).map_err(DateTimeError::from));
        m.insert("WAST", UtcOffset::from_hms(7, 0, 0).map_err(DateTimeError::from));
        m.insert("WEDT", UtcOffset::from_hms(1, 0, 0).map_err(DateTimeError::from));
        m.insert("WEST", UtcOffset::from_hms(1, 0, 0).map_err(DateTimeError::from));
        m.insert("WET", Ok(UtcOffset::UTC));
        m
    };
}

impl DateTime {
    /// Creates a new `DateTime` instance with the current date and time in UTC.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// println!("Current UTC time: {}", dt);
    /// ```
    pub fn new() -> Self {
        Self::new_with_tz("UTC").expect("UTC is always valid")
    }

    /// Creates a new `DateTime` instance with the current date and time in the specified timezone.
    ///
    /// # Arguments
    ///
    /// * `tz` - A string slice that holds the timezone abbreviation (e.g., "UTC", "EST", "PST").
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - The new `DateTime` instance or an error if the timezone is invalid.
    ///
    /// # Note
    ///
    /// This function supports only fixed UTC offsets. Daylight Saving Time (DST) is not automatically
    /// adjusted. If DST adjustments are needed, they must be handled manually by choosing the correct
    /// timezone offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new_with_tz("EST").expect("EST is a valid timezone");
    /// println!("Current time in EST: {}", dt);
    /// ```

    pub fn new_with_tz(tz: &str) -> Result<Self, DateTimeError> {
        let offset = *TIMEZONE_OFFSETS
            .get(tz)
            .ok_or(DateTimeError::InvalidTimezone)?;

        let now_utc = OffsetDateTime::now_utc();
        let now_with_offset = now_utc.to_offset(offset?);

        Ok(Self {
            datetime: PrimitiveDateTime::new(
                now_with_offset.date(),
                now_with_offset.time(),
            ),
            offset: offset?,
        })
    }

    /// Creates a new `DateTime` instance with the current date and time in a custom timezone offset.
    ///
    /// # Arguments
    ///
    /// * `hours` - The hour component of the offset.
    /// * `minutes` - The minute component of the offset.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - The new `DateTime` instance or an error if the offset is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new_with_custom_offset(5, 30).expect("Valid custom offset");
    /// println!("Current time with custom offset: {}", dt);
    /// ```
    pub fn new_with_custom_offset(
        hours: i8,
        minutes: i8,
    ) -> Result<Self, DateTimeError> {
        if hours.abs() > 23 || minutes.abs() > 59 {
            return Err(DateTimeError::InvalidTimezone);
        }

        let offset = UtcOffset::from_hms(hours, minutes, 0)
            .map_err(|_| DateTimeError::InvalidTimezone)?;
        let now_utc = OffsetDateTime::now_utc();
        let now_with_offset = now_utc.to_offset(offset);

        Ok(Self {
            datetime: PrimitiveDateTime::new(
                now_with_offset.date(),
                now_with_offset.time(),
            ),
            offset,
        })
    }

    /// Creates a `DateTime` instance from individual components.
    ///
    /// # Arguments
    ///
    /// * `year` - The year to set.
    /// * `month` - The month to set (1-12).
    /// * `day` - The day of the month to set.
    /// * `hour` - The hour to set (0-23).
    /// * `minute` - The minute to set (0-59).
    /// * `second` - The second to set (0-59).
    /// * `offset` - The `UtcOffset` to set.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance created from the specified components, or an error if any component is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// use time::UtcOffset;
    ///
    /// let dt = DateTime::from_components(2023, 5, 20, 15, 30, 0, UtcOffset::UTC).expect("Valid date components");
    /// println!("Created date: {}", dt);
    /// ```
    pub fn from_components(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        offset: UtcOffset,
    ) -> Result<Self, DateTimeError> {
        let date = Date::from_calendar_date(
            year,
            Month::try_from(month)
                .map_err(|_| DateTimeError::InvalidDate)?,
            day,
        )
        .map_err(|_| DateTimeError::InvalidDate)?;
        let time = Time::from_hms(hour, minute, second)
            .map_err(|_| DateTimeError::InvalidTime)?;
        Ok(Self {
            datetime: PrimitiveDateTime::new(date, time),
            offset,
        })
    }

    /// Parses a string into a `DateTime` instance.
    ///
    /// This method supports RFC 3339 and ISO 8601 date formats.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the date/time to parse.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - The parsed `DateTime` instance or an error if parsing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::parse("2023-05-20T15:30:00Z").expect("Valid RFC 3339 date");
    /// println!("Parsed date: {}", dt);
    /// ```
    pub fn parse(input: &str) -> Result<Self, DateTimeError> {
        let (datetime, offset) = if let Ok(dt) =
            PrimitiveDateTime::parse(
                input,
                &time::format_description::well_known::Rfc3339,
            ) {
            (dt, UtcOffset::UTC)
        } else if let Ok(date) = Date::parse(
            input,
            &time::format_description::well_known::Iso8601::DATE,
        ) {
            (
                PrimitiveDateTime::new(date, Time::MIDNIGHT),
                UtcOffset::UTC,
            )
        } else {
            return Err(DateTimeError::InvalidFormat);
        };

        Ok(Self { datetime, offset })
    }

    /// Parses a string into a `DateTime` instance using a custom format.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that holds the date/time to parse.
    /// * `format` - The format string to use for parsing.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - The parsed `DateTime` instance or an error if parsing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::parse_custom_format("2023-05-20 15:30:00", "[year]-[month]-[day] [hour]:[minute]:[second]").expect("Valid custom format");
    /// println!("Parsed date: {}", dt);
    /// ```
    pub fn parse_custom_format(
        input: &str,
        format: &str,
    ) -> Result<Self, DateTimeError> {
        let format = time::format_description::parse(format)
            .map_err(|_| DateTimeError::InvalidFormat)?;
        let datetime = PrimitiveDateTime::parse(input, &format)
            .map_err(|_| DateTimeError::InvalidFormat)?;

        Ok(Self {
            datetime,
            offset: UtcOffset::UTC,
        })
    }

    /// Updates the `DateTime` instance to the current date and time.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - The updated `DateTime` instance or an error if the update fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// use std::thread::sleep;
    /// use std::time::Duration;
    ///
    /// let mut dt = DateTime::new();
    /// sleep(Duration::from_secs(1));
    /// dt = dt.update().expect("Update should succeed");
    /// println!("Updated time: {}", dt);
    /// ```
    pub fn update(&self) -> Result<Self, DateTimeError> {
        let now = OffsetDateTime::now_utc().to_offset(self.offset);
        Ok(Self {
            datetime: PrimitiveDateTime::new(now.date(), now.time()),
            offset: self.offset,
        })
    }

    /// Converts the `DateTime` instance to another timezone.
    ///
    /// # Arguments
    ///
    /// * `new_tz` - The timezone abbreviation to convert to.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance in the target timezone or an error if the conversion fails.
    ///
    /// # Note
    ///
    /// This function only supports fixed UTC offsets. If the target timezone observes DST, you will need
    /// to handle the offset change manually.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new_with_tz("EST").expect("Valid timezone");
    /// let converted_dt = dt.convert_to_tz("PST").expect("Conversion to PST should succeed");
    /// println!("Converted time: {}", converted_dt);
    /// ```
    pub fn convert_to_tz(
        &self,
        new_tz: &str,
    ) -> Result<Self, DateTimeError> {
        let new_offset = *TIMEZONE_OFFSETS
            .get(new_tz)
            .ok_or(DateTimeError::InvalidTimezone)?;

        let new_datetime = self
            .datetime
            .assume_offset(self.offset)
            .to_offset(new_offset?);
        Ok(Self {
            datetime: PrimitiveDateTime::new(
                new_datetime.date(),
                new_datetime.time(),
            ),
            offset: new_offset?,
        })
    }

    /// Returns the Unix timestamp of the `DateTime` instance.
    ///
    /// # Returns
    ///
    /// * `i64` - The Unix timestamp in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let timestamp = dt.unix_timestamp();
    /// println!("Unix timestamp: {}", timestamp);
    /// ```
    pub fn unix_timestamp(&self) -> i64 {
        self.datetime.assume_offset(self.offset).unix_timestamp()
    }

    /// Adds a specified number of days to the `DateTime` instance.
    ///
    /// # Arguments
    ///
    /// * `days` - The number of days to add (can be negative for subtraction).
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance with the days added, or an error if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let future_dt = dt.add_days(7).expect("Adding 7 days should succeed");
    /// println!("Date after 7 days: {}", future_dt);
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

    /// Returns a new `DateTime` instance representing the next day.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance for the next day, or an error if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let tomorrow = dt.next_day().expect("Getting next day should succeed");
    /// println!("Tomorrow: {}", tomorrow);
    /// ```
    pub fn next_day(&self) -> Result<Self, DateTimeError> {
        self.add_days(1)
    }

    /// Returns a new `DateTime` instance representing the previous day.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance for the previous day, or an error if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let yesterday = dt.previous_day().expect("Getting previous day should succeed");
    /// println!("Yesterday: {}", yesterday);
    /// ```
    pub fn previous_day(&self) -> Result<Self, DateTimeError> {
        self.add_days(-1)
    }

    /// Formats the `DateTime` instance according to the specified format string.
    ///
    /// # Arguments
    ///
    /// * `format_str` - A string slice that holds the desired format.
    ///
    /// # Returns
    ///
    /// * `Result<String, DateTimeError>` - The formatted date/time string or an error if formatting fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let formatted = dt.format("[year]-[month]-[day]").expect("Format should be valid");
    /// println!("Formatted date: {}", formatted);
    /// ```
    pub fn format(
        &self,
        format_str: &str,
    ) -> Result<String, DateTimeError> {
        let format = time::format_description::parse(format_str)
            .map_err(|_| DateTimeError::InvalidFormat)?;
        self.datetime
            .format(&format)
            .map_err(|_| DateTimeError::InvalidFormat)
    }

    /// Formats the `DateTime` instance as an RFC 3339 string.
    ///
    /// # Returns
    ///
    /// * `Result<String, DateTimeError>` - The formatted RFC 3339 string or an error if formatting fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let formatted = dt.format_rfc3339().expect("RFC 3339 format should succeed");
    /// println!("RFC 3339 date: {}", formatted);
    /// ```
    pub fn format_rfc3339(&self) -> Result<String, DateTimeError> {
        self.datetime
            .assume_offset(self.offset)
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(|_| DateTimeError::InvalidFormat)
    }

    /// Formats the `DateTime` instance as an ISO 8601 string.
    ///
    /// # Returns
    ///
    /// * `Result<String, DateTimeError>` - The formatted ISO 8601 string or an error if formatting fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let formatted = dt.format_iso8601().expect("ISO 8601 format should succeed");
    /// println!("ISO 8601 date: {}", formatted);
    /// ```
    pub fn format_iso8601(&self) -> Result<String, DateTimeError> {
        self.format("[year]-[month]-[day]T[hour]:[minute]:[second]")
    }

    /// Calculates the duration between this `DateTime` and another `DateTime`.
    ///
    /// # Arguments
    ///
    /// * `other` - The `DateTime` to compare with.
    ///
    /// # Returns
    ///
    /// * `Duration` - The duration between the two `DateTime` instances.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt1 = DateTime::new();
    /// let dt2 = dt1.add_days(1).expect("Adding 1 day should succeed");
    /// let duration = dt1.duration_since(&dt2);
    /// println!("Duration: {:?}", duration);
    /// ```
    pub fn duration_since(&self, other: &Self) -> Duration {
        let self_offset_datetime =
            self.datetime.assume_offset(self.offset);
        let other_offset_datetime =
            other.datetime.assume_offset(other.offset);

        let seconds_difference = self_offset_datetime.unix_timestamp()
            - other_offset_datetime.unix_timestamp();
        let nanoseconds_difference = self_offset_datetime.nanosecond()
            as i64
            - other_offset_datetime.nanosecond() as i64;

        Duration::seconds(seconds_difference)
            + Duration::nanoseconds(nanoseconds_difference)
    }

    /// Returns the start of the week for the `DateTime` instance.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance at the start of the week.
    pub fn start_of_week(&self) -> Result<Self, DateTimeError> {
        let days_to_subtract =
            self.weekday().number_days_from_monday() as i64;
        self.add_days(-days_to_subtract)
    }

    /// Returns the end of the week for the `DateTime` instance.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance at the end of the week.
    pub fn end_of_week(&self) -> Result<Self, DateTimeError> {
        let days_to_add =
            6 - self.weekday().number_days_from_monday() as i64;
        self.add_days(days_to_add)
    }

    /// Returns the start of the month for the `DateTime` instance.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance at the start of the month.
    pub fn start_of_month(&self) -> Result<Self, DateTimeError> {
        self.set_date(self.year(), self.month() as u8, 1)
    }

    /// Returns the end of the month for the `DateTime` instance.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance at the end of the month.
    pub fn end_of_month(&self) -> Result<Self, DateTimeError> {
        let year = self.year();
        let month = self.month() as u8;
        let last_day = match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if (year % 4 == 0 && year % 100 != 0)
                    || (year % 400 == 0)
                {
                    29 // Leap year
                } else {
                    28
                }
            }
            _ => return Err(DateTimeError::InvalidDate),
        };

        self.set_date(year, month, last_day)
    }

    /// Returns the start of the year for the `DateTime` instance.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance at the start of the year.
    pub fn start_of_year(&self) -> Result<Self, DateTimeError> {
        self.set_date(self.year(), 1, 1)
    }

    /// Returns the end of the year for the `DateTime` instance.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance at the end of the year.
    pub fn end_of_year(&self) -> Result<Self, DateTimeError> {
        self.set_date(self.year(), 12, 31)
    }

    /// Checks if the `DateTime` instance falls within a specific range.
    ///
    /// # Arguments
    ///
    /// * `start` - The start `DateTime` of the range.
    /// * `end` - The end `DateTime` of the range.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the `DateTime` falls within the range, otherwise `false`.
    pub fn is_within_range(&self, start: &Self, end: &Self) -> bool {
        self >= start && self <= end
    }

    // Getter methods

    /// Returns the year of the `DateTime` instance.
    pub fn year(&self) -> i32 {
        self.datetime.year()
    }

    /// Returns the month of the `DateTime` instance.
    pub fn month(&self) -> Month {
        self.datetime.month()
    }

    /// Returns the day of the month of the `DateTime` instance.
    pub fn day(&self) -> u8 {
        self.datetime.day()
    }

    /// Returns the hour of the `DateTime` instance.
    pub fn hour(&self) -> u8 {
        self.datetime.hour()
    }

    /// Returns the minute of the `DateTime` instance.
    pub fn minute(&self) -> u8 {
        self.datetime.minute()
    }

    /// Returns the second of the `DateTime` instance.
    pub fn second(&self) -> u8 {
        self.datetime.second()
    }

    /// Returns the microsecond of the `DateTime` instance.
    pub fn microsecond(&self) -> u32 {
        self.datetime.microsecond()
    }

    /// Returns the weekday of the `DateTime` instance.
    pub fn weekday(&self) -> Weekday {
        self.datetime.weekday()
    }

    /// Returns the day of the year (ordinal) of the `DateTime` instance.
    pub fn ordinal(&self) -> u16 {
        self.datetime.ordinal()
    }

    /// Returns the ISO week number of the `DateTime` instance.
    pub fn iso_week(&self) -> u8 {
        self.datetime.iso_week()
    }

    /// Returns the UTC offset of the `DateTime` instance.
    pub fn offset(&self) -> UtcOffset {
        self.offset
    }

    /// Returns the local time zone offset of the `DateTime` instance.
    pub fn now(&self) -> Self {
        Self::new()
    }

    /// Sets a new date for the `DateTime` instance.
    ///
    /// # Arguments
    ///
    /// * `year` - The year to set.
    /// * `month` - The month to set (1-12).
    /// * `day` - The day of the month to set.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance with the updated date, or an error if the date is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let new_dt = dt.set_date(2024, 1, 1).expect("Setting date to 2024-01-01 should succeed");
    /// println!("New date: {}", new_dt);
    /// ```
    pub fn set_date(
        &self,
        year: i32,
        month: u8,
        day: u8,
    ) -> Result<Self, DateTimeError> {
        let new_date = Date::from_calendar_date(
            year,
            Month::try_from(month)
                .map_err(|_| DateTimeError::InvalidDate)?,
            day,
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

    /// Sets a new time for the `DateTime` instance.
    ///
    /// # Arguments
    ///
    /// * `hour` - The hour to set (0-23).
    /// * `minute` - The minute to set (0-59).
    /// * `second` - The second to set (0-59).
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance with the updated time, or an error if the time is invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// let new_dt = dt.set_time(12, 0, 0).expect("Setting time to 12:00:00 should succeed");
    /// println!("New time: {}", new_dt);
    /// ```
    pub fn set_time(
        &self,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, DateTimeError> {
        let new_time = Time::from_hms(hour, minute, second)
            .map_err(|_| DateTimeError::InvalidTime)?;
        Ok(Self {
            datetime: PrimitiveDateTime::new(
                self.datetime.date(),
                new_time,
            ),
            offset: self.offset,
        })
    }

    /// Validates whether the provided string represents a valid day of the month.
    ///
    /// # Arguments
    ///
    /// * `day` - A string slice representing the day (expected in "1" to "31").
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the day is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_day("15"));
    /// assert!(!DateTime::is_valid_day("32"));
    /// ```
    pub fn is_valid_day(day: &str) -> bool {
        if let Ok(d) = day.parse::<u8>() {
            (1..=31).contains(&d)
        } else {
            false
        }
    }

    /// Validates whether the provided string represents a valid hour of the day.
    ///
    /// # Arguments
    ///
    /// * `hour` - A string slice representing the hour (expected in "0" to "23").
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the hour is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_hour("23"));
    /// assert!(!DateTime::is_valid_hour("24"));
    /// ```
    pub fn is_valid_hour(hour: &str) -> bool {
        if let Ok(h) = hour.parse::<u8>() {
            h < 24
        } else {
            false
        }
    }

    /// Validates whether the provided string represents a valid minute of the hour.
    ///
    /// # Arguments
    ///
    /// * `minute` - A string slice representing the minute (expected in "0" to "59").
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the minute is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_minute("59"));
    /// assert!(!DateTime::is_valid_minute("60"));
    /// ```
    pub fn is_valid_minute(minute: &str) -> bool {
        if let Ok(m) = minute.parse::<u8>() {
            m < 60
        } else {
            false
        }
    }

    /// Validates whether the provided string represents a valid second of the minute.
    ///
    /// # Arguments
    ///
    /// * `second` - A string slice representing the second (expected in "0" to "59").
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the second is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_second("59"));
    /// assert!(!DateTime::is_valid_second("60"));
    /// ```
    pub fn is_valid_second(second: &str) -> bool {
        if let Ok(s) = second.parse::<u8>() {
            s < 60
        } else {
            false
        }
    }

    /// Validates whether the provided string represents a valid month of the year.
    ///
    /// # Arguments
    ///
    /// * `month` - A string slice representing the month (expected in "1" to "12").
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the month is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_month("12"));
    /// assert!(!DateTime::is_valid_month("13"));
    /// ```
    pub fn is_valid_month(month: &str) -> bool {
        if let Ok(m) = month.parse::<u8>() {
            (1..=12).contains(&m)
        } else {
            false
        }
    }

    /// Validates whether the provided string represents a valid year.
    ///
    /// # Arguments
    ///
    /// * `year` - A string slice representing the year.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the year is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_year("2023"));
    /// assert!(!DateTime::is_valid_year("abcd"));
    /// ```
    pub fn is_valid_year(year: &str) -> bool {
        year.parse::<i32>().is_ok()
    }

    /// Validates whether the provided string represents a valid microsecond.
    ///
    /// # Arguments
    ///
    /// * `microsecond` - A string slice representing the microsecond (expected in "0" to "999999").
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the microsecond is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_microsecond("123456"));
    /// assert!(!DateTime::is_valid_microsecond("1000000"));
    /// ```
    pub fn is_valid_microsecond(microsecond: &str) -> bool {
        if let Ok(ms) = microsecond.parse::<u32>() {
            ms < 1_000_000
        } else {
            false
        }
    }

    /// Validates whether the provided string represents a valid ordinal day of the year.
    ///
    /// # Arguments
    ///
    /// * `ordinal` - A string slice representing the ordinal day (expected in "1" to "366").
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the ordinal is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_ordinal("200"));
    /// assert!(!DateTime::is_valid_ordinal("400"));
    /// ```
    pub fn is_valid_ordinal(ordinal: &str) -> bool {
        if let Ok(o) = ordinal.parse::<u16>() {
            (1..=366).contains(&o)
        } else {
            false
        }
    }

    /// Validates whether the provided string represents a valid ISO week number.
    ///
    /// # Arguments
    ///
    /// * `week` - A string slice representing the ISO week number (expected in "1" to "53").
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the ISO week number is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_iso_week("10"));
    /// assert!(!DateTime::is_valid_iso_week("54"));
    /// ```
    pub fn is_valid_iso_week(week: &str) -> bool {
        if let Ok(w) = week.parse::<u8>() {
            (1..=53).contains(&w)
        } else {
            false
        }
    }

    /// Validates whether the provided string represents a valid time in `HH:MM:SS` format.
    ///
    /// # Arguments
    ///
    /// * `time` - A string slice representing the time in `HH:MM:SS` format.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the time is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_time("12:30:45"));
    /// assert!(!DateTime::is_valid_time("24:00:00"));
    /// ```
    pub fn is_valid_time(time: &str) -> bool {
        let parts: Vec<&str> = time.split(':').collect();
        if parts.len() != 3 {
            return false;
        }
        Self::is_valid_hour(parts[0])
            && Self::is_valid_minute(parts[1])
            && Self::is_valid_second(parts[2])
    }

    /// Validates whether the provided string is a valid ISO 8601 formatted date.
    ///
    /// # Arguments
    ///
    /// * `date` - A string slice representing the ISO 8601 date.
    ///
    /// # Returns
    ///
    /// * `bool` - Returns `true` if the date is valid, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// assert!(DateTime::is_valid_iso_8601("2023-01-01T12:00:00Z"));
    /// assert!(!DateTime::is_valid_iso_8601("2023-13-01T12:00:00Z"));
    /// ```
    pub fn is_valid_iso_8601(date: &str) -> bool {
        DateTime::parse(date).is_ok()
    }
}

impl fmt::Display for DateTime {
    /// Formats the `DateTime` instance as a string using RFC 3339 format.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    ///
    /// let dt = DateTime::new();
    /// println!("{}", dt);
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_rfc3339().map_err(|_| fmt::Error)?)
    }
}

impl FromStr for DateTime {
    type Err = DateTimeError;

    /// Parses a string into a `DateTime` instance.
    ///
    /// This implementation uses the `DateTime::parse` method.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// use std::str::FromStr;
    ///
    /// let dt = DateTime::from_str("2023-05-20T15:30:00Z").expect("Valid RFC 3339 date");
    /// println!("Parsed date: {}", dt);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Default for DateTime {
    /// Returns the current UTC time as the default value for `DateTime`.
    fn default() -> Self {
        Self::new()
    }
}

impl Add<Duration> for DateTime {
    type Output = Result<Self, DateTimeError>;

    /// Adds a `Duration` to the `DateTime` instance.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The `Duration` to add.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance with the duration added, or an error if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// use time::Duration;
    ///
    /// let dt = DateTime::new();
    /// let new_dt = (dt + Duration::hours(2)).expect("Adding 2 hours should succeed");
    /// println!("Time after 2 hours: {}", new_dt);
    /// ```
    fn add(self, rhs: Duration) -> Self::Output {
        let new_datetime = self
            .datetime
            .checked_add(rhs)
            .ok_or(DateTimeError::InvalidDate)?;
        Ok(Self {
            datetime: new_datetime,
            offset: self.offset,
        })
    }
}

impl Sub<Duration> for DateTime {
    type Output = Result<Self, DateTimeError>;

    /// Subtracts a `Duration` from the `DateTime` instance.
    ///
    /// # Arguments
    ///
    /// * `rhs` - The `Duration` to subtract.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DateTimeError>` - A new `DateTime` instance with the duration subtracted, or an error if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::datetime::DateTime;
    /// use time::Duration;
    ///
    /// let dt = DateTime::new();
    /// let new_dt = (dt - Duration::hours(2)).expect("Subtracting 2 hours should succeed");
    /// println!("Time 2 hours ago: {}", new_dt);
    /// ```
    fn sub(self, rhs: Duration) -> Self::Output {
        match self.datetime.checked_sub(rhs) {
            Some(new_datetime) => {
                // Additional manual validation can be placed here if needed.
                Ok(Self {
                    datetime: new_datetime,
                    offset: self.offset,
                })
            }
            None => {
                // Debug output to understand why it failed, if needed.
                eprintln!(
                    "Subtraction resulted in an invalid date/time."
                );
                Err(DateTimeError::InvalidDate)
            }
        }
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.datetime.cmp(&other.datetime)
    }
}

impl Hash for DateTime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.datetime.hash(state);
        self.offset.hash(state);
    }
}
