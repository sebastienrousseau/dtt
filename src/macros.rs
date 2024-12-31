// macros.rs
//
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

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

/// Creates a new `DateTime` instance with the current date and time in UTC.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_now;
///
/// let now = dtt_now!();
/// println!("Current date and time: {}", now);
/// ```
#[macro_export]
macro_rules! dtt_now {
    () => {{
        $crate::datetime::DateTime::new()
    }};
}

/// Prints the arguments to the console.
///
/// # Arguments
///
/// - `($($arg:tt)*)`: The arguments to be printed.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_print;
///
/// dtt_print!("Hello, World!");
/// ```
#[macro_export]
macro_rules! dtt_parse {
    ($input:expr) => {{
        $crate::datetime::DateTime::parse($input)
    }};
}

/// Prints the arguments to the console.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_print;
///
/// dtt_print!("Hello, World!");
/// ```
#[macro_export]
macro_rules! dtt_print {
    ($($arg:tt)*) => {
        println!("{}", format_args!("{}", $($arg)*));
    };
}

/// Creates a new vector of the given elements.
///
/// # Arguments
///
/// - `$($elem:expr),*`: The elements to be added to the vector.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_vec;
///
/// let v = dtt_vec!(1, 2, 3);
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
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
/// # Arguments
///
/// - `$($key:expr => $value:expr),*`: The key-value pairs to be added to the map.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_map;
///
/// let m = dtt_map!("one" => 1, "two" => 2);
/// assert_eq!(m.get("one"), Some(&1));
/// assert_eq!(m.get("two"), Some(&2));
/// ```
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
///
/// - `$cond:expr`: The condition to be asserted.
/// - `$msg:expr` (optional): The message to display if the assertion fails.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_parse;
/// use dtt::dtt_assert;
///
/// let is_valid = dtt_parse!("2020-02-29").is_ok();
/// dtt_assert!(is_valid, "The date must be valid.");
/// ```
#[macro_export]
macro_rules! dtt_assert {
    ($cond:expr $(, $msg:expr)?) => {
        if cfg!(debug_assertions) {
            assert!($cond $(, $msg)?);
        }
    };
}

/// Generates a function that validates a given input string based on a specified type.
///
/// # Arguments
///
/// - `$name:ident`: The name of the validation function.
/// - `$type:ty`: The type to validate.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_is_valid_function;
/// use paste::paste;
///
/// dtt_is_valid_function!(day, u8);
/// assert!(is_valid_day("15"));
/// assert!(!is_valid_day("32"));
/// ```
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
/// # Arguments
///
/// - `$x:expr`: The first value to be compared.
/// - `$(, $y:expr)*`: Additional values to be compared.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_min;
///
/// assert_eq!(dtt_min!(10, 20, 30), 10);
/// ```
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
/// # Arguments
///
/// - `$($elem:expr),* $(,)?`: A comma-separated list of expressions. Each expression represents an element to be added to the vector.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_create_vec;
///
/// let v = dtt_create_vec![1, 2, 3];
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
#[macro_export]
macro_rules! dtt_create_vec {
    ($($elem:expr),* $(,)?) => {{
        vec![$($elem),*]
    }};
}

/// Returns the maximum of the given values.
///
/// # Arguments
///
/// - `$x:expr`: The first value to be compared.
/// - `$(, $y:expr)*`: Additional values to be compared.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_max;
///
/// assert_eq!(dtt_max!(10, 20, 30), 30);
/// ```
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

/// Joins multiple strings into a single string.
///
/// # Arguments
///
/// - `$($s:expr),*`: The strings to be joined.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_join;
///
/// let s = dtt_join!("Hello", " ", "World");
/// assert_eq!(s, "Hello World");
/// ```
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
/// # Arguments
///
/// - `$($v:expr),*`: The elements to be printed.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_print_vec;
///
/// dtt_print_vec!(&[1, 2, 3]);
/// ```
#[macro_export]
macro_rules! dtt_print_vec {
    ($($v:expr),*) => {{
        for v in $($v),* {
            println!("{}", v);
        }
    }};
}

/// Generates a function that validates a given input string based on a specified type.
///
/// # Arguments
///
/// - `$name:ident`: The name of the validation function.
/// - `$type:ty`: The type to validate.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_is_valid_function;
/// use paste::paste;
///
/// dtt_is_valid_function!(day, u8);
/// assert!(is_valid_day("15"));
/// assert!(!is_valid_day("32"));
/// ```
#[macro_export]
macro_rules! is_valid {
    ($name:ident, $type:ty) => {
        pub fn $name(input: &str) -> bool {
            $crate::datetime::$name(input)
        }
    };
}

/// Creates a new `DateTime` instance with the specified timezone.
///
/// # Arguments
///
/// - `$tz:expr`: The timezone string.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_new_with_tz;
///
/// let dt = dtt_new_with_tz!("CET");
/// assert_eq!(dt.offset().to_string(), "+01:00:00");
/// ```
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
/// # Arguments
///
/// - `$date:expr`: The `DateTime` instance.
/// - `$days:expr`: The number of days to be added.
///
/// # Example
///
/// ```rust
/// use dtt::{dtt_add_days, dtt_parse};
///
/// let dt = dtt_parse!("2023-01-01T12:00:00+00:00").unwrap();
/// let future_date = dtt_add_days!(dt, 5).unwrap();
/// assert_eq!(future_date.day(), 6);
/// ```
#[macro_export]
macro_rules! dtt_add_days {
    ($date:expr, $days:expr) => {
        $date.add_days($days)
    };
}

