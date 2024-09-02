<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/dtt/images/logos/dtt.svg"
alt="DateTime (DTT) logo" height="66" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# DateTime (DTT)

A Rust library for parsing, validating, manipulating, and formatting dates and times.

[![Made With Love][made-with-rust]][14] [![Crates.io][crates-badge]][08] [![lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![Codecov][codecov-badge]][15] [![Build Status][build-badge]][16] [![Codecov][github-badge]][07]

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

• [Website][01] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Overview

The `DateTime (DTT)` library is a comprehensive and flexible tool that enables developers to manage dates and times with ease. It offers a wide range of functions, macros, and data structures for performing date and time operations, such as creating, parsing, validating, and formatting date-time objects.

The library supports the creation of new `DateTime` objects with either UTC or custom timezone specifications, providing accurate and relevant date and time information. Additionally, it offers mechanisms for validating input dates and times, ensuring reliable and precise operations.

## Features

The `DateTime (DTT)` library offers the following features:

- **Core Fields**:
  - `datetime`: The date and time in UTC (`PrimitiveDateTime`).
  - `offset`: The timezone offset in UTC (`UtcOffset`).

- **Core Methods**:
  - `new()`: Creates a new `DateTime` instance with the current UTC time.
  - `new_with_tz(tz: &str)`: Creates a new `DateTime` object with the specified timezone.
  - `new_with_custom_offset(hours: i8, minutes: i8)`: Creates a `DateTime` object with a custom UTC offset.
  - `from_components(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8, offset: UtcOffset)`: Creates a `DateTime` object from individual date and time components.
  - `update(&self)`: Updates the `DateTime` object to the current date and time.
  - `now()`: Returns the current `DateTime`.
  - `from_str(s: &str)`: Parses a string into a `DateTime` instance (implementation of `FromStr` trait).
  - `default()`: Returns the current UTC time as the default value for `DateTime`.

- **Parsing and Formatting**:
  - `parse(input: &str)`: Parses a date-time string into a `DateTime` object.
  - `parse_custom_format(input: &str, format: &str)`: Parses a date-time string using a custom format.
  - `format(&self, format_str: &str)`: Formats the `DateTime` object as a string using the specified format.
  - `format_rfc3339(&self)`: Formats the `DateTime` as an RFC 3339 string.
  - `format_iso8601(&self)`: Formats the `DateTime` as an ISO 8601 string.

- **Date-Time Manipulation**:
  - `convert_to_tz(&self, new_tz: &str)`: Converts the `DateTime` object to a different timezone.
  - `unix_timestamp(&self)`: Returns the Unix timestamp of the `DateTime` object.
  - `add_days(&self, days: i64)`: Adds a specified number of days to the `DateTime`.
  - `add_months(&self, months: i32)`: Adds a specified number of months to the `DateTime`.
  - `add_years(&self, years: i32)`: Adds a specified number of years to the `DateTime`.
  - `sub_months(&self, months: i32)`: Subtracts a specified number of months from the `DateTime`.
  - `sub_years(&self, years: i32)`: Subtracts a specified number of years from the `DateTime`.
  - `next_day(&self)`: Returns a new `DateTime` instance representing the next day.
  - `previous_day(&self)`: Returns a new `DateTime` instance representing the previous day.
  - `start_of_week(&self)`: Returns a new `DateTime` instance at the start of the week.
  - `end_of_week(&self)`: Returns a new `DateTime` instance at the end of the week.
  - `start_of_month(&self)`: Returns a new `DateTime` instance at the start of the month.
  - `end_of_month(&self)`: Returns a new `DateTime` instance at the end of the month.
  - `start_of_year(&self)`: Returns a new `DateTime` instance at the start of the year.
  - `end_of_year(&self)`: Returns a new `DateTime` instance at the end of the year.
  - `is_within_range(&self, start: &Self, end: &Self)`: Checks if the `DateTime` falls within a specific range.
  - `duration_since(&self, other: &Self)`: Calculates the duration between two `DateTime` instances.

- **Getters**:
  - `year(&self)`: Returns the year.
  - `month(&self)`: Returns the month.
  - `day(&self)`: Returns the day of the month.
  - `hour(&self)`: Returns the hour.
  - `minute(&self)`: Returns the minute.
  - `second(&self)`: Returns the second.
  - `microsecond(&self)`: Returns the microsecond.
  - `weekday(&self)`: Returns the weekday.
  - `ordinal(&self)`: Returns the day of the year (ordinal).
  - `iso_week(&self)`: Returns the ISO week number.
  - `offset(&self)`: Returns the UTC offset.

- **Setters**:
  - `set_date(&self, year: i32, month: u8, day: u8)`: Sets a new date for the `DateTime` instance.
  - `set_time(&self, hour: u8, minute: u8, second: u8)`: Sets a new time for the `DateTime` instance.

- **Validation**:
  - `is_valid_day(day: &str)`: Checks if the input represents a valid day of the month.
  - `is_valid_hour(hour: &str)`: Checks if the input represents a valid hour of the day.
  - `is_valid_minute(minute: &str)`: Checks if the input represents a valid minute of the hour.
  - `is_valid_second(second: &str)`: Checks if the input represents a valid second of the minute.
  - `is_valid_month(month: &str)`: Checks if the input represents a valid month of the year.
  - `is_valid_year(year: &str)`: Checks if the input represents a valid year.
  - `is_valid_microsecond(microsecond: &str)`: Checks if the input represents a valid microsecond.
  - `is_valid_ordinal(ordinal: &str)`: Checks if the input represents a valid ordinal day of the year.
  - `is_valid_iso_week(week: &str)`: Checks if the input represents a valid ISO week number.
  - `is_valid_time(time: &str)`: Checks if the input represents a valid time in `HH:MM:SS` format.
  - `is_valid_iso_8601(date: &str)`: Checks if the input represents a valid ISO 8601 formatted date.

- **Utility Functions**:
  - `format_time_in_timezone(tz: &str, format: &str)`: Formats the current time for a specific timezone.

- **Arithmetic Operations**:
  - `Add<Duration>`: Adds a `Duration` to the `DateTime` instance.
  - `Sub<Duration>`: Subtracts a `Duration` from the `DateTime` instance.

- **Comparison Operations**:
  - `PartialOrd`: Allows partial ordering comparisons between `DateTime` instances.
  - `Ord`: Allows total ordering comparisons between `DateTime` instances.

- **Hashing**:
  - `Hash`: Allows `DateTime` instances to be used as keys in hash-based collections.

- **Macros**:
  - `dtt_now!()`: Generates the current date and time.
  - `dtt_parse!(input)`: Parses a date-time string into a `DateTime` object.
  - `dtt_print!(datetime)`: Prints a `DateTime` object.
  - `dtt_vec![]`: Creates a vector.
  - `dtt_map!{}`: Creates a map.
  - `dtt_assert!`: Asserts conditions during testing.
  - `is_valid!`: Checks the validity of various date-time components.
  - `dtt_is_valid_function!(func_name)`: Defines a custom validation function.
  - `dtt_new_with_tz!(tz)`: Creates a new `DateTime` object with a specified timezone.
  - `dtt_add_days!(datetime, days)`: Adds days to a `DateTime` object.
  - `dtt_sub_days!(datetime, days)`: Subtracts days from a `DateTime` object.
  - `dtt_diff_seconds!(datetime1, datetime2)`: Calculates the difference in seconds between two `DateTime` objects.
  - `dtt_diff_days!(datetime1, datetime2)`: Calculates the difference in days between two `DateTime` objects.
  - `dtt_clone!`: Creates a deep copy of a `DateTime` object.
  - `dtt_format!`: Formats a `DateTime` object using a provided format string.
  - `dtt_create_vec!`: Creates a new vector containing provided elements.
  - `dtt_min!`: Returns the minimum of given values.
  - `dtt_max!`: Returns the maximum of given values.
  - `dtt_join!`: Joins a vector of strings into a single string.
  - `dtt_print_vec!`: Prints a vector of elements to the console.

- **Helper Functions**:
  - `days_in_month(year: i32, month: u8)`: Determines the number of days in a given month and year.
  - `is_leap_year(year: i32)`: Determines if a year is a leap year.

- **Error Handling**:
  The library provides comprehensive error handling through the `DateTimeError` enum, allowing for robust error management in date and time operations.

- **Timezone Support**:
  DateTime (DTT) offers extensive timezone support, allowing for creation and manipulation of date-time objects across different timezones.

- **Serialization and Deserialization**:
  The library supports serialization and deserialization of `DateTime` objects using `serde`, facilitating easy integration with various data formats.

[01]: https://dttlib.com "DateTime (DTT) Library Website"
[04]: https://github.com/sebastienrousseau/dtt/issues "Issues"
[05]: https://github.com/sebastienrousseau/dtt/blob/main/CONTRIBUTING.md "Contributing Instructions"
[07]: https://github.com/sebastienrousseau/dtt "DateTime (DTT) Library"
[08]: https://crates.io/crates/dtt "Crates.io"
[09]: https://docs.rs/dtt "Docs.rs"
[10]: https://lib.rs/crates/dtt "Lib.rs"
[14]: https://www.rust-lang.org "The Rust Programming Language"
[15]: https://codecov.io/gh/sebastienrousseau/dtt "Codecov"
[16]: https://github.com/sebastienrousseau/dtt/actions?query=branch%3Amain "Build Status"

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/dtt/release.yml?branch=main&style=for-the-badge&logo=github "Build Status"
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/main-8da0cb?style=for-the-badge&labelColor=555555&logo=github "GitHub"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&token=X3ZP0K1SGI 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/dtt.svg?style=for-the-badge 'Crates.io badge'
[docs-badge]: https://img.shields.io/docsrs/dtt.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.7-orange.svg?style=for-the-badge 'Lib.rs badge'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'

## Changelog 📚
