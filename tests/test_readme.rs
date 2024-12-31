// test_readme.rs
//
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Test Suite for `DateTime` (DTT) Library
//!
//! This module contains a series of unit tests that demonstrate and verify the functionality of the `DateTime` (DTT) library. The tests cover various aspects of date and time manipulation, validation, and parsing using the `DateTime` struct.

#[cfg(test)]
mod tests {
    use dtt::datetime::DateTime;
    use std::str::FromStr;
    use time::Month;

    /// Test to demonstrate basic usage of the `DateTime` struct.
    ///
    /// This test verifies the creation of a `DateTime` instance and the use of the `dtt_print!` macro.
    #[test]
    fn example_1() {
        use dtt::dtt_print;

        // Create a new `DateTime` instance representing the current UTC time.
        let now = DateTime::new();

        // Print the current `DateTime` instance using the `dtt_print!` macro.
        dtt_print!(now);
    }

    /// Test to validate the `DateTime` component validation functions.
    ///
    /// This test checks the validity of various components such as microsecond, second, minute,
    /// hour, month, ordinal, and ISO 8601 formatted strings.
    #[test]
    fn example_2() {
        // Validate microseconds
        assert!(DateTime::is_valid_microsecond("999999"));
        assert!(!DateTime::is_valid_microsecond("1000000"));

        // Validate seconds
        assert!(DateTime::is_valid_second("59"));
        assert!(!DateTime::is_valid_second("60"));

        // Validate minutes
        assert!(DateTime::is_valid_minute("59"));
        assert!(!DateTime::is_valid_minute("60"));

        // Validate hours
        assert!(DateTime::is_valid_hour("23"));
        assert!(!DateTime::is_valid_hour("24"));

        // Validate months
        assert!(DateTime::is_valid_month("12"));
        assert!(!DateTime::is_valid_month("13"));

        // Validate ordinal days
        assert!(DateTime::is_valid_ordinal("366"));
        assert!(!DateTime::is_valid_ordinal("367"));

        // Validate time format
        assert!(DateTime::is_valid_time("23:59:59"));
        assert!(!DateTime::is_valid_time("24:00:00"));

        // Validate ISO 8601 format
        assert!(DateTime::is_valid_iso_8601("2023-05-11T17:30:00Z"));
        assert!(DateTime::is_valid_iso_8601("2023-05-11T17:30:00Z"));
    }

    /// Test to parse a date-time string and verify individual components of the `DateTime` object.
    ///
    /// This test ensures that the `DateTime::from_str` function correctly parses a date-time string
    /// and that the resulting `DateTime` object has the expected year, month, day, hour, minute,
    /// second, ISO week, ordinal day, and microsecond values.
    #[test]
    fn example_3() -> Result<(), Box<dyn std::error::Error>> {
        let date_str = "2022-01-01T12:00:00+01:00";
        let result: DateTime = DateTime::from_str(date_str)?;

        // Assert the parsed components of the `DateTime` object
        assert_eq!(result.year(), 2022);
        assert_eq!(result.month(), Month::January);
        assert_eq!(result.day(), 1);
        assert_eq!(result.hour(), 12);
        assert_eq!(result.minute(), 0);
        assert_eq!(result.second(), 0);
        assert_eq!(result.iso_week(), 52);
        assert_eq!(result.ordinal(), 1);
        assert_eq!(result.microsecond(), 0);

        Ok(())
    }
}
