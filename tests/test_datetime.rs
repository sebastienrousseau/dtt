// test_datetime.rs
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! Unit tests for the `DateTime` module.

use dtt::datetime::{DateTime, DateTimeError};
use std::hash::{Hash, Hasher};
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
        fn test_new_with_tz() {
            let dt = DateTime::new_with_tz("EST").unwrap();
            assert_eq!(
                dt.offset(),
                UtcOffset::from_hms(-5, 0, 0).unwrap()
            );
        }

        /// Test for creating a new `DateTime` instance with a custom UTC offset.
        #[test]
        fn test_new_with_custom_offset() {
            let dt = DateTime::new_with_custom_offset(5, 30).unwrap();
            assert_eq!(
                dt.offset(),
                UtcOffset::from_hms(5, 30, 0).unwrap()
            );
        }

        /// Test for creating a `DateTime` from its components.
        #[test]
        fn test_from_components() {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                15,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
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
        fn test_parse() {
            let dt = DateTime::parse("2024-08-31T15:00:00Z").unwrap();
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }

        /// Test for parsing a `DateTime` with a custom format.
        #[test]
        fn test_parse_custom_format() {
            let dt = DateTime::parse_custom_format(
                "2024-08-31 15:00:00",
                "[year]-[month]-[day] [hour]:[minute]:[second]",
            )
            .unwrap();
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
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
        fn test_format() {
            let dt = DateTime::new();
            let formatted = dt.format("[year]-[month]-[day]").unwrap();
            assert!(formatted.starts_with(&dt.year().to_string()));
        }

        /// Test for formatting a `DateTime` in RFC 3339 format.
        #[test]
        fn test_format_rfc3339() {
            let dt = DateTime::new();
            let rfc3339_format = dt.format_rfc3339().unwrap();
            assert!(rfc3339_format.contains("T"));
        }

        /// Test for formatting a `DateTime` in ISO 8601 format.
        #[test]
        fn test_format_iso8601() {
            let dt = DateTime::new();
            let iso8601_format = dt.format_iso8601().unwrap();
            assert!(iso8601_format.contains("T"));
        }

        /// Test for the `Display` implementation of `DateTime`.
        #[test]
        fn test_display() {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            let display_string = format!("{}", dt);
            assert!(display_string.starts_with("2024-01-01T00:00:00"));
        }
    }

    /// Tests related to arithmetic operations on `DateTime` objects.
    mod arithmetic_tests {
        use super::*;

        /// Test for adding a duration to a `DateTime`.
        #[test]
        fn test_add_duration() {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                12,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            let future_dt = (dt + Duration::days(1)).unwrap();
            assert_eq!(future_dt.day(), 1);
            assert_eq!(future_dt.month() as u8, 9);
        }

        /// Test for subtracting a duration from a `DateTime`.
        #[test]
        fn test_sub_duration() {
            let dt = DateTime::new();
            let past_dt = (dt - Duration::days(1)).unwrap();
            assert_eq!(
                past_dt.day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
        }

        /// Test for adding a negative number of days to a `DateTime`.
        #[test]
        fn test_add_negative_days() {
            let dt = DateTime::new();
            let result = dt.add_days(-1);
            assert!(result.is_ok());
            assert_eq!(
                result.unwrap().day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
        }

        /// Test for adding a duration that results in an invalid date.
        #[test]
        fn test_add_duration_invalid() {
            let dt = DateTime::from_components(
                9999,
                12,
                31,
                23,
                59,
                59,
                UtcOffset::UTC,
            )
            .unwrap();
            let result = dt + Duration::days(1);
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
        }

        /// Test for calculating the duration between two `DateTime` instances.
        #[test]
        fn test_duration_since() {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1).unwrap();
            let duration = dt1.duration_since(&dt2);
            assert_eq!(duration, Duration::seconds(-86400));
        }
    }

    /// Tests related to comparison operations on `DateTime` objects.
    mod comparison_tests {
        use super::*;

        /// Test for the `PartialOrd` implementation of `DateTime`.
        #[test]
        fn test_partial_ord() {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1).unwrap();
            assert!(dt1 < dt2);
        }

        /// Test for the `Ord` implementation of `DateTime`.
        #[test]
        fn test_ord() {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1).unwrap();
            assert!(dt1.cmp(&dt2) == std::cmp::Ordering::Less);
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
        fn test_is_within_range() {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1).unwrap();
            let dt3 = dt1.add_days(2).unwrap();
            assert!(dt2.is_within_range(&dt1, &dt3));
            assert!(!dt1.is_within_range(&dt2, &dt3));
        }
    }

    /// Tests related to week, month, and year operations on `DateTime` objects.
    mod calendar_operations_tests {
        use super::*;

        /// Test for getting the start of the week for a `DateTime`.
        #[test]
        fn test_start_of_week() {
            let dt = DateTime::new_with_tz("UTC").unwrap();
            let start_of_week = dt.start_of_week().unwrap();
            assert_eq!(start_of_week.weekday(), Weekday::Monday);
        }

        /// Test for getting the end of the week for a `DateTime`.
        #[test]
        fn test_end_of_week() {
            let dt = DateTime::new_with_tz("UTC").unwrap();
            let end_of_week = dt.end_of_week().unwrap();
            assert_eq!(end_of_week.weekday(), Weekday::Sunday);
        }

        /// Test for getting the start of the month for a `DateTime`.
        #[test]
        fn test_start_of_month() {
            let dt = DateTime::new_with_tz("UTC").unwrap();
            let start_of_month = dt.start_of_month().unwrap();
            assert_eq!(start_of_month.day(), 1);
        }

        /// Test for getting the end of the month for a `DateTime`.
        #[test]
        fn test_end_of_month() {
            let dt = DateTime::new_with_tz("UTC").unwrap();
            let end_of_month = dt.end_of_month().unwrap();
            assert!(
                end_of_month.day() == 28
                    || end_of_month.day() == 29
                    || end_of_month.day() == 30
                    || end_of_month.day() == 31
            );
        }

        /// Test for `end_of_month` method in different scenarios.
        #[test]
        fn test_end_of_month_scenarios() {
            // Test for February in a leap year
            let dt = DateTime::from_components(
                2024,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.end_of_month().unwrap().day(), 29);

            // Test for February in a non-leap year
            let dt = DateTime::from_components(
                2023,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.end_of_month().unwrap().day(), 28);

            // Test for a 30-day month
            let dt = DateTime::from_components(
                2024,
                4,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.end_of_month().unwrap().day(), 30);

            // Test for a 31-day month
            let dt = DateTime::from_components(
                2024,
                5,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.end_of_month().unwrap().day(), 31);
        }

        /// Test for getting the start of the year for a `DateTime`.
        #[test]
        fn test_start_of_year() {
            let dt = DateTime::new_with_tz("UTC").unwrap();
            let start_of_year = dt.start_of_year().unwrap();
            assert_eq!(start_of_year.month() as u8, 1);
            assert_eq!(start_of_year.day(), 1);
        }

        /// Test for getting the end of the year for a `DateTime`.
        #[test]
        fn test_end_of_year() {
            let dt = DateTime::new_with_tz("UTC").unwrap();
            let end_of_year = dt.end_of_year().unwrap();
            assert_eq!(end_of_year.month() as u8, 12);
            assert_eq!(end_of_year.day(), 31);
        }

        /// Test for getting the ordinal day of the year.
        #[test]
        fn test_ordinal_day() {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.ordinal(), 1);
        }

        /// Test for getting the ISO week number.
        #[test]
        fn test_iso_week() {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert!(dt.iso_week() > 0 && dt.iso_week() <= 53);
        }

        /// Test for microsecond precision in `DateTime`.
        #[test]
        fn test_microsecond_precision() {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.microsecond(), 0);
        }

        /// Test for getting the next day from a `DateTime`.
        #[test]
        fn test_next_day() {
            let dt = DateTime::new();
            let next_day = dt.next_day().unwrap();
            assert_eq!(next_day.day(), (dt.day() + 1) % 31);
        }

        /// Test for getting the previous day from a `DateTime`.
        #[test]
        fn test_previous_day() {
            let dt = DateTime::new();
            let previous_day = dt.previous_day().unwrap();
            assert_eq!(
                previous_day.day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
        }
    }

    /// Tests related to updating and converting `DateTime` objects.
    mod update_and_conversion_tests {
        use super::*;

        /// Test for updating a `DateTime` instance.
        #[test]
        fn test_update() {
            let dt = DateTime::new();
            std::thread::sleep(std::time::Duration::from_secs(2)); // Increase the sleep duration to 2 seconds
            let updated_dt = dt.update().unwrap();
            assert!(updated_dt > dt);
        }

        /// Test for converting a `DateTime` instance to a different timezone.
        #[test]
        fn test_convert_to_tz() {
            let dt = DateTime::new_with_tz("EST").unwrap();
            let paris_time = dt.convert_to_tz("CET").unwrap();
            assert_eq!(
                paris_time.offset(),
                UtcOffset::from_hms(1, 0, 0).unwrap()
            );
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
            let dt: DateTime = Default::default();
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
            let result = DateTime::parse("invalid-date-format");
            assert!(matches!(
                result,
                Err(DateTimeError::InvalidFormat)
            ));
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
        fn test_new_with_tz() {
            let dt = DateTime::new_with_tz("EST").unwrap();
            assert_eq!(
                dt.offset(),
                UtcOffset::from_hms(-5, 0, 0).unwrap()
            );
        }

        #[test]
        fn test_new_with_custom_offset() {
            let dt = DateTime::new_with_custom_offset(5, 30).unwrap();
            assert_eq!(
                dt.offset(),
                UtcOffset::from_hms(5, 30, 0).unwrap()
            );
        }

        #[test]
        fn test_from_components() {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                15,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }

        #[test]
        fn test_default() {
            let dt: DateTime = Default::default();
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }
    }

    /// Test suite for date and time manipulation methods.
    mod date_time_manipulation {
        use super::*;

        #[test]
        fn test_add_duration() {
            let dt = DateTime::from_components(
                2024,
                8,
                31,
                12,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            let future_dt = (dt + Duration::days(1)).unwrap();
            assert_eq!(future_dt.day(), 1);
            assert_eq!(future_dt.month() as u8, 9);
        }

        #[test]
        fn test_sub_duration() {
            let dt = DateTime::new();
            let past_dt = (dt - Duration::days(1)).unwrap();
            assert_eq!(
                past_dt.day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
        }

        #[test]
        fn test_add_negative_days() {
            let dt = DateTime::new();
            let result = dt.add_days(-1);
            assert!(result.is_ok());
            assert_eq!(
                result.unwrap().day(),
                if dt.day() == 1 { 31 } else { dt.day() - 1 }
            );
        }

        #[test]
        fn test_update() {
            let dt = DateTime::new();
            std::thread::sleep(std::time::Duration::from_secs(2)); // Increase the sleep duration to 2 seconds
            let updated_dt = dt.update().unwrap();
            assert!(updated_dt > dt);
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
        fn test_convert_to_tz() {
            let dt = DateTime::new_with_tz("EST").unwrap();
            let paris_time = dt.convert_to_tz("CET").unwrap();
            assert_eq!(
                paris_time.offset(),
                UtcOffset::from_hms(1, 0, 0).unwrap()
            );
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
        fn test_parse() {
            let dt = DateTime::parse("2024-08-31T15:00:00Z").unwrap();
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
            assert_eq!(dt.offset(), UtcOffset::UTC);
        }

        #[test]
        fn test_parse_custom_format() {
            let dt = DateTime::parse_custom_format(
                "2024-08-31 15:00:00",
                "[year]-[month]-[day] [hour]:[minute]:[second]",
            )
            .unwrap();
            assert_eq!(dt.year(), 2024);
            assert_eq!(dt.month() as u8, 8);
            assert_eq!(dt.day(), 31);
            assert_eq!(dt.hour(), 15);
            assert_eq!(dt.minute(), 0);
            assert_eq!(dt.second(), 0);
        }

        #[test]
        fn test_format() {
            let dt = DateTime::new();
            let formatted = dt.format("[year]-[month]-[day]").unwrap();
            assert!(formatted.starts_with(&dt.year().to_string()));
        }

        #[test]
        fn test_format_rfc3339() {
            let dt = DateTime::new();
            let rfc3339_format = dt.format_rfc3339().unwrap();
            assert!(rfc3339_format.contains("T"));
        }

        #[test]
        fn test_format_iso8601() {
            let dt = DateTime::new();
            let iso8601_format = dt.format_iso8601().unwrap();
            assert!(iso8601_format.contains("T"));
        }
    }

    /// Test suite for comparisons and hashing.
    mod comparisons_hashing {
        use super::*;
        use std::collections::hash_map::DefaultHasher;

        #[test]
        fn test_partial_ord() {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1).unwrap();
            assert!(dt1 < dt2);
        }

        #[test]
        fn test_ord() {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1).unwrap();
            assert!(dt1.cmp(&dt2) == std::cmp::Ordering::Less);
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
        fn test_is_within_range() {
            let dt1 = DateTime::new();
            let dt2 = dt1.add_days(1).unwrap();
            let dt3 = dt1.add_days(2).unwrap();
            assert!(dt2.is_within_range(&dt1, &dt3));
            assert!(!dt1.is_within_range(&dt2, &dt3));
        }
    }

    /// Test suite for edge cases and other functionalities.
    mod edge_cases {
        use super::*;

        #[test]
        fn test_add_duration_invalid() {
            let dt = DateTime::from_components(
                9999,
                12,
                31,
                23,
                59,
                59,
                UtcOffset::UTC,
            )
            .unwrap();
            let result = dt + Duration::days(1);
            assert!(matches!(result, Err(DateTimeError::InvalidDate)));
        }

        #[test]
        fn test_end_of_month_scenarios() {
            // Test for February in a leap year
            let dt = DateTime::from_components(
                2024,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.end_of_month().unwrap().day(), 29);

            // Test for February in a non-leap year
            let dt = DateTime::from_components(
                2023,
                2,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.end_of_month().unwrap().day(), 28);

            // Test for a 30-day month
            let dt = DateTime::from_components(
                2024,
                4,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.end_of_month().unwrap().day(), 30);

            // Test for a 31-day month
            let dt = DateTime::from_components(
                2024,
                5,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.end_of_month().unwrap().day(), 31);
        }

        #[test]
        fn test_microsecond_precision() {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.microsecond(), 0);
        }

        #[test]
        fn test_ordinal_day() {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert_eq!(dt.ordinal(), 1);
        }

        #[test]
        fn test_iso_week() {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            assert!(dt.iso_week() > 0 && dt.iso_week() <= 53);
        }

        #[test]
        fn test_display() {
            let dt = DateTime::from_components(
                2024,
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
            .unwrap();
            let display_string = format!("{}", dt);
            assert!(display_string.starts_with("2024-01-01T00:00:00"));
        }
    }
}