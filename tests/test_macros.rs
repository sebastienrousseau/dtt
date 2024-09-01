/// Tests for the `dtt` crate, which provides a collection of macros for various date and time operations.
#[cfg(test)]
mod tests {
    use dtt::*;
    use paste::paste;
    use std::collections::HashMap;
    use time::Month;

    /// Tests for basic macros
    mod basic_macros {
        use super::*;

        /// Tests the `dtt_print` macro.
        #[test]
        fn test_dtt_print() {
            dtt_print!("Hello, World!");
        }

        /// Tests the `dtt_vec` macro.
        #[test]
        fn test_dtt_vec() {
            let v = dtt_vec!(1, 2, 3);
            assert_eq!(v, vec![1, 2, 3]);
        }

        /// Tests the `dtt_map` macro.
        #[test]
        fn test_dtt_map() {
            let m = dtt_map!("one" => 1, "two" => 2);
            let mut expected = HashMap::new();
            expected.insert("one", 1);
            expected.insert("two", 2);
            assert_eq!(m, expected);
        }

        /// Tests the `dtt_min` macro.
        #[test]
        fn test_dtt_min() {
            assert_eq!(dtt_min!(10, 20, 30), 10);
        }

        /// Tests the `dtt_max` macro.
        #[test]
        fn test_dtt_max() {
            assert_eq!(dtt_max!(10, 20, 30), 30);
        }

        /// Tests the `dtt_join` macro.
        #[test]
        fn test_dtt_join() {
            let s = dtt_join!("Hello", " ", "World");
            assert_eq!(s, "Hello World");
        }

        /// Tests the `dtt_print_vec` macro.
        #[test]
        fn test_dtt_print_vec() {
            dtt_print_vec!(&[1, 2, 3]);
        }

        /// Tests the `dtt_create_vec` macro.
        #[test]
        fn test_dtt_create_vec() {
            let v = dtt_create_vec![1, 2, 3];
            assert_eq!(v, vec![1, 2, 3]);
        }
    }

    /// Tests for datetime parsing and formatting macros
    mod datetime_macros {
        use super::*;

        /// Tests the `dtt_now` macro, ensuring it returns a valid formatted string.
        #[test]
        fn test_dtt_now() {
            let now = dtt_now!();
            let formatted_now = now
                .format("%Y-%m-%d %H:%M:%S")
                .expect("Failed to format DateTime");
            assert!(!formatted_now.is_empty());
        }

        /// Tests the `dtt_parse` macro with a valid date string.
        #[test]
        fn test_dtt_parse() {
            let input = "2022-01-01T12:00:00+01:00";
            match dtt_parse!(input) {
                Ok(date) => {
                    assert_eq!(date.year(), 2022);
                    assert_eq!(date.month(), Month::January);
                    assert_eq!(date.day(), 1);
                    assert_eq!(date.hour(), 12);
                    assert_eq!(date.minute(), 0);
                    assert_eq!(date.second(), 0);
                }
                Err(err) => panic!("Parsing failed: {}", err),
            }
        }

        /// Tests the `dtt_new_with_tz` macro, ensuring the correct timezone offset.
        #[test]
        fn test_dtt_new_with_tz() {
            let tz = "CET";
            let dt = dtt_new_with_tz!(tz);
            assert_eq!(dt.offset().to_string(), "+01:00:00");
        }

        /// Tests the `dtt_add_days` macro, adding days to a date.
        #[test]
        fn test_dtt_add_days() {
            let dt = dtt_parse!("2023-01-01T12:00:00+00:00")
                .expect("Failed to parse DateTime");
            let future_date =
                dtt_add_days!(dt, 5).expect("Failed to add days");
            assert_eq!(future_date.day(), 6);
        }

        /// Tests the `dtt_sub_days` macro, subtracting days from a date.
        #[test]
        fn test_dtt_sub_days() {
            let dt = dtt_parse!("2023-01-06T12:00:00+00:00")
                .expect("Failed to parse DateTime");
            let past_date =
                dtt_sub_days!(dt, 5).expect("Failed to subtract days");
            assert_eq!(past_date.day(), 1);
        }

        /// Tests the `dtt_format` macro with a complex date format.
        #[test]
        fn test_dtt_format() {
            let dt = dtt_parse!("2023-01-01T12:00:00+00:00")
                .expect("Failed to parse DateTime");
            let formatted = dtt_format!(
                dt,
                "{year}-{month}-{day}T{hour}:{minute}:{second}.{microsecond}{offset_sign}{offset_hour}:{offset_minute}"
            );
            let expected_output = "2023-01-01T12:00:00.000000+00:00";
            assert_eq!(formatted, expected_output);
        }

