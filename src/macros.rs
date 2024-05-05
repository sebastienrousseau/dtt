// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # Macros for the DTT Crate
//!
//! This module bundles all macros used across the DTT crate, facilitating date and time operations.
//!
//! ## Overview
//!
//! - **Utility Macros**: General purpose macros such as `dtt_print` for logging and `dtt_vec` for vector operations.
//! - **DateTime Construction**: Macros like `dtt_now` and `dtt_new_with_tz` for creating `DateTime` instances.
//! - **DateTime Operations**: Macros that perform arithmetic and comparisons on `DateTime` objects.
//! - **Validation**: Macros like `is_valid` which validate various aspects of date and time inputs.
//!
//! ## Provided Macros
//!
//! | Macro              | Description                                                          |
//! |--------------------|----------------------------------------------------------------------|
//! | `dtt_now`          | Returns the current `DateTime` instance.                             |
//! | `dtt_parse`        | Parses a string into a `DateTime` instance.                          |
//! | `dtt_new_with_tz`  | Creates a new `DateTime` instance with the specified timezone.       |
//! | `dtt_add_days`     | Adds days to a `DateTime` instance and returns the new instance.     |
//! | `dtt_diff_hours`   | Computes the difference in hours between two `DateTime` instances.   |
//! | `dtt_diff_minutes` | Computes the difference in minutes between two `DateTime` instances. |
//! | `dtt_diff_seconds` | Computes the difference in seconds between two `DateTime` instances. |
//! | `dtt_diff_days`    | Computes the difference in days between two `DateTime` instances.    |
//! | `dtt_print`        | Prints the arguments to the console.                                 |
//! | `dtt_vec`          | Creates a new vector of the given elements.                          |
//! | `dtt_map`          | Creates a new map of the given key-value pairs.                      |
//! | `dtt_assert`       | Checks if the given expression is true and panics if false.          |
//! | `dtt_min`          | Returns the minimum of the given values.                             |
//! | `dtt_max`          | Returns the maximum of the given values.                             |
//! | `dtt_split`        | Splits a string into a vector of words.                              |
//! | `dtt_join`         | Joins a vector of strings into a single string.                      |
//! | `dtt_print_vec`    | Prints a vector of elements to the console.                          |
//! | `is_valid`         | Validates string input based on the specified type and criteria.     |

/// Creates a new `DateTime` instance.
///
/// # Returns
///
/// A new `DateTime` instance.
///
#[macro_export]
macro_rules! dtt_now {
    () => {{
        dtt::DateTime::new()
    }};
}

/// Parses the input string into a `DateTime` instance.
///
/// # Arguments
///
/// - `$input:expr`: The input string to parse.
///
/// # Returns
///
/// A `Result` containing the parsed `DateTime` instance if successful, or an error if parsing fails.
///
/// # Examples
///
/// ```
/// use dtt::DateTime;
/// use dtt::dtt_parse;
///
/// let input = "2022-01-01T12:00:00+01:00";
/// match dtt_parse!(input) {
///     Ok(date) => {
///         assert_eq!(date.year, 2022);
///         assert_eq!(date.month.parse::<u8>().unwrap(), 1);
///         assert_eq!(date.day, 1);
///         assert_eq!(date.hour, 12);
///         assert_eq!(date.minute, 0);
///         assert_eq!(date.second, 0);
///     },
///     Err(err) => panic!("Parsing failed: {}", err),
/// }
/// ```
///
#[macro_export]
macro_rules! dtt_parse {
    ($input:expr) => {{
        dtt::DateTime::parse($input)
    }};
}

/// Prints the arguments to the console.
///
/// # Arguments
///
/// - `($($arg:tt)*)`: The arguments to be printed.
///
/// # Examples
/// ```
/// use dtt::dtt_print;
///
/// dtt_print!("Hello, World!");
/// ```
///
#[macro_export]
macro_rules! dtt_print {
    ($($arg:tt)*) => {
        println!("{}", format_args!("{}", $($arg)*));
    };
}

