// lib.rs
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//!
//! # A Rust library for parsing, validating, manipulating, and formatting dates and times.
//!
//! [![DTT Banner](https://kura.pro/dtt/images/banners/banner-dtt.svg)](https://dttlib.com)
//!
//! [![Made With Love][made-with-rust]][14] [![Crates.io][crates-badge]][08] [![lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![Codecov][codecov-badge]][15] [![Build Status][build-badge]][16]
//!
//! <!-- markdownlint-disable MD033 MD041 -->
//! <center>
//! <!-- markdownlint-enable MD033 MD041 -->
//!
//! • [Website][01] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]
//!
//! <!-- markdownlint-disable MD033 MD041 -->
//! </center>
//! <!-- markdownlint-enable MD033 MD041 -->
//!
//! ## Overview
//!
//! The `DateTime (DTT)` library is a comprehensive and flexible tool that enables developers to manage dates and times with ease. It offers a wide range of functions, macros, and data structures for performing date and time operations, such as creating, parsing, validating, and formatting date-time objects.
//!
//! The library supports the creation of new `DateTime` objects with either UTC or custom timezone specifications, providing accurate and relevant date and time information. Additionally, it offers mechanisms for validating input dates and times, ensuring reliable and precise operations.
//!
//! ## Features
//!
//! The `DateTime (DTT)` library offers the following features:
//!
//! - **Core Fields**:
//!   - `datetime`: The date and time in UTC (`PrimitiveDateTime`).
//!   - `offset`: The timezone offset in UTC (`UtcOffset`).
//!
//! - **Core Methods**:
//!   - `new()`: Creates a new `DateTime` instance with the current UTC time.
//!   - `new_with_tz(tz: &str)`: Creates a new `DateTime` object with the specified timezone.
//!   - `new_with_custom_offset(hours: i8, minutes: i8)`: Creates a `DateTime` object with a custom UTC offset.
//!   - `from_components(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8, offset: UtcOffset)`: Creates a `DateTime` object from individual date and time components.
//!
//! - **Parsing and Formatting**:
//!   - `parse(input: &str)`: Parses a date-time string into a `DateTime` object.
//!   - `parse_custom_format(input: &str, format: &str)`: Parses a date-time string using a custom format.
//!   - `format(&self, format_str: &str)`: Formats the `DateTime` object as a string using the specified format.
//!
//! - **Validation**:
//!   - `is_valid_day(input: &str)`: Checks if the input represents a valid day of the month.
//!   - `is_valid_hour(input: &str)`: Checks if the input represents a valid hour of the day.
//!   - `is_valid_second(input: &str)`: Checks if the input represents a valid second of the minute.
//!   - `is_valid_minute(input: &str)`: Checks if the input represents a valid minute of the hour.
//!   - `is_valid_month(input: &str)`: Checks if the input represents a valid month of the year.
//!   - `is_valid_ordinal(input: &str)`: Checks if the input represents a valid ordinal date.
//!   - `is_valid_time(input: &str)`: Checks if the input represents a valid time.
//!   - `is_valid_iso_week(input: &str)`: Checks if the input represents a valid ISO week number.
//!   - `is_valid_iso_8601(input: &str)`: Checks if the input represents a valid ISO 8601 date and time.
//!   - `is_valid_microsecond(input: &str)`: Checks if the input represents a valid microsecond.
//!
//! - **Date-Time Manipulation**:
//!   - `update(&self)`: Updates the `DateTime` object with the current date and time based on the timezone.
//!   - `convert_to_tz(&self, new_tz: &str)`: Converts the `DateTime` object to a different timezone.
//!   - `unix_timestamp(&self)`: Returns the Unix timestamp of the `DateTime` object.
//!   - `add_days(&self, days: i64)`: Creates a new `DateTime` object with the specified number of days added.
//!
//! - **Utility Macros**:
//!   - `dtt_now!()`: Generates the current date and time.
//!   - `dtt_parse!(input)`: Parses a date-time string into a `DateTime` object.
//!   - `dtt_print!(datetime)`: Prints a `DateTime` object.
//!   - `dtt_vec![]`: Creates a vector.
//!   - `dtt_map!{}`: Creates a map.
//!   - `dtt_assert!`: Asserts conditions during testing.
//!
//! - **Validation Macros**:
//!   - `is_valid!`: Checks the validity of various date-time components.
//!   - `dtt_is_valid_function!(func_name)`: Defines a custom validation function.
//!
//! - **Date-Time Manipulation Macros**:
//!   - `dtt_new_with_tz!(tz)`: Creates a new `DateTime` object with a specified timezone.
//!   - `dtt_add_days!(datetime, days)`: Adds days to a `DateTime` object.
//!   - `dtt_sub_days!(datetime, days)`: Subtracts days from a `DateTime` object.
//!   - `dtt_diff_seconds!(datetime1, datetime2)`: Calculates the difference in seconds between two `DateTime` objects.
//!   - `dtt_diff_days!(datetime1, datetime2)`: Calculates the difference in days between two `DateTime` objects.
//!
//! ## Documentation
//!
//! For full API documentation, please visit [https://doc.dttlib.com/][17] or [https://docs.rs/dtt][09].
//!
//! ## Rust Version Compatibility
//!
//! Compiler support: requires rustc 1.56.0+
//!
//! ## License
//!
//! Licensed under either of the [Apache License][02] or the
//! [MIT license][03] at your option.
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
//!
//! [01]: https://dttlib.com "DateTime (DTT) Library Website"
//! [02]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
//! [03]: https://opensource.org/licenses/MIT "MIT license"
//! [04]: https://github.com/sebastienrousseau/dtt/issues "Issues"
//! [05]: https://github.com/sebastienrousseau/dtt/blob/main/CONTRIBUTING.md "Contributing Instructions"
//! [08]: https://crates.io/crates/dtt "Crates.io"
//! [09]: https://docs.rs/dtt "Docs.rs"
//! [10]: https://lib.rs/crates/dtt "Lib.rs"
//! [14]: https://www.rust-lang.org "The Rust Programming Language"
//! [15]: https://codecov.io/gh/sebastienrousseau/dtt "Codecov"
//! [16]: https://github.com/sebastienrousseau/dtt/actions?query=branch%3Amain "Build Status"
//!
//! [build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/dtt/release.yml?branch=main&style=for-the-badge&logo=github "Build Status"
//! [codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&token=X3ZP0K1SGI 'Codecov'
//! [crates-badge]: https://img.shields.io/crates/v/dtt.svg?style=for-the-badge 'Crates.io badge'
//! [docs-badge]: https://img.shields.io/docsrs/dtt.svg?style=for-the-badge 'Docs.rs badge'
//! [libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.7-orange.svg?style=for-the-badge 'Lib.rs badge'
//! [made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'

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

use error::AppError;
use std::env;

/// Runs the main logic of the application.
///
/// # Errors
///
/// This function will return an error in the following situations:
/// - If the `DTT_TEST_MODE` environment variable is set to `1`, an `AppError::SimulatedError` is returned.
/// - If there is an issue retrieving an environment variable, an `AppError::EnvVarError` is returned.
///
/// # Examples
///
/// ```rust
/// use dtt::run;
/// use dtt::error::AppError;
/// fn main() -> Result<(), AppError> {
///     if std::env::var("DTT_TEST_MODE").unwrap_or_default() == "1" {
///         // This will return a SimulatedError.
///         return Err(AppError::SimulatedError);
///     }
///
///     // Your main application logic here
///
///     Ok(())
/// }
/// ```
pub fn run() -> Result<(), AppError> {
    // Check if the test mode is enabled via the environment variable
    if env::var("DTT_TEST_MODE").unwrap_or_default() == "1" {
        return Err(AppError::SimulatedError);
    }
    let name = "dtt";
    println!("Welcome to `{}` 👋!", { name }.to_uppercase());
    println!(
        "A Rust library for parsing, validating, manipulating, and formatting dates and times."
    );
    Ok(())
}