        /// Tests the `dtt_format` macro with a more complex format.
        #[test]
        fn test_dtt_format_complex() {
            let dt =
                dtt_parse!("2023-06-15T18:30:45.123456+00:00").unwrap();
            let formatted = dtt_format!(
                dt,
                "Year: {year}, Month: {month}, Day: {day}, Hour: {hour}, Minute: {minute}, Second: {second}, Microsecond: {microsecond}, Offset: {offset_sign}{offset_hour}:{offset_minute}"
            );
            assert_eq!(formatted, "Year: 2023, Month: 06, Day: 15, Hour: 18, Minute: 30, Second: 45, Microsecond: 123456, Offset: +00:00");
        }

        /// Tests the `dtt_clone` macro for cloning a datetime.
        #[test]
        fn test_dtt_clone() {
            let dt = dtt_parse!("2023-01-01T12:00:00+00:00").unwrap();
            let cloned = dtt_clone!(dt);
            assert_eq!(dt.datetime, cloned.datetime);
            assert_eq!(dt.offset, cloned.offset);

            assert_eq!(dt.year(), cloned.year());
            assert_eq!(dt.month(), cloned.month());
            assert_eq!(dt.day(), cloned.day());
            assert_eq!(dt.hour(), cloned.hour());
            assert_eq!(dt.minute(), cloned.minute());
            assert_eq!(dt.second(), cloned.second());
        }
    }

    /// Tests for date and time difference macros
    mod diff_macros {
        use super::*;

        /// Tests the `dtt_diff_seconds` macro for correct second difference.
        #[test]
        fn test_dtt_diff_seconds() {
            let dt1 = "1640995200"; // Unix timestamp for 2023-01-01T12:00:00Z
            let dt2 = "1640995230"; // Unix timestamp for 2023-01-01T12:00:30Z
            let seconds_difference = dtt_diff_seconds!(dt1, dt2);
            assert_eq!(seconds_difference, "30");
        }

        /// Tests the `dtt_diff_days` macro for correct day difference.
        #[test]
        fn test_dtt_diff_days() {
            let dt1 = "1640995200"; // Unix timestamp for 2023-01-01T00:00:00Z
            let dt2 = "1641081600"; // Unix timestamp for 2023-01-02T00:00:00Z
            let days_difference = dtt_diff_days!(dt1, dt2);
            assert_eq!(days_difference, 1);
        }

        /// Tests the `dtt_diff_seconds` macro with invalid input, expecting an error.
        #[test]
        fn test_dtt_diff_seconds_error() {
            let dt1 = "invalid";
            let dt2 = "1640995230";
            let result = dtt_diff_seconds!(dt1, dt2);
            assert_eq!(result, "Error: Invalid input");
        }

        /// Tests the `dtt_diff_days` macro with invalid input, expecting a panic.
        #[test]
        #[should_panic(expected = "Error: Invalid input")]
        fn test_dtt_diff_days_error() {
            let dt1 = "invalid";
            let dt2 = "1641081600";
            let _ = dtt_diff_days!(dt1, dt2);
        }
    }

    /// Tests for validation macros
    mod validation_macros {
        use super::*;

        /// Tests the `dtt_is_valid_function` macro for day and month validation.
        #[test]
        fn test_dtt_is_valid_function() {
            dtt_is_valid_function!(day, u8);
            assert!(is_valid_day("15"));
            assert!(!is_valid_day("32"));

            dtt_is_valid_function!(month, u8);
            assert!(is_valid_month("12"));
            assert!(!is_valid_month("13"));
        }

        /// Tests the `is_valid!` macro for hour and minute validation.
        #[test]
        fn test_is_valid() {
            is_valid!(is_valid_hour, u8);
            assert!(is_valid_hour("23"));
            assert!(!is_valid_hour("24"));

            is_valid!(is_valid_minute, u8);
            assert!(is_valid_minute("59"));
            assert!(!is_valid_minute("60"));
        }
    }

    /// Tests for assertion macros
    mod assertion_macros {
        use super::*;

        /// Tests the `dtt_assert` macro with a valid date.
        #[test]
        fn test_dtt_assert() {
            let date_string = "2020-02-29"; // Leap year date to check the validity
            let valid_date = dtt_parse!(date_string).is_ok();
            dtt_assert!(valid_date, "The date must be valid.");
        }

        /// Tests the `dtt_assert` macro with an invalid date, expecting a panic.
        #[test]
        #[should_panic(expected = "The date must be valid.")]
        fn test_dtt_assert_fail() {
            let date_string = "2023-02-29"; // Invalid date for non-leap year
            let valid_date = dtt_parse!(date_string).is_ok();
            dtt_assert!(valid_date, "The date must be valid.");
        }
    }
}