/// Creates a new vector of the given elements.
///
/// # Parameters
///
/// - `$($elem:expr),*`: The elements to be added to the vector.
/// # Returns
/// A new vector containing the given elements.
/// # Example
/// ```
/// use dtt::dtt_vec;
/// let v = dtt_vec!(1, 2, 3);
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
#[macro_export]
#[doc = "Creates a new vector of the given elements."]
macro_rules! dtt_vec {
    ($($elem:expr),*) => {{
        let mut v = Vec::new();
        $(
            v.push($elem);
        )*
        v
    }};
}

/// Creates a new map of the given key-value pairs.
///
/// # Parameters
///
/// - `$($key:expr => $value:expr),*`: The key-value pairs to be added to the map.
/// # Returns
/// A new map containing the given key-value pairs.
///
/// # Example
/// ```
/// use dtt::dtt_map;
/// let m = dtt_map!(
///     "key1" => "value1",
///     "key2" => "value2",
/// );
/// assert_eq!(m.get("key1"), Some(&"value1"));
/// assert_eq!(m.get("key2"), Some(&"value2"));
/// ```
#[macro_export]
#[doc = "Creates a new map of the given key-value pairs."]
macro_rules! dtt_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        use std::collections::HashMap;
        let mut m = HashMap::new();
        $(
            m.insert($key, $value);
        )*
        m
    }};
}

/// Asserts that the given expression is true.
///
/// If the expression is false, the macro panics with the message "Assertion failed!".
///
/// # Arguments
/// - `($($arg:tt)*)`: The expression to be evaluated.
///
/// # Examples
/// ```
/// use dtt::DateTime;
/// use dtt::dtt_assert;
///
/// // Example date components
/// let year = 2023;
/// let month = "February";
/// let day = 29;
///
/// // Check if the day is valid using the library's function
/// let valid_date = DateTime::is_valid_day(&day.to_string());
///
/// // Use the assert macro to validate the day
/// dtt_assert!(valid_date, "The day must be within the valid range for the given month and year.");
/// ```
#[macro_export]
#[doc = "Asserts that the given expression is true."]
macro_rules! dtt_assert {
    ($cond:expr) => {
        if !$cond {
            panic!("Assertion failed!");
        }
    };
    ($cond:expr, $msg:expr) => {
        if !$cond {
            panic!("{}", $msg);
        }
    };
}

/// This macro returns the minimum of the given values.
///
/// # Parameters
///
/// - `$x:expr`: The first value to be compared.
/// - `$(, $y:expr)*)`: Additional values to be compared.
///
/// # Returns
///
/// The minimum value among the given values.
///
/// # Examples
/// ```
/// use dtt::dtt_min;
///
/// // Example usage
/// let min_value = dtt_min!(5, 3, 10, 1);
/// assert_eq!(min_value, 1);
/// ```
#[macro_export]
#[doc = "Returns the minimum of the given values."]
macro_rules! dtt_min {
    ($x:expr $(, $y:expr)*) => {{
        let mut min = $x;
        $(
            if min > $y { min = $y; }
        )*
        min
    }};
}

/// Returns the latest `DateTime` among the given dates.
///
/// This macro takes in one or more `DateTime` objects and returns the one that represents the most recent date and time.
/// It compares each `DateTime` with the current latest date and updates it if a more recent date is found.
///
/// # Parameters
/// - `$x:expr`: The first `DateTime` object to be compared.
/// - `$(, $y:expr)*)`: Additional `DateTime` objects to be compared.
///
/// # Examples
///
/// ```rust
/// use dtt::{DateTime, dtt_max, DateTimeError};
///
/// let max_int = dtt_max!(10, 20);
/// assert_eq!(max_int, 20);
/// ```
#[macro_export]
#[doc = "Returns the maximum of the given values."]
macro_rules! dtt_max {
    ($x:expr) => {
        $x
    };
    ($x:expr, $y:expr $(, $z:expr)*) => {{
        let mut max = $x;
        if max < $y {
            max = $y;
        }
        $(
            if max < $z {
                max = $z;
            }
        )*
        max
    }};
}

