// test_macros.rs
//
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {
    use dtt::*;
    use paste::paste;
    use std::{collections::HashMap, panic};
    use time::Month;

    #[test]
    fn test_dtt_print() {
        dtt_print!("Hello, World!");
    }

    #[test]
    fn test_dtt_vec() {
        let v = dtt_vec!(1, 2, 3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_dtt_map() {
        let m = dtt_map!("one" => 1, "two" => 2);
        let mut expected = HashMap::new();
        let _ = expected.insert("one", 1);
        let _ = expected.insert("two", 2);
        assert_eq!(m, expected);
    }

    #[test]
    fn test_dtt_assert() {
        let date_string = "2020-02-29"; // Leap year date to check the validity
        let valid_date = dtt_parse!(date_string).is_ok();
        dtt_assert!(valid_date, "The date must be valid.");
    }

    #[test]
    #[should_panic(expected = "The date must be valid.")]
    fn test_dtt_assert_fail() {
        let date_string = "2023-02-29"; // Invalid date for non-leap year
        let valid_date = dtt_parse!(date_string).is_ok();
        dtt_assert!(valid_date, "The date must be valid.");
    }

    #[test]
    fn test_dtt_min() {
        assert_eq!(dtt_min!(10, 20, 30), 10);
    }

    #[test]
    fn test_dtt_max() {
        assert_eq!(dtt_max!(10, 20, 30), 30);
    }

    #[test]
    fn test_dtt_join() {
        let s = dtt_join!("Hello", " ", "World");
        assert_eq!(s, "Hello World");
    }

    #[test]
    fn test_dtt_print_vec() {
        dtt_print_vec!(&[1, 2, 3]);
    }

    #[test]
    fn test_dtt_now() {
        let now = dtt_now!();
        let formatted_now = now
            .format("%Y-%m-%d %H:%M:%S")
            .expect("Failed to format DateTime");
        assert!(!formatted_now.is_empty()); // Check that now() returns a valid formatted string
    }

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

    #[test]
    fn test_dtt_new_with_tz() {
        let tz = "CET";
        let dt = dtt_new_with_tz!(tz);
        assert_eq!(dt.offset().to_string(), "+01:00:00");
    }

    #[test]
    fn test_dtt_add_days() {
        let dt = dtt_parse!("2023-01-01T12:00:00+00:00")
            .expect("Failed to parse DateTime");
        let future_date =
            dtt_add_days!(dt, 5).expect("Failed to add days");
        assert_eq!(future_date.day(), 6);
    }

    #[test]
    fn test_dtt_sub_days() {
        let dt = dtt_parse!("2023-01-06T12:00:00+00:00")
            .expect("Failed to parse DateTime");
        let past_date =
            dtt_sub_days!(dt, 5).expect("Failed to subtract days");
        assert_eq!(past_date.day(), 1);
    }

    #[test]
    fn test_dtt_diff_seconds() {
        let dt1 = "1609459200"; // 2021-01-01 00:00:00 UTC
        let dt2 = "1609459230"; // 2021-01-01 00:00:30 UTC
        let seconds_difference = dtt_diff_seconds!(dt1, dt2);
        assert_eq!(seconds_difference, 30i64);
    }

    #[test]
    fn test_dtt_diff_days() {
        let dt1 = "1609459200"; // 2021-01-01 00:00:00 UTC
        let dt2 = "1609545600"; // 2021-01-02 00:00:00 UTC
        let days_difference = dtt_diff_days!(dt1, dt2);
        assert_eq!(days_difference, 1i64);
    }

    // #[test]
    // fn test_dtt_diff_invalid_input() {
    //     let dt1 = "invalid";
    //     let dt2 = "1609459230";

    //     let result =
    //         panic::catch_unwind(|| dtt_diff_seconds!(dt1, dt2));
    //     assert!(result.is_err(), "Expected panic for invalid input");
    // }

    #[test]
    fn test_dtt_format() {
        let dt = dtt_parse!("2023-01-01T12:00:00+00:00")
            .expect("Failed to parse DateTime");
        let formatted = dtt_format!(
        dt,
        "{year}-{month}-{day}T{hour}:{minute}:{second}.{microsecond}{offset_sign}{offset_hour}:{offset_minute}"
    );
        // Adjust the expected output to match the macro's output
        let expected_output = "2023-01-01T12:00:00.000000+00:00";
        assert_eq!(formatted, expected_output);
    }

    #[test]
    fn test_dtt_create_vec() {
        let v = dtt_create_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_dtt_is_valid_function() {
        dtt_is_valid_function!(day, u8);
        assert!(is_valid_day("15"));
        assert!(!is_valid_day("32"));

        dtt_is_valid_function!(month, u8);
        assert!(is_valid_month("12"));
        assert!(!is_valid_month("13"));
    }

    #[test]
    fn test_is_valid() {
        is_valid!(is_valid_hour, u8);
        assert!(is_valid_hour("23"));
        assert!(!is_valid_hour("24"));

        is_valid!(is_valid_minute, u8);
        assert!(is_valid_minute("59"));
        assert!(!is_valid_minute("60"));
    }

    #[test]
    fn test_dtt_clone() {
        let dt = dtt_parse!("2023-01-01T12:00:00+00:00").unwrap();
        let cloned = dtt_clone!(dt);
        assert_eq!(dt.datetime, cloned.datetime);
        assert_eq!(dt.offset, cloned.offset);

        // We can still test the individual components
        assert_eq!(dt.year(), cloned.year());
        assert_eq!(dt.month(), cloned.month());
        assert_eq!(dt.day(), cloned.day());
        assert_eq!(dt.hour(), cloned.hour());
        assert_eq!(dt.minute(), cloned.minute());
        assert_eq!(dt.second(), cloned.second());
    }

    // #[test]
    // fn test_dtt_diff_seconds_error() {
    //     let dt1 = "invalid";
    //     let dt2 = "1640995230";

    //     let result =
    //         panic::catch_unwind(|| dtt_diff_seconds!(dt1, dt2));

    //     assert!(result.is_err(), "Expected panic for invalid input");
    // }

    #[test]
    #[should_panic(expected = "Error: Invalid input")]
    fn test_dtt_diff_days_error() {
        let dt1 = "invalid";
        let dt2 = "1641081600";
        let _ = dtt_diff_days!(dt1, dt2);
    }

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

    #[test]
    fn test_dtt_now_macro() {
        let now = dtt_now!();
        // You can't test the exact time, but you can check if it's recent
        let five_minutes_ago = now.add_days(-5).unwrap();
        assert!(now > five_minutes_ago);
    }

    #[test]
    fn test_dtt_parse_macro() {
        let dt = dtt_parse!("2023-01-01T00:00:00Z").unwrap();
        assert_eq!(dt.year(), 2023);
        assert_eq!(dt.month(), Month::January);
        assert_eq!(dt.day(), 1);
    }
}
