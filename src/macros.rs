// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! Macros for the dtt crate.
//!
//! This module bundles all macros used across the dtt crate.
//! These include macros for validating input, and macros for
//! generating the DateTime struct.
//!
//! ## Generic macros for the dtt crate.
//!
//! This crate provides the following macros:
//!
//! | Macro | Description |
//! |--------|------------|
//! | `dtt` | The main macro for the dtt crate. It takes any number of arguments and parses them into a Rust value.
//! | `dtt_print` | Prints the arguments to the console.
//! | `dtt_vec` | Creates a new vector of the given elements.
//! | `dtt_map` | Creates a new map of the given key-value pairs.
//! | `dtt_assert` | Checks if the given expression is true.
//! | `dtt_min` | Returns the minimum of the given values.
//! | `dtt_max` | Returns the maximum of the given values.
//! | `dtt_split` | Splits a string into a vector of words.
//! | `dtt_join` | Joins a vector of strings into a single string.
//! | `dtt_print_vec` | Prints a vector of elements to the console.//!
//!
//! # Examples
//!
//! ```
//! use dtt::DateTime;
//! use dtt::{dtt, dtt_print, dtt_vec, dtt_map, dtt_assert, dtt_min, dtt_max, dtt_split, dtt_join, dtt_print_vec};
//!
//! // dtt macro - parsing a string into a DateTime struct
//! let input = "2023-01-01T00:00:00Z";
//! let result = dtt!(input);
//!
//! // dtt_print macro - printing a string to the console
//! let message = "Hello, world!";
//! dtt_print!(message);
//!
//! // dtt_vec macro - creating a new vector of the given elements
//! let v = dtt_vec![1, 2, 3, 4, 5];
//! assert_eq!(v, vec![1, 2, 3, 4, 5]);
//!
//! // dtt_map macro - creating a new map of the given key-value pairs
//! let m = dtt_map!{"one" => 1, "two" => 2};
//! assert_eq!(m["one"], 1);
//! assert_eq!(m["two"], 2);
//!
//! // dtt_assert macro - checking if the given expression is true
//! dtt_assert!(1 + 1 == 2); // does not panic
//!
//! // dtt_min macro - returning the minimum of the given values
//! let min = dtt_min!(5, 2, 9, 4, 7);
//! assert_eq!(min, 2);
//!
//! // dtt_max macro - returning the maximum of the given values
//! let max = dtt_max!(5, 2, 9, 4, 7);
//! assert_eq!(max, 9);
//!
//! // dtt_split macro - splitting a string into a vector of words
//! let words = dtt_split!("Hello, world!");
//! assert_eq!(words, vec!["Hello,", "world!"]);
//!
//! // dtt_join macro - joining a vector of strings together into a single string
//! let sentence = dtt_join!("Hello", ", ", "world", "!");
//! assert_eq!(sentence, "Hello, world!");
//!
//! // dtt_print_vec macro - printing a vector of elements to the console
//! let v = vec![1, 2, 3, 4, 5];
//! dtt_print_vec!(v); // prints "1", "2", "3", "4", "5" each on their own line
//! ```
//!
//! ## Macro for validating a string input against a regex.
//!
//! This macro generates a function that takes a string as input and returns a boolean.
//!
//! It takes two arguments:
//!
//! - `$name`: The name of the function to be generated.
//! - `$type`: The type of the input to be validated.
//!
//! The function returns `true` if the input can be parsed into the given type, and `false` otherwise.
//!
//! The macro can be used to validate multiple types of DateTime methods such as:
//!
//! | Macro | Description |
//! |--------|------------|
//! | `is_valid!(day, u32)` | Validates that a `day` is valid. |
//! | `is_valid!(hour, u32)` | Validates that an `hour` is valid. |
//! | `is_valid!(minute, u32)` | Validates that a `minute` is valid. |
//! | `is_valid!(month, u32)` | Validates that a `month` is valid. |
//! | `is_valid!(second, u32)` | Validates that a `second` is valid. |
//! | `is_valid_time!(time, u32)` | Validates that a `time` is valid. |
//! | `is_valid_iso_8601!(iso_8601, u32)` | Validates that a `iso_8601` is valid. |
//!
//! # Examples
//!
//! ```
//! use dtt::is_valid;
//! use dtt::DateTime;
//!
//! // Validation of a day
//! let input = "31".to_string();
//! is_valid!(day, String);
//! let result = day(&input);
//! assert!(result);
//!
//! // Validation of an hour
//! let input = "23".to_string();
//! is_valid!(hour, String);
//! let result = hour(&input);
//! assert!(result);
//!
//!
//! // Validation of a minute
//! let input = "59".to_string();
//! is_valid!(minute, String);
//! let result = minute(&input);
//! assert!(result);
//!
//! // Validation of a month
//! let input = "12".to_string();
//! is_valid!(month, String);
//! let result = month(&input);
//! assert!(result);
//!
//! // Validation of a second
//! let input = "59".to_string();
//! is_valid!(second, String);
//! let result = second(&input);
//! assert!(result);
//!
//! // Validation of a microsecond
//! let input = "999999".to_string();
//! is_valid!(microsecond, String);
//! let result = microsecond(&input);
//! assert!(result);
//!
//! // Validation of an ordinal
//! let input = "365".to_string();
//! is_valid!(ordinal, String);
//! let result = ordinal(&input);
//! assert!(result);
//!
//! // Validation of a time
//! let input = "23:59:59".to_string();
//! is_valid!(time, String);
//! let result = time(&input);
//! assert!(result);
//!
//! // Validation of an iso_8601
//! let input = "2022-01-01T12:00:00+01:00".to_string();
//! is_valid!(iso_8601, String);
//! let result = iso_8601(&input);
//! ```