/// This macro takes a vector of strings and joins them together into a
/// single string.
///
/// # Parameters
/// - `$($s:expr),*`: The strings to be joined.
///
/// # Returns
/// A single string containing the concatenated input strings.
///
/// # Examples
/// ```
/// use dtt::dtt_join;
/// let s = dtt_join!("Hello", " ", "World");
/// assert_eq!(s, "Hello World");
/// ```
///
#[macro_export]
#[doc = "Joins a vector of strings into a single string."]
macro_rules! dtt_join {
    ($($s:expr),*) => {{
        let mut s = String::new();
        $(
            s += &$s;
        )*
        s
    }};
}

/// This macro takes a vector of elements and prints them to the
/// console.
///
/// # Parameters
/// - `$($v:expr),*`: The elements to be printed.
///
/// # Examples
///
/// ```
/// use dtt::dtt_print_vec;
/// dtt_print_vec!(&[1, 2, 3]);
/// ```
#[macro_export]
#[doc = "Prints a vector of elements to the console."]
macro_rules! dtt_print_vec {
    ($($v:expr),*) => {{
        for v in $($v),* {
            println!("{}", v);
        }
    }};
}

/// This macro generates a function that takes a string as input and
/// returns a boolean.
/// It takes two arguments:
/// - `$name`: The name of the function to be generated.
/// - `$type`: The type of the input to be validated.
/// The function returns `true` if the input can be parsed into the given type, and `false` otherwise.
/// The macro can be used to validate multiple types of DateTime methods such as:
/// | Macro | Description |
/// |--------|------------|
/// | `is_valid!(day, u32)` | Validates that a `day` is valid. |
/// | `is_valid!(hour, u32)` | Validates that an `hour` is valid. |
/// | `is_valid!(minute, u32)` | Validates that a `minute` is valid. |
/// | `is_valid!(month, u32)` | Validates that a `month` is valid. |
/// | `is_valid!(second, u32)` | Validates that a `second` is valid. |
/// | `is_valid_time!(time, u32)` | Validates that a `time` is valid. |
/// | `is_valid_iso_8601!(iso_8601, u32)` | Validates that a `iso_8601` is valid. |
/// # Examples
/// ```
/// use dtt::is_valid;
/// use dtt::DateTime;
///
/// // Validation of a day
/// let input = "31".to_string();
/// is_valid!(day, String);
/// let result = day(&input);
/// assert!(result);
///
/// // Validation of an hour
/// let input = "23".to_string();
/// is_valid!(hour, String);
/// let result = hour(&input);
/// assert!(result);
///
/// // Validation of a minute
/// let input = "59".to_string();
/// is_valid!(minute, String);
/// let result = minute(&input);
/// assert!(result);
///
/// // Validation of a month
/// let input = "12".to_string();
/// is_valid!(month, String);
/// let result = month(&input);
/// assert!(result);
///
/// // Validation of a second
/// let input = "59".to_string();
/// is_valid!(second, String);
/// let result = second(&input);
/// assert!(result);
///
/// // Validation of a microsecond
/// let input = "999999".to_string();
/// is_valid!(microsecond, String);
/// let result = microsecond(&input);
/// assert!(result);
///
/// // Validation of an ordinal
/// let input = "365".to_string();
/// is_valid!(ordinal, String);
/// let result = ordinal(&input);
/// assert!(result);
///
/// // Validation of a time
/// let input = "23:59:59".to_string();
/// is_valid!(time, String);
/// let result = time(&input);
/// assert!(result);
///
/// // Validation of an iso_8601
/// let input = "2022-01-01T12:00:00+01:00".to_string();
/// is_valid!(iso_8601, String);
/// let result = iso_8601(&input);
/// ```
#[macro_export]
#[doc = "Validates the input based on the specified type and criteria."]
macro_rules! is_valid {
    ($name:ident, $type:ty) => {
        fn $name(input: &str) -> bool {
            match input.parse::<$type>() {
                Ok(parsed_val) => match stringify!($name) {
                    "day" => dtt::DateTime::is_valid_day(
                        &parsed_val.to_string(),
                    ),
                    "hour" => dtt::DateTime::is_valid_hour(
                        &parsed_val.to_string(),
                    ),
                    "minute" => dtt::DateTime::is_valid_minute(
                        &parsed_val.to_string(),
                    ),
                    "month" => dtt::DateTime::is_valid_month(
                        &parsed_val.to_string(),
                    ),
                    "second" => dtt::DateTime::is_valid_second(
                        &parsed_val.to_string(),
                    ),
                    "microsecond" => {
                        dtt::DateTime::is_valid_microsecond(
                            &parsed_val.to_string(),
                        )
                    }
                    "ordinal" => dtt::DateTime::is_valid_ordinal(
                        &parsed_val.to_string(),
                    ),
                    "time" => dtt::DateTime::is_valid_time(
                        &parsed_val.to_string(),
                    ),
                    "iso_8601" => dtt::DateTime::is_valid_iso_8601(
                        &parsed_val.to_string(),
                    ),
                    _ => false,
                },
                Err(_) => false,
            }
        }
    };
}

