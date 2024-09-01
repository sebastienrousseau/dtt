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

/// Creates a new `DateTime` instance.
///
/// # Returns
///
/// A new `DateTime` instance.
///
#[macro_export]
macro_rules! dtt_now {
    () => {{
        $crate::datetime::DateTime::new()
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
#[macro_export]
macro_rules! dtt_parse {
    ($input:expr) => {{
        $crate::datetime::DateTime::parse($input)
    }};
}

/// Prints the arguments to the console.
///
/// # Arguments
///
/// - `($($arg:tt)*)`: The arguments to be printed.
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
///
/// # Returns
///
/// A new vector containing the given elements.
///
#[macro_export]
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
///
/// # Returns
///
/// A new map containing the given key-value pairs.
///
#[macro_export]
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
#[macro_export]
macro_rules! dtt_assert {
    ($cond:expr $(, $msg:expr)?) => {
        if cfg!(debug_assertions) {
            assert!($cond $(, $msg)?);
        }
    };
}

/// This macro generates a function that validates a given input string based on a specified type.
///
/// # Parameters
///
/// - `$name:ident`: The name of the validation function.
/// - `$type:ty`: The type to validate.
///
/// # Returns
///
/// The function returns a boolean value indicating whether the input string is valid for the specified type.
///
#[macro_export]
macro_rules! dtt_is_valid_function {
    ($name:ident, $type:ty) => {
        paste! {
            pub fn [<is_valid_ $name>](input: &str) -> bool {
                if let Ok(parsed_val) = input.parse::<$type>() {
                    $crate::datetime::DateTime::[<is_valid_ $name>](&parsed_val.to_string())
                } else {
                    false
                }
            }
        }
    };
}

/// Returns the minimum of the given values.
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
#[macro_export]
macro_rules! dtt_min {
    ($x:expr $(, $y:expr)*) => {{
        let mut min = $x;
        $(
            let y = $y;
            // This will cause a compile-time error if the types are not comparable
            if std::cmp::PartialOrd::lt(&y, &min) { min = y; }
        )*
        min
    }};
}

/// Creates a new vector containing the provided elements.
///
/// This macro takes a variable number of expressions and creates a new vector containing those expressions.
///
/// # Parameters
///
/// - `$($elem:expr),* $(,)?`: A comma-separated list of expressions. Each expression represents an element to be added to the vector.
///
/// # Return
///
/// - A new vector containing the provided elements.
///
#[macro_export]
macro_rules! dtt_create_vec {
    ($($elem:expr),* $(,)?) => {{
        vec![$($elem),*]
    }};
}


/// Returns the maximum of the given values.
///
/// # Parameters
///
/// - `$x:expr`: The first value to be compared.
/// - `$(, $y:expr)*)`: Additional values to be compared.
///
/// # Returns
///
/// The maximum value among the given values.
///
#[macro_export]
macro_rules! dtt_max {
    ($x:expr $(, $y:expr)*) => {{
        let mut max = $x;
        $(
            if max < $y { max = $y; }
        )*
        max
    }};
}

/// Joins a vector of strings into a single string.
///
/// # Parameters
///
/// - `$($s:expr),*`: The strings to be joined.
///
/// # Returns
///
/// A single string containing the concatenated input strings.
///
#[macro_export]
macro_rules! dtt_join {
    ($($s:expr),*) => {{
        let mut s = String::new();
        $(
            s += &$s;
        )*
        s
    }};
}

/// Prints a vector of elements to the console.
///
/// # Parameters
///
/// - `$($v:expr),*`: The elements to be printed.
///
#[macro_export]
macro_rules! dtt_print_vec {
    ($($v:expr),*) => {{
        for v in $($v),* {
            println!("{}", v);
        }
    }};
}

/// Validates string input based on the specified type and criteria.
///
/// # Parameters
///
/// - `$name:ident`: The name of the validation function.
/// - `$type:ty`: The type to validate.
///
#[macro_export]
macro_rules! is_valid {
    ($name:ident, $type:ty) => {
        fn $name(input: &str) -> bool {
            match input.parse::<$type>() {
                Ok(parsed_val) => match stringify!($name) {
                    "day" => $crate::datetime::DateTime::is_valid_day(
                        &parsed_val.to_string(),
                    ),
                    "hour" => $crate::datetime::DateTime::is_valid_hour(
                        &parsed_val.to_string(),
                    ),
                    "minute" => {
                        $crate::datetime::DateTime::is_valid_minute(
                            &parsed_val.to_string(),
                        )
                    }
                    "month" => $crate::datetime::DateTime::is_valid_month(
                        &parsed_val.to_string(),
                    ),
                    "second" => {
                        $crate::datetime::DateTime::is_valid_second(
                            &parsed_val.to_string(),
                        )
                    }
                    "microsecond" => {
                        $crate::datetime::DateTime::is_valid_microsecond(
                            &parsed_val.to_string(),
                        )
                    }
                    "ordinal" => {
                        $crate::datetime::DateTime::is_valid_ordinal(
                            &parsed_val.to_string(),
                        )
                    }
                    "time" => $crate::datetime::DateTime::is_valid_time(
                        &parsed_val.to_string(),
                    ),
                    "iso_8601" => {
                        $crate::datetime::DateTime::is_valid_iso_8601(
                            &parsed_val.to_string(),
                        )
                    }
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
/// - `$tz:expr`: The timezone string.
///
/// # Returns
///
/// A new `DateTime` instance with the specified timezone.
///
#[macro_export]
macro_rules! dtt_new_with_tz {
    ($tz:expr) => {{
        $crate::datetime::DateTime::new_with_tz($tz).expect(
            "Failed to create DateTime with the specified timezone",
        )
    }};
}

/// Adds the specified number of days to the given `DateTime` instance.
///
/// # Parameters
///
/// - `$date:expr`: The `DateTime` instance.
/// - `$days:expr`: The number of days to be added.
///
/// # Returns
///
/// A new `DateTime` instance with the specified number of days added.
///
/// ```
#[macro_export]
macro_rules! dtt_add_days {
    ($date:expr, $days:expr) => {
        $date.add_days($days)
    };
}

/// Subtracts the specified number of days from the given `DateTime` instance.
///
/// # Parameters
///
/// - `$date:expr`: The `DateTime` instance.
/// - `$days:expr`: The number of days to be subtracted.
///
/// # Returns
///
/// A new `DateTime` instance with the specified number of days subtracted.
///
#[macro_export]
macro_rules! dtt_sub_days {
    ($date:expr, $days:expr) => {
        $date.add_days(-$days)
    };
}

/// Calculates the difference in seconds between two `DateTime` instances.
///
/// # Parameters
///
/// - `$dt1:expr`: The first `DateTime` instance.
/// - `$dt2:expr`: The second `DateTime` instance.
///
/// # Returns
///
/// The difference in seconds between the two `DateTime` instances.
///
#[macro_export]
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

/// Calculates the difference in days between two `DateTime` instances.
///
/// # Parameters
///
/// - `$dt1:expr`: The first `DateTime` instance.
/// - `$dt2:expr`: The second `DateTime` instance.
///
/// # Returns
///
/// The difference in days between the two `DateTime` instances.
///
#[macro_export]
macro_rules! dtt_diff_days {
    ($dt1:expr, $dt2:expr) => {{
        match ($dt1.parse::<i64>(), $dt2.parse::<i64>()) {
            (Ok(dt1), Ok(dt2)) => {
                let seconds_difference = if dt1 <= dt2 {
                    dt2 - dt1
                } else {
                    dt1 - dt2
                };
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
#[macro_export]
macro_rules! dtt_clone {
    ($dt:expr) => {{
        let dt = $dt;
        DateTime {
            day: dt.day(),
            hour: dt.hour(),
            iso_8601: dt.iso_8601.clone(),
            iso_week: dt.iso_week(),
            microsecond: dt.microsecond(),
            minute: dt.minute(),
            month: dt.month().clone(),
            now: dt.now().clone(),
            offset: dt.offset().clone(),
            ordinal: dt.ordinal(),
            second: dt.second(),
            time: dt.time().clone(),
            tz: dt.tz().clone(),
            weekday: dt.weekday().clone(),
            year: dt.year(),
        }
    }};
}

/// Formats a `DateTime` object using the provided format string.
///
/// # Arguments
///
/// - `$dt:expr`: The `DateTime` object to be formatted.
/// - `$format:expr`: The format string to use for formatting the `DateTime` object.
///
/// # Returns
///
/// A formatted `String` representation of the `DateTime` object.
///
#[macro_export]
macro_rules! dtt_format {
    ($dt:expr, $format:expr) => {{
        // Convert the month to a numeric value manually
        let month_number = match $dt.month() {
            Month::January => 1,
            Month::February => 2,
            Month::March => 3,
            Month::April => 4,
            Month::May => 5,
            Month::June => 6,
            Month::July => 7,
            Month::August => 8,
            Month::September => 9,
            Month::October => 10,
            Month::November => 11,
            Month::December => 12,
        };

        format!(
            $format,
            day = format!("{:02}", $dt.day()),
            hour = format!("{:02}", $dt.hour()),
            microsecond = format!("{:06}", $dt.microsecond()),
            minute = format!("{:02}", $dt.minute()),
            month = format!("{:02}", month_number), // Use the numeric month
            offset_sign = if $dt.offset().whole_seconds() >= 0 { "+" } else { "-" },
            offset_hour = format!("{:02}", $dt.offset().whole_hours().abs()),
            offset_minute = format!("{:02}", ($dt.offset().whole_seconds().abs() % 3600) / 60),
            second = format!("{:02}", $dt.second()),
            year = $dt.year(),
        )
    }};
}
