// test_datetime.rs
//
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Unit tests for the `DateTime` module.

use dtt::datetime::DateTime;
use dtt::error::DateTimeError;
use regex::Regex;
use std::hash::{DefaultHasher, Hash, Hasher};
use time::{Duration, UtcOffset, Weekday};

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests related to creation and initialization of `DateTime` objects.
    mod initialization_tests {
        use super::*;

        /// Test for creating a new `DateTime` instance with the default timezone (UTC).
        #[test]
        fn test_new() {
            let dt = DateTime::new();
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }

        /// Test for creating a new `DateTime` instance with a specific timezone.
        #[test]
        fn test_new_with_tz() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new_with_tz("EST")?;
            assert_eq!(dt.offset(), UtcOffset::from_hms(-5, 0, 0)?);
            Ok(())
        }

        /// Test for creating a new `DateTime` instance with a custom UTC offset.
        #[test]
        fn test_new_with_custom_offset(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new_with_custom_offset(5, 30)?;
            assert_eq!(dt.offset(), UtcOffset::from_hms(5, 30, 0)?);
            Ok(())
        }

        /// Test for creating a `DateTime` from its components.
        #[test]
        fn test_from_components(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                15,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
            Ok(())
        }

        /// Test for creating a `DateTime` with an invalid date.
        #[test]
        fn test_from_components_invalid_date() {
            let result = DateTime::from_components(
                2024,
                13,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            );
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
        }

        /// Test for creating a `DateTime` with an invalid time.
        #[test]
        fn test_from_components_invalid_time() {
            let result = DateTime::from_components(
                2024,
                1,
                1,
                24,
                0,
                0,
                UtcOffset::UTC,
            );
            assert!(matches!(result, Err(DateTimeError::InvalidTime)));
        }

        /// Test for handling an invalid timezone in `new_with_tz`.
        #[test]
        fn test_new_with_invalid_tz() {
            let result = DateTime::new_with_tz("INVALID");
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidTimezone)
            ));
        }

        /// Test for handling invalid custom offset in `new_with_custom_offset`.
        #[test]
        fn test_new_with_invalid_custom_offset() {
            let result = DateTime::new_with_custom_offset(25, 0); // Invalid hour offset
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidTimezone)
            ));

            let result = DateTime::new_with_custom_offset(23, 61); // Invalid minute offset
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidTimezone)
            ));

            let result = DateTime::new_with_custom_offset(-25, 0); // Negative invalid hour offset
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidTimezone)
            ));

            let result = DateTime::new_with_custom_offset(0, -61); // Negative invalid minute offset
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidTimezone)
            ));
        }
    }

    /// Tests related to parsing and formatting `DateTime` objects.
    mod parsing_and_formatting_tests {
        use super::*;

        /// Test for parsing a `DateTime` from a string.
        #[test]
        fn test_parse() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::parse("2024-08-31T15:00:00Z")?;
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
            Ok(())
        }

        /// Test for parsing a `DateTime` with a custom format.
        #[test]
        fn test_parse_custom_format(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::parse_custom_format(
                "2024-08-31 15:00:00",
                "[year]-[month]-[day] [hour]:[minute]:[second]",
            )?;
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            Ok(())
        }

        /// Test for handling an invalid format in `parse`.
        #[test]
        fn test_parse_invalid_format() {
            let result = DateTime::parse("invalid-date-format");
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidFormat)
            ));
        }

        /// Test for handling an invalid custom format in `parse_custom_format`.
        #[test]
        fn test_parse_custom_format_invalid() {
            let result = DateTime::parse_custom_format(
                "2024-01-01",
                "[invalid]",
            );
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidFormat)
            ));
        }

        /// Test for formatting a `DateTime` with a custom format string.
        #[test]
        fn test_format() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let formatted = dt.format("[year]-[month]-[day]")?;
            assert!(formatted.starts_with(&dt.year().to_string()));
            Ok(())
        }

        /// Test for formatting a `DateTime` in RFC 3339 format.
        #[test]
        fn test_format_rfc3339(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let rfc3339_format = dt.format_rfc3339()?;
            assert!(rfc3339_format.contains('T'));
            Ok(())
        }

        /// Test for formatting a `DateTime` in ISO 8601 format.
        #[test]
        fn test_format_iso8601(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let iso8601_format = dt.format_iso8601()?;
            assert!(iso8601_format.contains('T'));
            Ok(())
        }

        /// Test for the `Display` implementation of `DateTime`.
        #[test]
        fn test_display() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let display_string = format!("{}", dt);
            assert!(display_string.starts_with("2024-01-01T00:00:00"));
            Ok(())
        }
    }

    /// Tests related to arithmetic operations on `DateTime` objects.
    mod arithmetic_tests {
        use super::*;

        /// Test for adding a duration to a `DateTime`.
        #[test]
        fn test_add_duration() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let future_dt = (dt + Duration::days(1))?; // Unwrap the Result
            assert_eq!(future_dt.day(), 1);
            assert_eq!(future_dt.month() as u8, 9);
            Ok(())
        }

        /// Test for subtracting a duration from a `DateTime`.
        #[test]
        fn test_sub_duration() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new();
            let past_dt = (dt - Duration::days(1))?; // Unwrap the Result
            assert_eq!(
                past_dt.day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
            Ok(())
        }

        /// Test for adding a negative number of days to a `DateTime`.
        #[test]
        fn test_add_negative_days(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let result = dt.add_days(-1)?;
            assert_eq!(
                result.day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
            Ok(())
        }

        /// Test for adding a duration that results in an invalid date.
        #[test]
        fn test_add_duration_invalid(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                9999,
                12,
                31,
                23,
                59,
                59,
                UtcOffset::UTC,
            )?;
            let result = dt + Duration::days(1);
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
            Ok(())
        }

        /// Test for calculating the duration between two `DateTime` instances.
        #[test]
        fn test_duration_since(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1)?;
            let duration = dt1.duration_since(&dt2);
            assert_eq!(duration, Duration::seconds(-86400));
            Ok(())
        }
    }

    /// Tests related to comparison operations on `DateTime` objects.
    mod comparison_tests {
        use super::*;

        /// Test for the `PartialOrd` implementation of `DateTime`.
        #[test]
        fn test_partial_ord() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1)?;
            assert!(dt1 < dt2);
            Ok(())
        }

        /// Test for the `Ord` implementation of `DateTime`.
        #[test]
        fn test_ord() -> Result<(), Box<dyn std::error::Error>> {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1)?;
            assert!(dt1.cmp(&dt2) == std::cmp::Ordering::Less);
            Ok(())
        }

        /// Test for the `Hash` implementation of `DateTime`.
        #[test]
        fn test_hash() {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hasher;

            let dt = DateTime::new();
            let mut hasher = DefaultHasher::new();
            dt.hash(&mut hasher);
            let hash1 = hasher.finish();

            let mut hasher = DefaultHasher::new();
            dt.hash(&mut hasher);
            let hash2 = hasher.finish();

            assert_eq!(hash1, hash2);
        }

        /// Test for checking if a `DateTime` is within a specific range.
        #[test]
        fn test_is_within_range(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1)?;
            let dt3 = dt1.add_days(2)?;
            assert!(dt2.is_within_range(&dt1, &dt3));
            assert!(!dt1.is_within_range(&dt2, &dt3));
            Ok(())
        }
    }

    /// Tests related to week, month, and year operations on `DateTime` objects.
    mod calendar_operations_tests {
        use super::*;
        use time::Month;

        /// Test for getting the end of the month for a `DateTime`.
        #[test]
        fn test_end_of_month() {
            if let Ok(dt) = DateTime::new_with_tz("UTC") {
                if let Ok(end_of_month) = dt.end_of_month() {
                    assert!(
                        end_of_month.day() == 28
                            || end_of_month.day() == 29
                            || end_of_month.day() == 30
                            || end_of_month.day() == 31,
                        "Invalid day for end_of_month: {}",
                        end_of_month.day()
                    );
                } else {
                    eprintln!(
                        "Failed to get end_of_month for DateTime"
                    );
                }
            } else {
                eprintln!(
                    "Failed to create DateTime with UTC timezone"
                );
            }
        }

        /// Test for getting the start of the week for a `DateTime`.
        #[test]
        fn test_start_of_week() {
            if let Ok(dt) = DateTime::new_with_tz("UTC") {
                if let Ok(start_of_week) = dt.start_of_week() {
                    assert_eq!(
                        start_of_week.weekday(),
                        Weekday::Monday,
                        "Invalid start of week"
                    );
                } else {
                    eprintln!(
                        "Failed to get start_of_week for DateTime"
                    );
                }
            } else {
                eprintln!(
                    "Failed to create DateTime with UTC timezone"
                );
            }
        }

        /// Test for getting the end of the week for a `DateTime`.
        #[test]
        fn test_end_of_week() {
            if let Ok(dt) = DateTime::new_with_tz("UTC") {
                if let Ok(end_of_week) = dt.end_of_week() {
                    assert_eq!(
                        end_of_week.weekday(),
                        Weekday::Sunday,
                        "Invalid end of week"
                    );
                } else {
                    eprintln!("Failed to get end_of_week for DateTime");
                }
            } else {
                eprintln!(
                    "Failed to create DateTime with UTC timezone"
                );
            }
        }

        /// Test for getting the start of the month for a `DateTime`.
        #[test]
        fn test_start_of_month() {
            if let Ok(dt) = DateTime::new_with_tz("UTC") {
                if let Ok(start_of_month) = dt.start_of_month() {
                    assert_eq!(
                        start_of_month.day(),
                        1,
                        "Invalid start of month"
                    );
                } else {
                    eprintln!(
                        "Failed to get start_of_month for DateTime"
                    );
                }
            } else {
                eprintln!(
                    "Failed to create DateTime with UTC timezone"
                );
            }
        }

        /// Test for `end_of_month` method in different scenarios.
        #[test]
        fn test_end_of_month_scenarios(
        ) -> Result<(), Box<dyn std::error::Error>> {
            // Test for February in a leap year
            let dt = DateTime::from_components(
                2024,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.end_of_month()?.day(), 29);

            // Test for February in a non-leap year
            let dt = DateTime::from_components(
                2023,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.end_of_month()?.day(), 28);

            // Test for a 30-day month
            let dt = DateTime::from_components(
                2024,
                4,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.end_of_month()?.day(), 30);

            // Test for a 31-day month
            let dt = DateTime::from_components(
                2024,
                5,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.end_of_month()?.day(), 31);

            Ok(())
        }

        /// Test for getting the start of the year for a `DateTime`.
        #[test]
        fn test_start_of_year() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new_with_tz("UTC")?;
            let start_of_year = dt.start_of_year()?;
            assert_eq!(start_of_year.month() as u8, 1);
            assert_eq!(start_of_year.day(), 1);
            Ok(())
        }

        /// Test for getting the end of the year for a `DateTime`.
        #[test]
        fn test_end_of_year() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new_with_tz("UTC")?;
            let end_of_year = dt.end_of_year()?;
            assert_eq!(end_of_year.month() as u8, 12);
            assert_eq!(end_of_year.day(), 31);
            Ok(())
        }

        /// Test for getting the ordinal day of the year.
        #[test]
        fn test_ordinal_day() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.ordinal(), 1);
            Ok(())
        }

        /// Test for getting the ISO week number.
        #[test]
        fn test_iso_week() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert!(dt.iso_week() > 0 && dt.iso_week() <= 53);
            Ok(())
        }

        /// Test for microsecond precision in `DateTime`.
        #[test]
        fn test_microsecond_precision(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.microsecond(), 0);
            Ok(())
        }

        /// Test for getting the next day from a `DateTime`.
        #[test]
        fn test_next_day_mid_month(
        ) -> Result<(), Box<dyn std::error::Error>> {
            // A known date: Jan 15, 2024 => next_day => Jan 16, 2024
            let dt = DateTime::from_components(
                2024,
                1,
                15,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;

            let next_day = dt.next_day()?;
            assert_eq!(
                next_day.year(),
                2024,
                "Year should remain 2024"
            );
            assert_eq!(
                next_day.month(),
                Month::January,
                "Month should remain January"
            );
            assert_eq!(
                next_day.day(),
                16,
                "Day should increment to 16"
            );

            Ok(())
        }

        #[test]
        fn test_next_day_boundary(
        ) -> Result<(), Box<dyn std::error::Error>> {
            // E.g., Jan 31, 2024 => Feb 1, 2024 (2024 is a leap year, but January->February boundary is unaffected)
            let dt = DateTime::from_components(
                2024,
                1,
                31,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;

            let next_day = dt.next_day()?;
            assert_eq!(
                next_day.year(),
                2024,
                "Year should remain 2024"
            );
            assert_eq!(
                next_day.month(),
                Month::February,
                "Month should roll over to February"
            );
            assert_eq!(
                next_day.day(),
                1,
                "Day should roll over to the 1st"
            );

            Ok(())
        }

        /// Test for getting the previous day from a `DateTime`.
        #[test]
        fn test_previous_day_mid_month(
        ) -> Result<(), Box<dyn std::error::Error>> {
            // Scenario: use a date in the middle of the month to avoid boundary confusion
            let dt = DateTime::from_components(
                2024,
                1,
                15,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;

            // Expect January 14, 2024
            let prev_day = dt.previous_day()?;
            assert_eq!(
                prev_day.year(),
                2024,
                "Year should be the same"
            );
            assert_eq!(
                prev_day.month(),
                Month::January,
                "Month should still be January"
            );
            assert_eq!(prev_day.day(), 14, "Day should decrement by 1");

            Ok(())
        }

        #[test]
        fn test_previous_day_boundary(
        ) -> Result<(), Box<dyn std::error::Error>> {
            // Scenario: crossing a month boundary. For example, from March 1, 2024 to Feb 29, 2024.
            // 2024 is a leap year, so February has 29 days.
            let dt = DateTime::from_components(
                2024,
                3,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;

            // Expect February 29, 2024 (leap year)
            let prev_day = dt.previous_day()?;
            assert_eq!(
                prev_day.year(),
                2024,
                "Year should remain 2024"
            );
            assert_eq!(
                prev_day.month(),
                Month::February,
                "Month should roll back to February"
            );
            assert_eq!(
                prev_day.day(),
                29,
                "Should land on the leap day in 2024"
            );

            Ok(())
        }
    }

    /// Tests related to updating and converting `DateTime` objects.
    mod update_and_conversion_tests {
        use super::*;

        /// Test for updating a `DateTime` instance.
        #[test]
        fn test_update() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            std::thread::sleep(std::time::Duration::from_secs(2)); // Sleep for 2 seconds
            let updated_dt = dt.update()?; // Propagate error using `?`
            assert!(updated_dt > dt);
            Ok(())
        }

        /// Test for converting a `DateTime` instance to a different timezone.
        #[test]
        fn test_convert_to_tz() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new_with_tz("EST")?;
            let paris_time = dt.convert_to_tz("CET")?;
            let expected_offset = UtcOffset::from_hms(1, 0, 0)?;
            assert_eq!(paris_time.offset(), expected_offset);
            Ok(())
        }

        /// Test for handling an invalid timezone in `convert_to_tz`.
        #[test]
        fn test_convert_to_invalid_tz() {
            let dt = DateTime::new();
            let result = dt.convert_to_tz("INVALID");
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidTimezone)
            ));
        }

        /// Test for getting the Unix timestamp of a `DateTime` instance.
        #[test]
        fn test_unix_timestamp() {
            let dt = DateTime::new();
            let timestamp = dt.unix_timestamp();
            assert!(timestamp > 0);
        }

        /// Test for setting an invalid date in `set_date`.
        #[test]
        fn test_set_invalid_date() {
            let dt = DateTime::new();
            let result = dt.set_date(2024, 2, 30);
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
        }

        /// Test for setting an invalid time in `set_time`.
        #[test]
        fn test_set_invalid_time() {
            let dt = DateTime::new();
            let result = dt.set_time(24, 0, 0);
            assert!(matches!(result, Err(DateTimeError::InvalidTime)));
        }

        /// Test for the default `DateTime` instance.
        #[test]
        fn test_default() {
            let dt: DateTime = DateTime::default();
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }
    }

    /// Test suite for error handling in `DateTimeError`.
    mod error_handling {
        use super::*;

        #[test]
        fn test_invalid_timezone_error() {
            let result = DateTime::new_with_tz("INVALID");
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidTimezone)
            ));
        }

        #[test]
        fn test_invalid_date_error() {
            let result = DateTime::from_components(
                2024,
                13,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            );
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
        }

        #[test]
        fn test_invalid_time_error() {
            let result = DateTime::from_components(
                2024,
                1,
                1,
                24,
                0,
                0,
                UtcOffset::UTC,
            );
            assert!(matches!(result, Err(DateTimeError::InvalidTime)));
        }

        #[test]
        fn test_invalid_format_error() {
            let error = DateTimeError::InvalidFormat;
            assert_eq!(error.to_string(), "Invalid date format");
        }

        #[test]
        fn test_invalid_format_custom_error() {
            let result = DateTime::parse_custom_format(
                "2024-01-01",
                "[invalid]",
            );
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidFormat)
            ));
        }

        #[test]
        fn test_parse_error() {
            use time::format_description::well_known::Rfc3339;

            let invalid_datetime_str = "2023-02-30T25:61:61Z"; // Invalid date and time string
            let result = time::OffsetDateTime::parse(
                invalid_datetime_str,
                &Rfc3339,
            );

            assert!(result.is_err());
        }

        #[test]
        fn test_component_range_error() {
            use time::error::ComponentRange;
            use time::{Date, Month};

            // Example: Creating a date with an invalid day (32) for January.
            let result =
                Date::from_calendar_date(2023, Month::January, 32);

            assert!(
                matches!(result, Err(ComponentRange { .. })),
                "Expected a component range error due to invalid day."
            );
        }
    }

    /// Test suite for construction of `DateTime` objects.
    mod construction {
        use super::*;

        #[test]
        fn test_new() {
            let dt = DateTime::new();
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }

        #[test]
        fn test_new_with_tz() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new_with_tz("EST")?;
            let expected_offset = UtcOffset::from_hms(-5, 0, 0)?;
            assert_eq!(dt.offset(), expected_offset);
            Ok(())
        }

        #[test]
        fn test_new_with_custom_offset(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new_with_custom_offset(5, 30)?;
            let expected_offset = UtcOffset::from_hms(5, 30, 0)?;
            assert_eq!(dt.offset(), expected_offset);
            Ok(())
        }

        #[test]
        fn test_from_components(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                15,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
            Ok(())
        }

        #[test]
        fn test_default() {
            let dt: DateTime = DateTime::default();
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }
    }

    /// Test suite for date and time manipulation methods.
    mod date_time_manipulation {
        use super::*;

        #[test]
        fn test_add_duration() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let future_dt = (dt + Duration::days(1))?;
            assert_eq!(future_dt.day(), 1);
            assert_eq!(future_dt.month() as u8, 9);
            Ok(())
        }

        #[test]
        fn test_sub_duration() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new();
            let past_dt = (dt - Duration::days(1))?;
            assert_eq!(
                past_dt.day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
            Ok(())
        }

        #[test]
        fn test_add_negative_days(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let updated_dt = dt.add_days(-1)?;
            assert_eq!(
                updated_dt.day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
            Ok(())
        }

        #[test]
        fn test_update() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            std::thread::sleep(std::time::Duration::from_secs(2)); // Sleep for 2 seconds
            let updated_dt = dt.update()?;
            assert!(updated_dt > dt);
            Ok(())
        }

        #[test]
        fn test_set_invalid_date() {
            let dt = DateTime::new();
            let result = dt.set_date(2024, 2, 30);
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
        }

        #[test]
        fn test_set_invalid_time() {
            let dt = DateTime::new();
            let result = dt.set_time(24, 0, 0);
            assert!(matches!(result, Err(DateTimeError::InvalidTime)));
        }
    }

    /// Test suite for timezone handling methods.
    mod timezone_handling {
        use super::*;

        #[test]
        fn test_convert_to_tz() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new_with_tz("EST")?;
            let paris_time = dt.convert_to_tz("CET")?;
            let expected_offset = UtcOffset::from_hms(1, 0, 0)?;
            assert_eq!(paris_time.offset(), expected_offset);
            Ok(())
        }

        #[test]
        fn test_convert_to_invalid_tz() {
            let dt = DateTime::new();
            let result = dt.convert_to_tz("INVALID");
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidTimezone)
            ));
        }
    }

    /// Test suite for formatting and parsing methods.
    mod formatting_parsing {
        use super::*;

        #[test]
        fn test_parse() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::parse("2024-08-31T15:00:00Z")?;
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
            Ok(())
        }

        #[test]
        fn test_parse_custom_format(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::parse_custom_format(
                "2024-08-31 15:00:00",
                "[year]-[month]-[day] [hour]:[minute]:[second]",
            )?;
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            Ok(())
        }

        #[test]
        fn test_format() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let formatted = dt.format("[year]-[month]-[day]")?;
            assert!(formatted.starts_with(&dt.year().to_string()));
            Ok(())
        }

        #[test]
        fn test_format_rfc3339(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let rfc3339_format = dt.format_rfc3339()?;
            assert!(rfc3339_format.contains('T'));
            Ok(())
        }

        #[test]
        fn test_format_iso8601(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let iso8601_format = dt.format_iso8601()?;
            assert!(iso8601_format.contains('T'));
            Ok(())
        }
    }

    /// Test suite for comparisons and hashing.
    mod comparisons_hashing {
        use super::*;
        use std::collections::hash_map::DefaultHasher;

        #[test]
        fn test_partial_ord() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1)?;
            assert!(dt1 < dt2);
            Ok(())
        }

        #[test]
        fn test_ord() -> Result<(), Box<dyn std::error::Error>> {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1)?;
            assert!(dt1.cmp(&dt2) == std::cmp::Ordering::Less);
            Ok(())
        }

        #[test]
        fn test_hash() {
            let dt = DateTime::new();
            let mut hasher = DefaultHasher::new();
            dt.hash(&mut hasher);
            let hash1 = hasher.finish();

            let mut hasher = DefaultHasher::new();
            dt.hash(&mut hasher);
            let hash2 = hasher.finish();

            assert_eq!(hash1, hash2);
        }

        #[test]
        fn test_is_within_range(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1)?;
            let dt3 = dt1.add_days(2)?;
            assert!(dt2.is_within_range(&dt1, &dt3));
            assert!(!dt1.is_within_range(&dt2, &dt3));
            Ok(())
        }
    }

    /// Test suite for edge cases and other functionalities.
    mod edge_cases {
        use super::*;

        #[test]
        fn test_add_duration_invalid(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                9999,
                12,
                31,
                23,
                59,
                59,
                UtcOffset::UTC,
            )?;
            let result = dt + Duration::days(1);
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
            Ok(())
        }

        #[test]
        fn test_end_of_month_scenarios(
        ) -> Result<(), Box<dyn std::error::Error>> {
            // Test for February in a leap year
            let dt = DateTime::from_components(
                2024,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let end_date = dt.end_of_month()?;
            assert_eq!(end_date.day(), 29);

            // Test for February in a non-leap year
            let dt = DateTime::from_components(
                2023,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let end_date = dt.end_of_month()?;
            assert_eq!(end_date.day(), 28);

            // Test for a 30-day month
            let dt = DateTime::from_components(
                2024,
                4,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let end_date = dt.end_of_month()?;
            assert_eq!(end_date.day(), 30);

            // Test for a 31-day month
            let dt = DateTime::from_components(
                2024,
                5,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let end_date = dt.end_of_month()?;
            assert_eq!(end_date.day(), 31);

            Ok(())
        }

        #[test]
        fn test_microsecond_precision(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.microsecond(), 0);
            Ok(())
        }

        #[test]
        fn test_ordinal_day() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.ordinal(), 1);
            Ok(())
        }

        #[test]
        fn test_iso_week() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert!(dt.iso_week() > 0 && dt.iso_week() <= 53);
            Ok(())
        }

        #[test]
        fn test_display() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let display_string = format!("{}", dt);
            assert!(display_string.starts_with("2024-01-01T00:00:00"));
            Ok(())
        }
    }

    /// Tests for `DateTimeError` enum.
    mod datetime_error_tests {
        use super::*;

        #[test]
        fn test_invalid_format_error() {
            let error = DateTimeError::InvalidFormat;
            assert_eq!(error.to_string(), "Invalid date format");
        }

        #[test]
        fn test_invalid_timezone_error() {
            let err = DateTimeError::InvalidTimezone;
            assert_eq!(
                format!("{}", err),
                "Invalid or unsupported timezone; DST not supported"
            );
        }

        #[test]
        fn test_invalid_date_error() {
            let err = DateTimeError::InvalidDate;
            assert_eq!(format!("{}", err), "Invalid date");
        }

        #[test]
        fn test_invalid_time_error() {
            let err = DateTimeError::InvalidTime;
            assert_eq!(format!("{}", err), "Invalid time");
        }
    }

    /// Tests for `DateTime` creation methods.
    mod datetime_creation_tests {
        use super::*;

        #[test]
        fn test_new() {
            let dt = DateTime::new();
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }

        #[test]
        fn test_new_with_tz() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new_with_tz("EST")?;
            let expected_offset = UtcOffset::from_hms(-5, 0, 0)?;
            assert_eq!(dt.offset(), expected_offset);
            Ok(())
        }

        #[test]
        fn test_new_with_custom_offset(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new_with_custom_offset(5, 30)?;
            let expected_offset = UtcOffset::from_hms(5, 30, 0)?;
            assert_eq!(dt.offset(), expected_offset);
            Ok(())
        }

        #[test]
        fn test_from_components(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                15,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
            Ok(())
        }

        #[test]
        fn test_parse() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::parse("2024-08-31T15:00:00Z")?;
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
            Ok(())
        }

        #[test]
        fn test_parse_custom_format(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::parse_custom_format(
                "2024-08-31 15:00:00",
                "[year]-[month]-[day] [hour]:[minute]:[second]",
            )?;
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            Ok(())
        }

        #[test]
        fn test_update() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            std::thread::sleep(std::time::Duration::from_secs(2));
            let updated_dt = dt.update()?;
            assert!(
                updated_dt > dt,
                "Expected updated_dt to be later than dt"
            );
            Ok(())
        }

        #[test]
        fn test_convert_to_tz() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new_with_tz("EST")?;
            let paris_time = dt.convert_to_tz("CET")?;
            let expected_offset = UtcOffset::from_hms(1, 0, 0)?;
            assert_eq!(paris_time.offset(), expected_offset);
            Ok(())
        }

        #[test]
        fn test_format_rfc3339(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let rfc3339_format = dt.format_rfc3339()?;
            assert!(rfc3339_format.contains('T'));
            Ok(())
        }

        #[test]
        fn test_format_iso8601(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let iso8601_format = dt.format_iso8601()?;
            assert!(iso8601_format.contains('T'));
            Ok(())
        }

        #[test]
        fn test_default() {
            let dt: DateTime = DateTime::default();
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }

        #[test]
        fn test_add_duration() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let future_dt = (dt + Duration::days(1))?;
            assert_eq!(future_dt.day(), 1);
            assert_eq!(future_dt.month() as u8, 9);
            Ok(())
        }

        #[test]
        fn test_sub_duration() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::new();
            let past_dt = (dt - Duration::days(1))?;
            assert_eq!(
                past_dt.day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
            Ok(())
        }

        #[test]
        fn test_add_duration_invalid(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                9999,
                12,
                31,
                23,
                59,
                59,
                UtcOffset::UTC,
            )?;
            let result = dt + Duration::days(1);
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
            Ok(())
        }

        #[test]
        fn test_add_negative_days(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new();
            let updated_dt = dt.add_days(-1)?;
            assert_eq!(
                updated_dt.day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
            Ok(())
        }

        #[test]
        fn test_set_invalid_date() {
            let dt = DateTime::new();
            let result = dt.set_date(2024, 2, 30);
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
        }

        #[test]
        fn test_set_invalid_time() {
            let dt = DateTime::new();
            let result = dt.set_time(24, 0, 0);
            assert!(matches!(result, Err(DateTimeError::InvalidTime)));
        }

        #[test]
        fn test_display() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let display_string = format!("{}", dt);
            assert!(display_string.starts_with("2024-01-01T00:00:00"));
            Ok(())
        }

        #[test]
        fn test_end_of_month_scenarios() {
            // Test for February in a leap year
            if let Ok(dt) = DateTime::from_components(
                2024,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            ) {
                if let Ok(end_of_month) = dt.end_of_month() {
                    assert_eq!(end_of_month.day(), 29);
                } else {
                    eprintln!("Failed to get end_of_month for 2024-02");
                }
            } else {
                eprintln!("Failed to create DateTime for 2024-02-01");
            }

            // Test for February in a non-leap year
            if let Ok(dt) = DateTime::from_components(
                2023,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            ) {
                if let Ok(end_of_month) = dt.end_of_month() {
                    assert_eq!(end_of_month.day(), 28);
                } else {
                    eprintln!("Failed to get end_of_month for 2023-02");
                }
            } else {
                eprintln!("Failed to create DateTime for 2023-02-01");
            }

            // Test for a 30-day month
            if let Ok(dt) = DateTime::from_components(
                2024,
                4,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            ) {
                if let Ok(end_of_month) = dt.end_of_month() {
                    assert_eq!(end_of_month.day(), 30);
                } else {
                    eprintln!("Failed to get end_of_month for 2024-04");
                }
            } else {
                eprintln!("Failed to create DateTime for 2024-04-01");
            }

            // Test for a 31-day month
            if let Ok(dt) = DateTime::from_components(
                2024,
                5,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            ) {
                if let Ok(end_of_month) = dt.end_of_month() {
                    assert_eq!(end_of_month.day(), 31);
                } else {
                    eprintln!("Failed to get end_of_month for 2024-05");
                }
            } else {
                eprintln!("Failed to create DateTime for 2024-05-01");
            }
        }

        #[test]
        fn test_microsecond_precision(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.microsecond(), 0);
            Ok(())
        }

        #[test]
        fn test_ordinal_day() -> Result<(), Box<dyn std::error::Error>>
        {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert_eq!(dt.ordinal(), 1);
            Ok(())
        }

        #[test]
        fn test_iso_week() -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            assert!(dt.iso_week() > 0 && dt.iso_week() <= 53);
            Ok(())
        }

        #[test]
        fn test_new_with_unique_timezone_offset(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new_with_tz("WADT")?;
            let offset = UtcOffset::from_hms(8, 45, 0)?;
            assert_eq!(dt.offset(), offset);
            Ok(())
        }

        #[test]
        fn test_new_with_custom_offset_edge_cases(
        ) -> Result<(), Box<dyn std::error::Error>> {
            // Test with positive custom offset
            let dt = DateTime::new_with_custom_offset(23, 59)?;
            let expected_offset = UtcOffset::from_hms(23, 59, 0)?;
            assert_eq!(dt.offset(), expected_offset);

            // Test with negative custom offset
            let dt = DateTime::new_with_custom_offset(-23, -59)?;
            let expected_offset = UtcOffset::from_hms(-23, -59, 0)?;
            assert_eq!(dt.offset(), expected_offset);

            Ok(())
        }
    }

    mod update_and_conversion {
        use super::*;

        #[test]
        fn test_convert_to_dst_timezone(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new_with_tz("EST")?;
            let dst_time = dt.convert_to_tz("EDT")?;
            let expected_offset = UtcOffset::from_hms(-4, 0, 0)?;
            assert_eq!(dst_time.offset(), expected_offset);
            Ok(())
        }

        #[test]
        fn test_convert_multiple_timezones(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::new_with_tz("EST")?;
            let dt_pst = dt.convert_to_tz("PST")?;
            let dt_gmt = dt_pst.convert_to_tz("GMT")?;
            assert_eq!(dt_gmt.offset(), UtcOffset::UTC);
            Ok(())
        }
    }

    mod arithmetic_operations {
        use super::*;

        #[test]
        fn test_add_duration_across_year_end(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                12,
                31,
                23,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let future_dt = (dt + Duration::hours(2))?;
            assert_eq!(future_dt.year(), 2025);
            assert_eq!(future_dt.month() as u8, 1);
            assert_eq!(future_dt.day(), 1);
            assert_eq!(future_dt.hour(), 1);
            Ok(())
        }

        #[test]
        fn test_sub_duration_across_year_start(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                1,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let past_dt = (dt - Duration::hours(2))?;
            assert_eq!(past_dt.year(), 2023);
            assert_eq!(past_dt.month() as u8, 12);
            assert_eq!(past_dt.day(), 31);
            assert_eq!(past_dt.hour(), 23);
            Ok(())
        }
    }

    mod comparative_and_hashing {
        use super::*;

        #[test]
        fn test_hash_uniqueness(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt1 = DateTime::from_components(
                2024,
                1,
                1,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let dt2 = DateTime::from_components(
                2024,
                1,
                1,
                12,
                0,
                0,
                UtcOffset::from_hms(1, 0, 0)?,
            )?;

            let mut hasher1 = DefaultHasher::new();
            dt1.hash(&mut hasher1);
            let hash1 = hasher1.finish();

            let mut hasher2 = DefaultHasher::new();
            dt2.hash(&mut hasher2);
            let hash2 = hasher2.finish();

            assert_ne!(hash1, hash2);
            Ok(())
        }
    }

    mod start_end_functions {
        use super::*;

        #[test]
        fn test_end_of_february_leap_year(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let end_of_feb = dt.end_of_month()?;
            assert_eq!(end_of_feb.day(), 29);
            Ok(())
        }

        #[test]
        fn test_end_of_february_non_leap_year(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let end_of_feb = dt.end_of_month()?;
            assert_eq!(end_of_feb.day(), 28);
            Ok(())
        }
    }

    #[cfg(test)]
    mod datetime_tests {
        use super::*;
        use time::{Date, Month, PrimitiveDateTime, Time, UtcOffset};

        #[test]
        fn test_datetime_creation(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let date =
                Date::from_calendar_date(2023, Month::January, 1)
                    .map_err(|err| {
                        format!("Failed to create Date: {:?}", err)
                    })?;
            let time = Time::from_hms(12, 0, 0).map_err(|err| {
                format!("Failed to create Time: {:?}", err)
            })?;
            let datetime = PrimitiveDateTime::new(date, time);
            let offset =
                UtcOffset::from_hms(0, 0, 0).map_err(|err| {
                    format!("Failed to create UtcOffset: {:?}", err)
                })?;
            let dt = DateTime { datetime, offset };

            assert_eq!(dt.datetime, datetime);
            assert_eq!(dt.offset, offset);
            Ok(())
        }

        #[test]
        fn test_datetime_serialization(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let date =
                Date::from_calendar_date(2023, Month::January, 1)
                    .map_err(|err| {
                        format!("Failed to create Date: {:?}", err)
                    })?;
            let time = Time::from_hms(12, 0, 0).map_err(|err| {
                format!("Failed to create Time: {:?}", err)
            })?;
            let datetime = PrimitiveDateTime::new(date, time);
            let offset =
                UtcOffset::from_hms(0, 0, 0).map_err(|err| {
                    format!("Failed to create UtcOffset: {:?}", err)
                })?;
            let dt = DateTime { datetime, offset };

            let serialized =
                serde_json::to_string(&dt).map_err(|err| {
                    format!("Failed to serialize DateTime: {:?}", err)
                })?;
            let deserialized: DateTime =
                serde_json::from_str(&serialized).map_err(|err| {
                    format!("Failed to deserialize DateTime: {:?}", err)
                })?;

            assert_eq!(dt, deserialized);
            Ok(())
        }

        #[test]
        fn test_datetime_comparison(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let date1 =
                Date::from_calendar_date(2023, Month::January, 1)
                    .map_err(|err| {
                        format!("Failed to create Date: {:?}", err)
                    })?;
            let time1 = Time::from_hms(12, 0, 0).map_err(|err| {
                format!("Failed to create Time: {:?}", err)
            })?;
            let datetime1 = PrimitiveDateTime::new(date1, time1);
            let offset1 =
                UtcOffset::from_hms(0, 0, 0).map_err(|err| {
                    format!("Failed to create UtcOffset: {:?}", err)
                })?;
            let dt1 = DateTime {
                datetime: datetime1,
                offset: offset1,
            };

            let date2 =
                Date::from_calendar_date(2023, Month::January, 2)
                    .map_err(|err| {
                        format!("Failed to create Date: {:?}", err)
                    })?;
            let time2 = Time::from_hms(12, 0, 0).map_err(|err| {
                format!("Failed to create Time: {:?}", err)
            })?;
            let datetime2 = PrimitiveDateTime::new(date2, time2);
            let offset2 =
                UtcOffset::from_hms(0, 0, 0).map_err(|err| {
                    format!("Failed to create UtcOffset: {:?}", err)
                })?;
            let dt2 = DateTime {
                datetime: datetime2,
                offset: offset2,
            };

            assert_ne!(dt1, dt2);
            assert!(dt1 < dt2);
            Ok(())
        }

        #[test]
        fn test_datetime_offset_handling(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let date =
                Date::from_calendar_date(2023, Month::January, 1)
                    .map_err(|err| {
                        format!("Failed to create Date: {:?}", err)
                    })?;
            let time = Time::from_hms(12, 0, 0).map_err(|err| {
                format!("Failed to create Time: {:?}", err)
            })?;
            let datetime = PrimitiveDateTime::new(date, time);
            let offset =
                UtcOffset::from_hms(2, 0, 0).map_err(|err| {
                    format!("Failed to create UtcOffset: {:?}", err)
                })?;
            let dt = DateTime { datetime, offset };

            assert_eq!(dt.offset, offset);
            Ok(())
        }

        #[test]
        fn test_datetime_copy_clone(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let date =
                Date::from_calendar_date(2023, Month::January, 1)
                    .map_err(|err| {
                        format!("Failed to create Date: {:?}", err)
                    })?;
            let time = Time::from_hms(12, 0, 0).map_err(|err| {
                format!("Failed to create Time: {:?}", err)
            })?;
            let datetime = PrimitiveDateTime::new(date, time);
            let offset =
                UtcOffset::from_hms(0, 0, 0).map_err(|err| {
                    format!("Failed to create UtcOffset: {:?}", err)
                })?;
            let dt = DateTime { datetime, offset };

            let dt_copy = dt;

            assert_eq!(dt, dt_copy);
            Ok(())
        }

        #[test]
        fn test_datetime_debug_output() {
            let date = if let Ok(date) =
                Date::from_calendar_date(2023, Month::January, 1)
            {
                date
            } else {
                assert_eq!(
                    Ok::<(), &str>(()),
                    Err("Failed to create Date")
                );
                return;
            };
            let time = if let Ok(time) = Time::from_hms(12, 0, 0) {
                time
            } else {
                assert_eq!(
                    Ok::<(), &str>(()),
                    Err("Failed to create Time")
                );
                return;
            };
            let datetime = PrimitiveDateTime::new(date, time);
            let offset =
                if let Ok(offset) = UtcOffset::from_hms(0, 0, 0) {
                    offset
                } else {
                    assert_eq!(
                        Ok::<(), &str>(()),
                        Err("Failed to create UtcOffset")
                    );
                    return;
                };
            let dt = DateTime { datetime, offset };

            let debug_output = format!("{:?}", dt);
            assert!(debug_output.contains("DateTime"));
            assert!(debug_output.contains("datetime"));
            assert!(debug_output.contains("offset"));
        }

        #[test]
        fn test_new_with_invalid_timezone() {
            assert!(DateTime::new_with_tz("InvalidTZ").is_err());
        }

        #[test]
        fn test_new_with_custom_offset_invalid() {
            assert!(DateTime::new_with_custom_offset(25, 0).is_err());
            assert!(DateTime::new_with_custom_offset(0, 60).is_err());
        }

        #[test]
        fn test_parse_invalid_format() {
            assert!(DateTime::parse("not a date").is_err());
        }

        #[test]
        fn test_add_days_overflow(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                9999,
                12,
                31,
                23,
                59,
                59,
                UtcOffset::UTC,
            )?;
            assert!(dt.add_days(1).is_err());
            Ok(())
        }

        #[test]
        fn test_convert_to_invalid_timezone() {
            let dt = DateTime::new();
            assert!(dt.convert_to_tz("InvalidTZ").is_err());
        }

        #[test]
        fn test_is_valid_day_edge_cases() {
            assert!(DateTime::is_valid_day("1"));
            assert!(DateTime::is_valid_day("31"));
            assert!(!DateTime::is_valid_day("0"));
            assert!(!DateTime::is_valid_day("32"));
            assert!(!DateTime::is_valid_day("-1"));
        }
    }

    /// Tests for `DateTime` arithmetic operations (months and years)
    mod datetime_arithmetic_tests {
        use super::*;

        #[test]
        fn test_add_months_basic(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                1,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.add_months(1)?;
            assert_eq!(result.year(), 2023);
            assert_eq!(result.month() as u8, 2);
            assert_eq!(result.day(), 15);
            Ok(())
        }

        #[test]
        fn test_add_months_cross_year(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                12,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.add_months(2)?;
            assert_eq!(result.year(), 2024);
            assert_eq!(result.month() as u8, 2);
            assert_eq!(result.day(), 15);
            Ok(())
        }

        #[test]
        fn test_add_months_last_day_of_month(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                1,
                31,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.add_months(1)?;
            assert_eq!(result.year(), 2023);
            assert_eq!(result.month() as u8, 2);
            assert_eq!(result.day(), 28); // February has 28 days in 2023
            Ok(())
        }

        #[test]
        fn test_add_months_leap_year(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2024,
                1,
                31,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.add_months(1)?;
            assert_eq!(result.year(), 2024);
            assert_eq!(result.month() as u8, 2);
            assert_eq!(result.day(), 29); // February has 29 days in 2024 (leap year)
            Ok(())
        }

        #[test]
        fn test_sub_months_basic(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                3,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.sub_months(1)?;
            assert_eq!(result.year(), 2023);
            assert_eq!(result.month() as u8, 2);
            assert_eq!(result.day(), 15);
            Ok(())
        }

        #[test]
        fn test_sub_months_cross_year(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                1,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.sub_months(2)?;
            assert_eq!(result.year(), 2022);
            assert_eq!(result.month() as u8, 11);
            assert_eq!(result.day(), 15);
            Ok(())
        }

        #[test]
        fn test_sub_months_last_day_of_month(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                3,
                31,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.sub_months(1)?;
            assert_eq!(result.year(), 2023);
            assert_eq!(result.month() as u8, 2);
            assert_eq!(result.day(), 28); // February has 28 days in 2023
            Ok(())
        }

        #[test]
        fn test_add_years_basic(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                5,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.add_years(1)?;
            assert_eq!(result.year(), 2024);
            assert_eq!(result.month() as u8, 5);
            assert_eq!(result.day(), 15);
            Ok(())
        }

        #[test]
        fn test_sub_years_basic(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                5,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.sub_years(1)?;

            assert_eq!(result.year(), 2022);
            assert_eq!(result.month() as u8, 5);
            assert_eq!(result.day(), 15);

            Ok(())
        }

        #[test]
        fn test_add_years_leap_year() {
            if let Ok(dt) = DateTime::from_components(
                2024,
                2,
                29,
                12,
                0,
                0,
                UtcOffset::UTC,
            ) {
                if let Ok(result) = dt.add_years(1) {
                    assert_eq!(result.year(), 2025);
                    assert_eq!(result.month() as u8, 2);
                    assert_eq!(result.day(), 28); // February 28 in non-leap year
                } else {
                    eprintln!(
                        "Failed to add years to the DateTime object"
                    );
                }
            } else {
                eprintln!(
                    "Failed to create DateTime for testing leap year"
                );
            }
        }

        #[test]
        fn test_add_months_large_number(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                1,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.add_months(25)?;

            assert_eq!(result.year(), 2025);
            assert_eq!(result.month() as u8, 2);
            assert_eq!(result.day(), 15);

            Ok(())
        }

        #[test]
        fn test_sub_months_large_number(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                1,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.sub_months(25)?;

            assert_eq!(result.year(), 2020);
            assert_eq!(result.month() as u8, 12);
            assert_eq!(result.day(), 15);

            Ok(())
        }

        #[test]
        fn test_add_years_large_number(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                5,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.add_years(100)?;

            assert_eq!(result.year(), 2123);
            assert_eq!(result.month() as u8, 5);
            assert_eq!(result.day(), 15);

            Ok(())
        }

        #[test]
        fn test_sub_years_large_number(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                5,
                15,
                12,
                0,
                0,
                UtcOffset::UTC,
            )?;
            let result = dt.sub_years(100)?;

            assert_eq!(result.year(), 1923);
            assert_eq!(result.month() as u8, 5);
            assert_eq!(result.day(), 15);

            Ok(())
        }

        #[test]
        fn test_add_months_preserve_time(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                1,
                15,
                23,
                59,
                59,
                UtcOffset::UTC,
            )?;
            let result = dt.add_months(1)?;

            assert_eq!(result.hour(), 23);
            assert_eq!(result.minute(), 59);
            assert_eq!(result.second(), 59);

            Ok(())
        }

        #[test]
        fn test_add_years_preserve_time(
        ) -> Result<(), Box<dyn std::error::Error>> {
            let dt = DateTime::from_components(
                2023,
                1,
                15,
                23,
                59,
                59,
                UtcOffset::UTC,
            )?;
            let result = dt.add_years(1)?;

            assert_eq!(result.hour(), 23);
            assert_eq!(result.minute(), 59);
            assert_eq!(result.second(), 59);

            Ok(())
        }
    }

    mod format_time_in_timezone_tests {
        use super::*;

        #[test]
        fn test_format_time_in_timezone_utc() {
            // Create an instance of DateTime.
            // For example, use the current time or a fixed time.
            let now = DateTime::new();

            // Call `format_time_in_timezone` as an instance method on `now`.
            let result = now.format_time_in_timezone(
                "UTC",
                "[hour]:[minute]:[second]",
            );
            assert!(result.is_ok());
            println!("Formatted result: {:?}", result);
        }

        #[test]
        fn test_format_time_in_timezone_pst(
        ) -> Result<(), Box<dyn std::error::Error>> {
            // Create a DateTime instance
            let now = DateTime::new();

            // Format time in PST timezone
            let result = now.format_time_in_timezone(
                "PST",
                "[hour repr:12]:[minute] [period]",
            )?;

            // Compile the regex
            let re = Regex::new(r"^\d{2}:\d{2} (AM|PM)$")?;

            // Assert the formatted time matches the expected pattern
            assert!(
                re.is_match(&result),
                "Formatted time '{}' does not match expected pattern",
                result
            );

            Ok(())
        }

        #[test]
        fn test_format_time_in_timezone_invalid_tz() {
            // Again, create an instance of DateTime
            let now = DateTime::new();

            // Call the method as an instance method
            let result = now
                .format_time_in_timezone("INVALID", "[hour]:[minute]");

            // Validate we get an error matching `DateTimeError::InvalidTimezone`
            assert!(
                result.is_err(),
                "Expected an Err result with invalid timezone"
            );
            if let Err(e) = result {
                assert!(
                    matches!(e, DateTimeError::InvalidTimezone),
                    "Expected DateTimeError::InvalidTimezone, got {:?}",
                    e
                );
            }
        }
    }
}