/// Creates a new `DateTime` instance with the specified timezone.
///
/// # Parameters
///
/// - `$tz:expr`: The timezone string in the format "Z" or "HH:MM".
///
/// # Returns
///
/// A new `DateTime` instance with the specified timezone.
///
#[macro_export]
#[doc = "Creates a new DateTime instance with the specified timezone."]
macro_rules! dtt_new_with_tz {
    ($tz:expr) => {{
        dtt::DateTime::new_with_tz($tz).expect(
            "Failed to create DateTime with the specified timezone",
        )
    }};
}

/// Adds the specified number of days to the given `DateTime` instance.
///
/// # Parameters
///
/// - `$dt:expr`: The `DateTime` instance to which the number of days will be added.
/// - `$days:expr`: The number of days to be added to the given `DateTime` instance.
///
/// # Returns
///
/// A new `DateTime` instance with the specified number of days added.
///
/// # Examples
/// ```
/// use dtt::{DateTime, dtt_add_days};
///
/// let dt = DateTime::parse("2023-01-01T12:00:00+00:00").unwrap();
/// let future_date = dtt_add_days!(dt, 5);
/// println!("Five days later: {}", future_date.iso_8601);
/// ```
///
#[macro_export]
#[doc = "Adds the specified number of days to the given DateTime instance."]
macro_rules! dtt_add_days {
    ($date:expr, $days:expr) => {
        $date.add_days($days)
    };
}

/// Calculates the difference in hours between two DateTime instances.
///
/// # Parameters
///
/// - `$dt1:expr`: The first DateTime instance.
/// - `$dt2:expr`: The second DateTime instance.
///
/// # Returns
///
/// The difference in hours between the two DateTime instances.
///
/// # Examples
/// ```
/// use dtt::dtt_diff_hours;
///
/// let dt1 = "2023-01-01T12:00:00+00:00";
/// let dt2 = "2023-01-02T12:00:00+00:00";
/// let hours_difference = dtt_diff_hours!(dt1, dt2);
///
/// assert!(hours_difference > "23".to_string());
/// ```
///
#[macro_export]
#[doc = "Calculates the difference in hours between two DateTime instances."]
macro_rules! dtt_diff_hours {
    ($dt1:expr, $dt2:expr) => {{
        match ($dt1.parse::<usize>(), $dt2.parse::<usize>()) {
            (Ok(dt1), Ok(dt2)) => {
                let hours_difference =
                    if dt1 <= dt2 { dt2 - dt1 } else { dt1 - dt2 };
                hours_difference.to_string()
            }
            _ => String::from("Error: Invalid input"),
        }
    }};
}

