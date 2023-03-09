// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for parsing, validating, manipulating, and formatting dates and times
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/dtt/logo/logo-dtt.svg)](https://minifunctions.com/dtt/)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/dtt.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/dtt)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.3-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/dtt)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/dtt)
//! [![License](https://img.shields.io/crates/l/dtt.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! The DateTime (DTT) library is a comprehensive and flexible tool that
//! enables developers to manage dates and times with ease.
//!
//! It provides drop-in replacement methods for parsing, validating,
//! manipulating, and formatting dates and times in Rust.
//!
//! The [**`DateTime`**](./struct.DateTime.html) type to represent a
//! date and a time in a defined timezone.
//!
//! ## Features
//!
//! The library `DateTime` provides date and time types and methods to
//! make it easier to manipulate dates and times. It uses the serde
//! library to derive the Deserialize and Serialize traits to convert
//! the `DateTime` struct to and from various data formats. It also uses
//! the time and regex crates to deal with time conversions and regular
//! expressions respectively.
//!
//! The `DateTime` struct includes fields such as:
//!
//! | Feature | Description |
//! | --- | --- |
//! | `day` | Day of the month: (01-31) |
//! | `hour` | Hour of the day: (00-23) |
//! | `iso_8601` | ISO 8601 date and time: (e.g. "2023-01-01T00:00:00+00:00") |
//! | `iso_week` | ISO week number: (1-53) |
//! | `microsecond` | Microsecond: (0-999999) |
//! | `minute` | Minute of the hour: (0-59) |
//! | `month` | Month: (e.g. "January") |
//! | `now` | Now object: (e.g. "2023-01-01") |
//! | `offset` | Offset from UTC: (e.g. "+00:00") |
//! | `ordinal` | Ordinal date: (1-366) |
//! | `second` | Second of the minute: (0-59) |
//! | `time` | Time object: (e.g. "00:00:00") |
//! | `tz` | Time zone object: (e.g. "UTC") |
//! | `weekday` | Weekday object: (e.g. "Monday") |
//! | `year` | Year object: (e.g. "2023") |
//!
//! Each of which represents different aspects of a date and time.
//!
//! ## Usage
//!
//! - DateTime can be any `serde::Serialize` or `serde::Deserialize`
//! types.
//!
//! ## Examples
//!
//! ```rust
//!
//! // Import the DateTime library
//! extern crate dtt;
//! use self::dtt::DateTime;
//!
//! // Create a new DateTime object.
//!
//! // - The default timezone is UTC.
//! // - The default date is the current date.
//! // - The default time is the current time.
//! // - The default format is ISO 8601.
//! let dt = DateTime::new();
//!
//! // Display the current date.
//! println!("Date: {}", dt.now); // 2023-01-01
//!
//! // Display the current day.
//! println!("Day: {}", dt.day); // 01
//!
//! // Display the current hour.
//! println!("Hour: {}", dt.hour); // 00
//!
//! // Display the current ISO 8601 date and time.
//! println!("ISO 8601: {}", dt.iso_8601); // 2023-01-01T00:00:00+00:00
//!
//! // Display the current ISO week number.
//! println!("ISO Week Number: {}", dt.iso_week); // 1
//!
//! // Display the current microsecond.
//! println!("Microsecond: {}", dt.microsecond); // 000000
//!
//! // Display the current minute.
//! println!("Minute: {}", dt.minute); // 00
//!
//! // Display the current month.
//! println!("Month: {}", dt.month); // January
//!
//! // Display the current offset.
//! println!("Offset: {}", dt.offset); // +00:00
//!
//! // Display the current ordinal date.
//! println!("Ordinal Date: {}", dt.ordinal); // 1
//!
//! // Display the current second.
//! println!("Second: {}", dt.second); // 00
//!
//! // Display the current time.
//! println!("Time: {}", dt.time); // 00:00:00
//!
//! // Display the current timezone.
//! println!("Timezone: {}", dt.tz); // UTC
//!
//! // Display the current weekday.
//! println!("Weekday: {}", dt.weekday); // Monday
//!
//! // Display the current year.
//! println!("Year: {}", dt.year);
//!
//! ```
//! ## License
//!
//! The project is licensed under the terms of both the MIT license and
//! the Apache License (Version 2.0).
//!
//! - [Apache License, Version 2.0](https://opensource.org/licenses/Apache-2.0)
//! - [MIT License](https://opensource.org/licenses/MIT)
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution
//! intentionally submitted for inclusion in the work by you, as
//! defined in the Apache-2.0 license, shall be dual licensed as above,
//! without any additional terms or conditions.
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rustc::existing_doc_keyword)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/dtt/icons/ico-dtt.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/dtt/icons/ico-dtt.svg",
    html_root_url = "https://docs.rs/dtt"
)]
#![crate_name = "dtt"]
#![crate_type = "lib"]