/// This macro takes any number of arguments and parses them into a
/// Rust value.
#[macro_export]
macro_rules! dtt {
    ($($tt:tt)*) => {
        // Parse the arguments into a Rust value.
        $crate::parse!($($tt)*)
    };
}

/// This macro takes any number of arguments and parses them into a
/// Rust value.
#[macro_export]
macro_rules! parse {
    ($input:expr) => {
        DateTime::parse($input)
    };
}

/// This macro prints the arguments to the console.
#[macro_export]
macro_rules! dtt_print {
    ($($arg:tt)*) => {
        println!("{}", format_args!("{}", $($arg)*));
    };
}

/// This macro creates a new vector of the given elements.
#[macro_export]
macro_rules! dtt_vec {
    ($($elem:expr),*) => {{
        let mut v = Vec::new();
        $(v.push($elem);)*
        v
    }};
}

/// This macro creates a new map of the given key-value pairs.
#[macro_export]
macro_rules! dtt_map {
    ($($key:expr => $value:expr),*) => {{
        use std::collections::HashMap;
        let mut m = HashMap::new();
        $(m.insert($key, $value);)*
        m
    }};
}

/// This macro checks if the given expression is true.
#[macro_export]
macro_rules! dtt_assert {
    ($($arg:tt)*) => {
        if !$($arg)* {
            panic!("Assertion failed!");
        }
    };
}

/// This macro returns the minimum of the given values.
#[macro_export]
macro_rules! dtt_min {
    ($x:expr $(, $y:expr)*) => {{
        let mut min = $x;
        $(
            if min > $y { min = $y; }
        )*
        min
    }};
}

/// This macro returns the maximum of the given values.
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

/// This macro takes a string and splits it into a vector of words.
#[macro_export]
macro_rules! dtt_split {
    ($s:expr) => {{
        let mut v = Vec::new();
        for w in $s.split_whitespace() {
            v.push(w.to_string());
        }
        v
    }};
}

/// This macro takes a vector of strings and joins them together into a
/// single string.
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

/// This macro takes a vector of elements and prints them to the
/// console.
#[macro_export]
macro_rules! dtt_print_vec {
    ($($v:expr),*) => {{
        for v in $($v),* {
            println!("{}", v);
        }
    }};
}

/// This macro generates a function that takes a string as input and
/// returns a boolean.
#[macro_export]
macro_rules! is_valid {
    ($name:ident, $type:ty) => {
        fn $name(input: &str) -> bool {
            match input.parse::<$type>() {
                Ok(parsed_val) => match stringify!($name) {
                    "day" => {
                        DateTime::is_valid_day(&parsed_val.to_string())
                    }
                    "hour" => {
                        DateTime::is_valid_hour(&parsed_val.to_string())
                    }
                    "minute" => DateTime::is_valid_minute(
                        &parsed_val.to_string(),
                    ),
                    "month" => DateTime::is_valid_month(
                        &parsed_val.to_string(),
                    ),
                    "second" => DateTime::is_valid_second(
                        &parsed_val.to_string(),
                    ),
                    "microsecond" => DateTime::is_valid_microsecond(
                        &parsed_val.to_string(),
                    ),
                    "ordinal" => DateTime::is_valid_ordinal(
                        &parsed_val.to_string(),
                    ),
                    "time" => {
                        DateTime::is_valid_time(&parsed_val.to_string())
                    }
                    "iso_8601" => DateTime::is_valid_iso_8601(
                        &parsed_val.to_string(),
                    ),
                    _ => false,
                },
                Err(_) => false,
            }
        }
    };
}