/// Calculates the difference in minutes between two DateTime instances.
///
/// # Parameters
///
/// - `$dt1:expr`: The first DateTime instance.
/// - `$dt2:expr`: The second DateTime instance.
///
/// # Returns
/// The difference in minutes between the two DateTime instances.
///
/// # Examples
/// ```
/// use dtt::dtt_diff_minutes;
///
/// let dt1 = "2023-01-01T12:00:00+00:00";
/// let dt2 = "2023-01-02T12:00:00+00:00";
/// let minutes_difference = dtt_diff_minutes!(dt1, dt2);
///
/// assert!(minutes_difference > "60".to_string());
/// ```
///
#[macro_export]
#[doc = "Calculates the difference in minutes between two DateTime instances."]
macro_rules! dtt_diff_minutes {
    ($dt1:expr, $dt2:expr) => {{
        match ($dt1.parse::<usize>(), $dt2.parse::<usize>()) {
            (Ok(dt1), Ok(dt2)) => {
                let minutes_difference =
                    if dt1 <= dt2 { dt2 - dt1 } else { dt1 - dt2 };
                minutes_difference.to_string()
            }
            _ => String::from("Error: Invalid input"),
        }
    }};
}

/// Calculates the difference in seconds between two DateTime or timestamp instances as Unix timestamps.
///
/// # Parameters
///
/// - `$dt1:expr`: The first DateTime or timestamp instance (as a Unix timestamp).
/// - `$dt2:expr`: The second DateTime or timestamp instance (as a Unix timestamp).
///
/// # Returns
/// The difference in seconds between the two instances as a string.
///
/// # Examples
/// ```
/// use dtt::dtt_diff_seconds;
///
/// let dt1 = "1640995200"; // Unix timestamp for 2023-01-01T12:00:00Z
/// let dt2 = "1640995230"; // Unix timestamp for 2023-01-01T12:00:30Z
/// let seconds_difference = dtt_diff_seconds!(dt1, dt2);
/// assert_eq!(seconds_difference, "30");
/// ```
#[macro_export]
#[doc = "Calculates the difference in seconds between two DateTime instances."]
macro_rules! dtt_diff_seconds {
    ($dt1:expr, $dt2:expr) => {{
        match ($dt1.parse::<i64>(), $dt2.parse::<i64>()) {
            (Ok(dt1), Ok(dt2)) => {
                let seconds_difference =
                    if dt1 <= dt2 { dt2 - dt1 } else { dt1 - dt2 };
                seconds_difference.to_string()
            }
            _ => String::from("Error: Invalid input"),
        }
    }};
}

/// Calculates the difference in whole days between two DateTime or timestamp instances as Unix timestamps.
///
/// # Parameters
///
/// - `$dt1:expr`: The first DateTime or timestamp instance (as a Unix timestamp).
/// - `$dt2:expr`: The second DateTime or timestamp instance (as a Unix timestamp).
///
/// # Returns
/// The difference in whole days between the two instances.
///
/// # Examples
/// ```
/// use dtt::dtt_diff_days;
///
/// let dt1 = "1640995200"; // Unix timestamp for 2023-01-01T00:00:00Z
/// let dt2 = "1641081600"; // Unix timestamp for 2023-01-02T00:00:00Z
/// let days_difference = dtt_diff_days!(dt1, dt2);
/// assert_eq!(days_difference, 1);
/// ```
#[macro_export]
#[doc = "Calculates the difference in days between two DateTime instances."]
macro_rules! dtt_diff_days {
    ($dt1:expr, $dt2:expr) => {{
        match ($dt1.parse::<i64>(), $dt2.parse::<i64>()) {
            (Ok(dt1), Ok(dt2)) => {
                let seconds_difference = if dt1 <= dt2 {
                    dt2 - dt1
                } else {
                    dt1 - dt2
                };
                // Convert the seconds difference to whole days
                (seconds_difference / 86400).abs() // 86400 seconds in a day
            }
            _ => {
                panic!("Error: Invalid input")
            }
        }
    }};
}