extern crate serde;

use serde::{Deserialize, Serialize};

extern crate time;
use time::{Duration, OffsetDateTime};

extern crate regex;
use regex::Regex;

/// The `macros` module contains functions for generating macros.
pub mod macros;

///
/// DateTime struct to ease dates and times manipulation.
///
/// This module includes date and time types, such as day, hour,ISO 8601
/// date and time, and many more methods.
///
///
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DateTime {
    /// Day of the month: (1-31)
    pub day: u8,
    /// Hour of the day: (0-23)
    pub hour: u8,
    /// ISO 8601 date and time: (e.g. "2023-01-01T00:00:00+00:00")
    pub iso_8601: String,
    /// ISO week number: (1-53)
    pub iso_week: u8,
    /// Microsecond: (0-999999)
    pub microsecond: u32,
    /// Minute of the hour: (0-59)
    pub minute: u8,
    /// Month: (e.g. "January")
    pub month: String,
    /// Now object: (e.g. "2023-01-01")
    pub now: String,
    /// Offset from UTC: (e.g. "+00:00")
    pub offset: String,
    /// Ordinal date: (1-366)
    pub ordinal: u16,
    /// Second of the minute: (0-59)
    pub second: u8,
    /// Time object: (e.g. "00:00:00")
    pub time: String,
    /// Tz object: (e.g. "UTC")
    pub tz: String,
    /// Weekday object: (e.g. "Monday")
    pub weekday: String,
    /// Year object: (e.g. "2023")
    pub year: i32,
}

impl DateTime {
    /// Create a new Date object with UTC timezone.
    pub fn new() -> DateTime {
        Self::new_with_tz("UTC")
    }