/// Subtracts the specified number of days from the given `DateTime` instance.
///
/// # Arguments
///
/// - `$date:expr`: The `DateTime` instance.
/// - `$days:expr`: The number of days to be subtracted.
///
/// # Example
///
/// ```rust
/// use dtt::{dtt_sub_days, dtt_parse};
///
/// let dt = dtt_parse!("2023-01-06T12:00:00+00:00").unwrap();
/// let past_date = dtt_sub_days!(dt, 5).unwrap();
/// assert_eq!(past_date.day(), 1);
/// ```
#[macro_export]
macro_rules! dtt_sub_days {
    ($date:expr, $days:expr) => {
        $date.add_days(-$days)
    };
}

/// A helper macro to calculate the difference between two `DateTime` instances.
///
/// # Parameters
///
/// - `$dt1:expr`: The first `DateTime` instance.
/// - `$dt2:expr`: The second `DateTime` instance.
/// - `$unit:expr`: The unit for the difference (seconds, days, etc.).
///
/// # Returns
///
/// The difference in the specified unit between the two `DateTime` instances.
///
/// # Example
///
/// ```rust
/// use dtt::{dtt_diff, dtt_parse};
///
/// let dt1 = "1609459200"; // 2021-01-01 00:00:00 UTC
/// let dt2 = "1609459230"; // 2021-01-01 00:00:30 UTC
/// let seconds_difference = dtt_diff!(dt1, dt2, 1);
/// assert_eq!(seconds_difference, 30i64);
/// ```
#[macro_export]
macro_rules! dtt_diff {
    ($dt1:expr, $dt2:expr, $unit:expr) => {{
        match ($dt1.parse::<i64>(), $dt2.parse::<i64>()) {
            (Ok(dt1), Ok(dt2)) => {
                let difference =
                    if dt1 <= dt2 { dt2 - dt1 } else { dt1 - dt2 };
                (difference / $unit).abs()
            }
            _ => panic!("Error: Invalid input"),
        }
    }};
}

/// Calculates the difference in seconds between two `DateTime` instances.
///
/// # Arguments
///
/// - `$dt1:expr`: The first `DateTime` instance.
/// - `$dt2:expr`: The second `DateTime` instance.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_diff_seconds;
/// use dtt::dtt_diff;
///
/// let dt1 = "1609459200"; // 2021-01-01 00:00:00 UTC
/// let dt2 = "1609459230"; // 2021-01-01 00:00:30 UTC
/// let seconds_difference = dtt_diff_seconds!(dt1, dt2);
/// assert_eq!(seconds_difference, 30i64);
/// ```
#[macro_export]
macro_rules! dtt_diff_seconds {
    ($dt1:expr, $dt2:expr) => {
        dtt_diff!($dt1, $dt2, 1)
    };
}

/// Calculates the difference in days between two `DateTime` instances.
///
/// # Arguments
///
/// - `$dt1:expr`: The first `DateTime` instance.
/// - `$dt2:expr`: The second `DateTime` instance.
///
/// # Example
///
/// ```rust
/// use dtt::dtt_diff_days;
/// use dtt::dtt_diff;
///
/// let dt1 = "1609459200"; // 2021-01-01 00:00:00 UTC
/// let dt2 = "1609545600"; // 2021-01-02 00:00:00 UTC
/// let days_difference = dtt_diff_days!(dt1, dt2);
/// assert_eq!(days_difference, 1i64);
/// ```
#[macro_export]
macro_rules! dtt_diff_days {
    ($dt1:expr, $dt2:expr) => {
        dtt_diff!($dt1, $dt2, 86400) // 86400 seconds in a day
    };
}

/// Creates a copy of the provided `DateTime` object.
///
/// # Arguments
///
/// - `$dt:expr`: The `DateTime` object to be cloned.
///
/// # Example
///
/// ```rust
/// use dtt::{dtt_clone, dtt_parse};
///
/// let dt = dtt_parse!("2023-01-01T12:00:00+00:00").unwrap();
/// let cloned = dtt_clone!(dt);
/// assert_eq!(dt.year(), cloned.year());
/// ```
#[macro_export]
macro_rules! dtt_clone {
    ($dt:expr) => {{
        let dt = $dt;
        dt.clone()
    }};
}

/// Formats a `DateTime` object using the provided format string.
///
/// # Arguments
///
/// - `$dt:expr`: The `DateTime` object to be formatted.
/// - `$format:expr`: The format string to use for formatting the `DateTime` object.
///
/// # Example
///
/// ```rust
/// use dtt::{dtt_format, dtt_parse};
/// use time::Month;
///
/// let dt = dtt_parse!("2023-01-01T12:00:00+00:00").unwrap();
/// let formatted = dtt_format!(
///     dt,
///     "{year}-{month}-{day}T{hour}:{minute}:{second}.{microsecond}{offset_sign}{offset_hour}:{offset_minute}"
/// );
/// assert_eq!(formatted, "2023-01-01T12:00:00.000000+00:00");
/// ```
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