/// Creates a deep copy of the provided `DateTime` object.
///
/// # Arguments
///
/// - `$dt:expr`: The `DateTime` object to be cloned.
///
/// # Returns
///
/// A new `DateTime` object that is a deep copy of the input object.
///
/// # Examples
///
/// ```rust
/// use dtt::{DateTime, dtt_clone};
///
/// let dt1 = DateTime::new();
/// let dt2 = dtt_clone!(dt1.clone());
///
/// // dt2 is a separate instance, but has the same values as dt1
/// assert_eq!(dt1, dt2);
/// ```
#[macro_export]
#[doc = "Splits a string into a vector of words."]
macro_rules! dtt_clone {
    ($dt:expr) => {{
        let dt = $dt;
        DateTime {
            day: dt.day,
            hour: dt.hour,
            iso_8601: dt.iso_8601.clone(),
            iso_week: dt.iso_week,
            microsecond: dt.microsecond,
            minute: dt.minute,
            month: dt.month.clone(),
            now: dt.now.clone(),
            offset: dt.offset.clone(),
            ordinal: dt.ordinal,
            second: dt.second,
            time: dt.time.clone(),
            tz: dt.tz.clone(),
            weekday: dt.weekday.clone(),
            year: dt.year,
        }
    }};
}

/// Formats a `DateTime` object using the provided format string.
///
/// # Arguments
///
/// - `$dt:expr`: The `DateTime` object to be formatted.
/// - `$format:expr`: The format string to use for formatting the `DateTime` object. The format string should include the necessary placeholders for the fields of the `DateTime` struct, such as {year}, {month}, {day}, {hour}, {minute}, {second}, {microsecond}, {iso_8601}, {time}, {tz}, {offset}, {weekday}, {ordinal}, {iso_week}.
///
/// # Returns
///
/// A formatted `String` representation of the `DateTime` object.
///
/// # Examples
///
/// ```rust
/// use dtt::{DateTime, dtt_format};
///
/// let dt = DateTime {
///     day: 1,
///     hour: 0,
///     iso_8601: "2023-01-01T00:00:00+00:00".to_string(),
///     iso_week: 1,
///     microsecond: 0,
///     minute: 0,
///     month: "January".to_string(),
///     now: "2023-01-01".to_string(),
///     offset: "+00:00".to_string(),
///     ordinal: 1,
///     second: 0,
///     time: "00:00:00".to_string(),
///     tz: "UTC".to_string(),
///     weekday: "Sunday".to_string(),
///     year: 2023,
/// };
///
/// let formatted = dtt_format!(
///     dt,
///     "{year}-{month}-{day}T{hour}:{minute}:{second}.{microsecond}{offset}, Weekday: {weekday}, ISO Week: {iso_week}"
/// );
/// println!("{}", formatted);
///
///
/// ```
#[macro_export]
#[doc = "Formats a DateTime object using the provided format string."]
macro_rules! dtt_format {
    ($dt:expr, $format:expr) => {{
        format!(
            $format,
            day = $dt.day,
            hour = $dt.hour,
            iso_week = $dt.iso_week,
            microsecond = $dt.microsecond,
            minute = $dt.minute,
            month = $dt.month,
            offset = $dt.offset,
            second = $dt.second,
            weekday = $dt.weekday,
            year = $dt.year,
        )
    }};
}

/// Subtracts the specified number of days from the given `DateTime` instance.
///
/// # Parameters
///
/// - `$date:expr`: The `DateTime` instance from which the number of days will be subtracted.
/// - `$days:expr`: The number of days to be subtracted from the given `DateTime` instance.
///
/// # Returns
///
/// A new `DateTime` instance with the specified number of days subtracted.
///
/// # Examples
/// ```
/// use dtt::{DateTime, dtt_sub_days};
///
/// let dt = DateTime::parse("2023-01-01T12:00:00+00:00").unwrap();
/// let past_date = dtt_sub_days!(dt, 5);
/// println!("Five days earlier: {}", past_date.iso_8601);
/// ```
#[macro_export]
#[doc = "Subtracts the specified number of days from the given DateTime instance."]
macro_rules! dtt_sub_days {
    ($date:expr, $days:expr) => {
        $date.add_days(-$days)
    };
}