    /// Create a new DateTime object with a custom timezone.
    pub fn new_with_tz(tz: &str) -> Self {
        let offset = match tz {
            "UTC" => time::UtcOffset::UTC,
            _ => time::UtcOffset::from_hms(0, 0, 0).unwrap(),
        };
        let now_utc = if tz == "UTC" {
            OffsetDateTime::now_utc()
        } else {
            let (hours, minutes, _) = offset.as_hms();
            let total_seconds =
                (hours as i16 * 3600) + (minutes as i16 * 60);
            OffsetDateTime::now_utc()
                + Duration::seconds(total_seconds as i64)
        };
        let iso_8601 = now_utc.to_string();

        Self {
            day: now_utc.day(),
            hour: now_utc.hour(),
            iso_8601,
            iso_week: now_utc.iso_week(),
            microsecond: now_utc.microsecond(),
            minute: now_utc.minute(),
            month: now_utc.month().to_string(),
            now: now_utc.date().to_string(),
            offset: now_utc.offset().to_string(),
            ordinal: now_utc.ordinal(),
            second: now_utc.second(),
            time: now_utc.time().to_string(),
            tz: tz.to_owned(),
            weekday: now_utc.weekday().to_string(),
            year: now_utc.year(),
        }
    }
    /// Check if the input is a valid day.
    /// 31 is valid.
    /// 32 is not valid.
    pub fn is_valid_day(input: &str) -> bool {
        let mut valid = false;
        if let Ok(day) = input.parse::<u8>() {
            if (1..=31).contains(&day) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid hour.
    /// 23:59 is valid.
    /// 24:00 is not valid.
    pub fn is_valid_hour(input: &str) -> bool {
        let re: Regex =
            Regex::new(r"^([0-1][0-9]|2[0-3])(:[0-5][0-9])?$").unwrap();
        re.is_match(input)
    }
    /// Check if the input is a valid minute.
    /// 59 is valid.
    /// 60 is not valid.
    pub fn is_valid_minute(input: &str) -> bool {
        let mut valid = false;
        if let Ok(minute) = input.parse::<u8>() {
            if (0..=59).contains(&minute) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid month.
    /// 12 is valid.
    /// 13 is not valid.
    pub fn is_valid_month(input: &str) -> bool {
        let mut valid = false;
        if let Ok(month) = input.parse::<u8>() {
            if (1..=12).contains(&month) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid ordinal date.
    /// 366 is valid.
    /// 367 is not valid.
    pub fn is_valid_ordinal(input: &str) -> bool {
        let mut valid = false;
        if let Ok(ordinal) = input.parse::<u16>() {
            if (1..=366).contains(&ordinal) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid second.
    /// 59 is valid.
    /// 60 is not valid.
    pub fn is_valid_second(input: &str) -> bool {
        let mut valid = false;
        if let Ok(second) = input.parse::<u8>() {
            if (0..=59).contains(&second) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid time.
    /// 23:59:59 is valid.
    /// 24:00:00 is not valid.
    pub fn is_valid_time(input: &str) -> bool {
        let re =
            Regex::new(r"^([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$")
                .unwrap();
        re.is_match(input)
    }
    /// Check if the input is a valid ISO 8601 date and time.
    /// 2023-01-01T00:00:00+00:00 is valid.
    /// 2023-01-01T00:00:00+00:00:00 is not valid.
    pub fn is_valid_iso_8601(input: &str) -> bool {
        let re = Regex::new(
            r"^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})(?:\.\d+)?(Z|[+-]\d{2}:\d{2})$",
        )
        .unwrap();
        if !re.is_match(input) {
            return false;
        }
        let captures = re.captures(input).unwrap();
        // let year = captures[1].parse::<i32>().unwrap();
        let month = captures[2].parse::<u32>().unwrap();
        let day = captures[3].parse::<u32>().unwrap();
        let hour = captures[4].parse::<u32>().unwrap();
        let minute = captures[5].parse::<u32>().unwrap();
        let second = captures[6].parse::<u32>().unwrap();
        let tz = captures[7].to_string();
        if !(1..=12).contains(&month)
            || !(1..=31).contains(&day)
            || hour >= 24
            || minute >= 60
            || second >= 60
        {
            return false;
        }
        if tz != "Z" {
            let re = Regex::new(r"^[+-](\d{2}):(\d{2})$").unwrap();
            let captures = re.captures(&tz).unwrap();
            let tz_hour = captures[1].parse::<i32>().unwrap();
            let tz_minute = captures[2].parse::<i32>().unwrap();
            if !(0..=23).contains(&tz_hour)
                || !(0..=59).contains(&tz_minute)
            {
                return false;
            }
        }
        true
    }

    /// Check if the input is a valid ISO week number.
    /// 53 is valid.
    /// 54 is not valid.
    pub fn is_valid_iso_week(input: &str) -> bool {
        let mut valid = false;
        if let Ok(iso_week) = input.parse::<u8>() {
            if (1..=53).contains(&iso_week) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid microsecond.
    /// 999999 is valid.
    /// 1000000 is not valid.
    pub fn is_valid_microsecond(input: &str) -> bool {
        let mut valid = false;
        if let Ok(microsecond) = input.parse::<u32>() {
            if (0..=999999).contains(&microsecond) {
                valid = true;
            }
        }
        valid
    }

    /// Update the date and time.
    pub fn update(&mut self) -> String {
        let now_utc = if self.tz == "UTC" {
            OffsetDateTime::now_utc()
        } else {
            let re = Regex::new(r"([+-])(\d{2}):(\d{2})").unwrap();
            let captures = re.captures(self.tz.as_str());

            let captures = captures.unwrap();
            let hours = captures[2].parse::<i64>().unwrap();
            let minutes = captures[3].parse::<i64>().unwrap();

            let total_seconds =
                (hours as i16 * 3600) + (minutes as i16 * 60);
            OffsetDateTime::now_utc()
                + Duration::seconds(total_seconds as i64)
        };

        self.time = now_utc.to_string();
        self.time.clone()
    }
    /// Calculate the next day.
    /// Returns a new DateTime struct.
    /// The time zone is not updated.
    pub fn next_day(&self) -> DateTime {
        let next_day = OffsetDateTime::now_utc() + Duration::days(1);

        DateTime {
            day: next_day.day(),
            hour: next_day.hour(),
            iso_8601: next_day.to_string(),
            iso_week: next_day.iso_week(),
            microsecond: next_day.microsecond(),
            minute: next_day.minute(),
            month: next_day.month().to_string(),
            now: next_day.date().to_string(),
            offset: next_day.offset().to_string(),
            ordinal: next_day.ordinal(),
            second: next_day.second(),
            time: next_day.time().to_string(),
            tz: next_day.time().to_string(),
            weekday: next_day.weekday().to_string(),
            year: next_day.year(),
        }
    }
    /// Calculates the relative delta based on the current date and time
    /// and the fields of the `RelativeDelta` structure.
    /// Returns the `DateTime` structure that represents the resulting
    /// date and time.
    pub fn relative_delta(&self) -> DateTime {
        let mut new_dt: DateTime = DateTime::default();
        let ordinal = OffsetDateTime::now_utc().ordinal();
        let new_ordinal = ordinal as i64;

        let day = self.day;
        new_dt.day = day;

        let hour = self.hour;
        new_dt.hour = hour;

        let microsecond = self.microsecond;
        new_dt.microsecond = microsecond;

        let minute = self.minute;
        new_dt.minute = minute;

        let month = self.month.to_string();
        new_dt.month = month;

        let second = self.second;
        new_dt.second = second;

        let iso_week = self.iso_week;
        new_dt.iso_week = iso_week;

        let year = self.year;
        new_dt.year = year;

        new_dt.ordinal = new_ordinal as u16;

        DateTime {
            day: new_dt.day,
            hour: new_dt.hour,
            iso_8601: new_dt.iso_8601,
            iso_week: new_dt.iso_week,
            microsecond: new_dt.microsecond,
            minute: new_dt.minute,
            month: new_dt.month,
            now: new_dt.now,
            offset: new_dt.offset,
            ordinal: new_dt.ordinal,
            second: new_dt.second,
            time: new_dt.time,
            tz: new_dt.tz,
            weekday: new_dt.weekday,
            year: new_dt.year,
        }
    }
    /// Calculate the previous day.
    /// Returns the `DateTime` structure that represents the resulting
    /// date and time.
    pub fn previous_day(&self) -> DateTime {
        let previous_day =
            OffsetDateTime::now_utc() - Duration::days(1);
        DateTime {
            day: previous_day.day(),
            hour: previous_day.hour(),
            iso_8601: previous_day.to_string(),
            iso_week: previous_day.iso_week(),
            microsecond: previous_day.microsecond(),
            minute: previous_day.minute(),
            month: previous_day.month().to_string(),
            now: previous_day.date().to_string(),
            offset: previous_day.offset().to_string(),
            ordinal: previous_day.ordinal(),
            second: previous_day.second(),
            time: previous_day.time().to_string(),
            tz: previous_day.time().to_string(),
            weekday: previous_day.weekday().to_string(),
            year: previous_day.year(),
        }
    }
}

impl std::fmt::Display for DateTime {
    /// Display the date and time in ISO 8601 format.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Year: {}\nMonth: {}\nDay: {}\nWeekday: {}\nHour: {}\nMinute: {}\nSecond: {}\nMicrosecond: {}\nOrdinal: {}\nIso 8601: {}\nIso Week: {}\nTime: {}\nTZ: {}\nOffset: {}\nNow: {}", self.year, self.month, self.day, self.weekday, self.hour, self.minute, self.second, self.microsecond, self.ordinal, self.iso_8601, self.iso_week, self.time, self.tz, self.offset, self.now)
    }
}

impl std::str::FromStr for DateTime {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})\+(\d{2}):(\d{2})$")
            .unwrap();
        let captures = match re.captures(s) {
            Some(c) => c,
            None => return Err(()),
        };

        let year = captures[1].parse::<i32>().unwrap();
        let _month = captures[2].parse::<u8>().unwrap();
        let day = captures[3].parse::<u8>().unwrap();
        let hour = captures[4].parse::<u8>().unwrap();
        let minute = captures[5].parse::<u8>().unwrap();
        let second = captures[6].parse::<u8>().unwrap();

        let date = DateTime::default();

        Ok(DateTime {
            day,
            hour,
            iso_8601: s.to_owned(),
            iso_week: date.iso_week,
            microsecond: date.microsecond,
            minute,
            month: date.month.to_string(),
            now: date.now.to_string(),
            offset: date.offset.to_string(),
            ordinal: date.ordinal,
            second,
            time: date.time.to_string(),
            tz: date.time.to_string(),
            weekday: date.weekday,
            year,
        })
    }
}
