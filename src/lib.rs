// lib.rs
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//!
//! # A Rust library for parsing, validating, manipulating, and formatting dates and times
//!
//! [![DTT Banner](https://kura.pro/dtt/images/banners/banner-dtt.svg)](https://dttlib.com)
//!
//! <center>
//!
//! [![Crates.io](https://img.shields.io/crates/v/dtt.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/dtt)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/dtt)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.7-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/dtt)
//! [![License](https://img.shields.io/crates/l/dtt.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//!
//! </center>
//!
//! ## Overview 📖
//!
//! The `DateTime (DTT)` library is a comprehensive and flexible tool that enables developers to manage dates and times with ease. It offers a range of functions and data structures that allow you to perform various date and time operations with ease, such as determining the day of the month, hour of the day, working with ISO 8601 date and time formats, and many others.
//!
//! The library supports the creation of new `DateTime` objects with either UTC or custom timezone specifications, ensuring that you always have accurate and relevant date and time information. Additionally, it provides a mechanism to validate input dates and times, ensuring that you always have accurate information to work with.
//!
//! ## Features ✨
//!
//! The `DateTime (DTT)` struct includes the following fields and methods:
//!
//! ### Fields
//!
//! | Feature | Description | Type |
//! | --- | --- | --- |
//! | `day` | Day of the month: (1-31) | `u8` |
//! | `hour` | Hour of the day: (0-23) | `u8` |
//! | `iso_8601` | ISO 8601 date and time: (e.g. "2023-01-01T00:00:00+00:00") | `String` |
//! | `iso_week` | ISO week number: (1-53) | `u8` |
//! | `microsecond` | Microsecond: (0-999999) | `u32` |
//! | `minute` | Minute of the hour: (0-59) | `u8` |
//! | `month` | Month: (e.g. "January") | `String` |
//! | `now` | Now object: (e.g. "2023-01-01") | `String` |
//! | `offset` | Offset from UTC: (e.g. "+00:00") | `String` |
//! | `ordinal` | Ordinal date: (1-366) | `u16` |
//! | `second` | Second of the minute: (0-59) | `u8` |
//! | `time` | Time object: (e.g. "00:00:00") | `String` |
//! | `tz` | Time zone object: (e.g. "UTC") | `String` |
//! | `weekday` | Weekday object: (e.g. "Monday") | `String` |
//! | `year` | Year object: (e.g. "2023") | `i32` |
//!
//! ### Methods
//!
//! - `new()`: Creates a new `DateTime` object with the current UTC time.
//! - `new_with_tz(tz: &str)`: Creates a new `DateTime` object with the specified timezone.
//! - `is_valid_day(input: &str)`: Checks if the input represents a valid day of the month.
//! - `is_valid_hour(input: &str)`: Checks if the input represents a valid hour of the day.
//! - `is_valid_second(input: &str)`: Checks if the input represents a valid second of the minute.
//! - `is_valid_minute(input: &str)`: Checks if the input represents a valid minute of the hour.
//! - `is_valid_month(input: &str)`: Checks if the input represents a valid month of the year.
//! - `is_valid_ordinal(input: &str)`: Checks if the input represents a valid ordinal date.
//! - `is_valid_time(input: &str)`: Checks if the input represents a valid time.
//! - `is_valid_iso_week(input: &str)`: Checks if the input represents a valid ISO week number.
//! - `is_valid_iso_8601(input: &str)`: Checks if the input represents a valid ISO 8601 date and time.
//! - `is_valid_microsecond(input: &str)`: Checks if the input represents a valid microsecond.
//! - `update(&mut self)`: Updates the `DateTime` object with the current date and time based on the timezone.
//! - `add_days(&self, days: i32)`: Creates a new `DateTime` object with the specified number of days added.
//! - `next_day(&self)`: Creates a new `DateTime` object representing the next day.
//! - `previous_day(&self)`: Creates a new `DateTime` object representing the previous day.
//! - `relative_delta(&self)`: Creates a new `DateTime` object with the relative delta based on the current date and time.
//! - `format(&self, format_str: &str)`: Formats the `DateTime` object as a string using the specified format.
//!
//! The library also provides various getter methods to extract the individual components of the `DateTime` object, such as `year()`, `month()`, `day()`, `hour()`, `minute()`, `second()`, `microsecond()`, `weekday()`, `ordinal()`, `iso_8601()`, `iso_week()`, `time()`, `tz()`, and `offset()`.
//!
//! Additionally, the `DateTime (DTT)` struct implements the `FromStr` trait, allowing for parsing a string into a `DateTime` object.
//!
//! ## License 📝
//!
//! The project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).
//!
//! - [Apache License, Version 2.0](https://opensource.org/license/apache-2-0/)
//! - [MIT license](https://opensource.org/licenses/MIT)
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!

#![doc(
    html_favicon_url = "https://kura.pro/dtt/images/favicon.ico",
    html_logo_url = "https://kura.pro/dtt/images/logos/dtt.svg",
    html_root_url = "https://docs.rs/dtt"
)]
#![crate_name = "dtt"]
#![crate_type = "lib"]

/// The `datetime` module contains the `DateTime` struct and its associated methods.
pub mod datetime;

/// The `error` module contains custom error types for DateTime parsing.
pub mod error;

/// The `macros` module contains functions for generating macros.
pub mod macros;

use std::error::Error;

/// This is the main entry point for the `DateTime (DTT)` library.
pub fn run() -> Result<(), Box<dyn Error>> {
    if std::env::var("DTT_TEST_MODE").unwrap_or_default() == "1" {
        return Err("Simulated error".into());
    }
    let name = "dtt";
    println!("Welcome to `{}` 👋!", { name }.to_uppercase());
    println!(
        "A Rust library for parsing, validating, manipulating, and formatting dates and times."
    );
    Ok(())
}
