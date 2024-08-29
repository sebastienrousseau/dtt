// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {
    use dtt::is_valid;
    use dtt::DateTime;
    use dtt::*;

    #[test]
    // Demonstrates the use of the `dtt_print!` macro
    fn test_dtt_print() {
        dtt_print!("Hello, World!");
    }

    #[test]
    // Demonstrates the use of the `dtt_vec!` macro
    fn test_dtt_vec() {
        let v = dtt_vec!(1, 2, 3);
        assert_eq!(v, &[1, 2, 3]);
    }

    #[test]
    // Demonstrates the use of the `dtt_map!` macro
    fn test_dtt_map() {
        let m = dtt_map!("one" => 1, "two" => 2);
        let mut expected = std::collections::HashMap::new();
        expected.insert("one", 1);
        expected.insert("two", 2);
        assert_eq!(m, expected);
    }

    #[test]
    // Demonstrates the use of the `dtt_assert!` macro
    fn test_dtt_assert() {
        let day = 29;
        let valid_date = DateTime::is_valid_day(&day.to_string());
        dtt_assert!(valid_date, "The day must be within the valid range for the given month and year.");
    }

    #[test]
    #[should_panic(
        expected = "The day must be within the valid range for the given month and year."
    )]
    fn test_dtt_assert_fail() {
        let day = 32;
        let valid_date = DateTime::is_valid_day(&day.to_string());
        dtt_assert!(valid_date, "The day must be within the valid range for the given month and year.");
    }

    #[test]
    fn test_dtt_min() {
        assert_eq!(dtt_min!(10, 20, 30), 10);
    }

    #[test]
    // Demonstrates the use of the `dtt_max!` macro
    fn test_dtt_max() {
        assert_eq!(dtt_max!(10, 20, 30), 30);
    }

    // #[test]
    // // Demonstrates the use of the `dtt_split!` macro
    // fn test_dtt_split() {
    //     let v = dtt_split!("Hello World");
    //     assert_eq!(v, vec!["Hello", "World"]);
    // }

    #[test]
    fn test_dtt_join() {
        let s = dtt_join!("Hello", " ", "World");
        assert_eq!(s, "Hello World");
    }

    #[test]
    // Demonstrates the use of the `dtt_print_vec!` macro
    fn test_dtt_print_vec() {
        dtt_print_vec!(&[1, 2, 3]);
    }

    #[test]
    // Demonstrates the use of the `is_valid!` macro
    fn test_is_valid() {
        let input = "31";
        is_valid!(day, String);
        let result = day(input);
        assert!(result);
    }

    #[test]
    // Demonstrates the use of the `is_valid_day!` macro
    fn test_is_valid_day() {
        assert!(DateTime::is_valid_day("31"));
        assert!(!DateTime::is_valid_day("32"));
        assert!(!DateTime::is_valid_day("a"));
    }

    #[test]
    // Demonstrates the use of the `is_valid_hour!` macro
    fn test_is_valid_hour() {
        assert!(DateTime::is_valid_hour("23"));
        assert!(!DateTime::is_valid_hour("24"));
        assert!(!DateTime::is_valid_hour("a"));
    }

    #[test]
    fn test_is_valid_minute() {
        assert!(DateTime::is_valid_minute("59"));
        assert!(!DateTime::is_valid_minute("60"));
        assert!(!DateTime::is_valid_minute("c"));
    }

    #[test]
    // Demonstrates the use of the `is_valid_month!` macro
    fn test_is_valid_month() {
        assert!(DateTime::is_valid_month("12"));
        assert!(!DateTime::is_valid_month("13"));
        assert!(!DateTime::is_valid_month("d"));
    }

    #[test]
    // Demonstrates the use of the `is_valid_second!` macro
    fn test_is_valid_second() {
        assert!(DateTime::is_valid_second("59"));
        assert!(!DateTime::is_valid_second("60"));
        assert!(!DateTime::is_valid_second("e"));
    }

    #[test]
    // Demonstrates the use of the `is_valid_microsecond!` macro
    fn test_is_valid_microsecond() {
        assert!(DateTime::is_valid_microsecond("999999"));
        assert!(!DateTime::is_valid_microsecond("1000000"));
        assert!(!DateTime::is_valid_microsecond("b"));
    }

    #[test]
    // Demonstrates the use of the `is_valid_ordinal!` macro
    fn test_is_valid_ordinal() {
        assert!(DateTime::is_valid_ordinal("366"));
        assert!(!DateTime::is_valid_ordinal("367"));
        assert!(!DateTime::is_valid_ordinal("e"));
    }

    #[test]
    // Demonstrates the use of the `is_valid_time!` macro
    fn test_is_valid_time() {
        assert!(DateTime::is_valid_time("23:59:59"));
        assert!(!DateTime::is_valid_time("24:00:00"));
        assert!(!DateTime::is_valid_time("g"));
    }

    #[test]
    // Demonstrates the use of the `is_valid_iso_8601!` macro
    fn test_is_valid_iso_8601() {
        assert!(DateTime::is_valid_iso_8601("2023-05-11T17:30:00Z"));
        assert!(DateTime::is_valid_iso_8601(
            "2023-05-11T17:30:00+01:00"
        ));
        assert!(DateTime::is_valid_iso_8601(
            "2023-05-11T17:30:00.123456Z"
        ));
        assert!(DateTime::is_valid_iso_8601(
            "2023-05-11T17:30:00.123456+01:00"
        ));
    }

    // Define validation function for hour
    is_valid!(hour, String);

    #[test]
    fn test_is_valid_hour_macro() {
        assert!(hour("23")); // Valid hour
        assert!(!hour("24")); // Invalid hour
        assert!(!hour("not_a_number")); // Invalid input
    }

    // Define validation function for minute
    is_valid!(minute, String);

    #[test]
    fn test_is_valid_minute_macro() {
        assert!(minute("59")); // Valid minute
        assert!(!minute("60")); // Invalid minute
        assert!(!minute("not_a_number")); // Invalid input
    }

    // Define validation function for month
    is_valid!(month, String);

    #[test]
    fn test_is_valid_month_macro() {
        assert!(month("12")); // Valid month
        assert!(!month("13")); // Invalid month
        assert!(!month("not_a_number")); // Invalid input
    }

    // Define validation function for second
    is_valid!(second, String);

    #[test]
    fn test_is_valid_second_macro() {
        assert!(second("59")); // Valid second
        assert!(!second("60")); // Invalid second
        assert!(!second("not_a_number")); // Invalid input
    }

    // Define validation function for microsecond
    is_valid!(microsecond, String);
    #[test]
    fn test_is_valid_microsecond_macro() {
        assert!(microsecond("999999")); // Valid microsecond
        assert!(!microsecond("1000000")); // Invalid microsecond
        assert!(!microsecond("not_a_number")); // Invalid input
    }

    // Define validation function for ordinal
    is_valid!(ordinal, String);

    #[test]
    fn test_is_valid_ordinal_macro() {
        assert!(ordinal("366")); // Valid ordinal
        assert!(!ordinal("367")); // Invalid ordinal
        assert!(!ordinal("not_a_number")); // Invalid input
    }

    // Define validation function for time
    is_valid!(time, String);

    #[test]
    fn test_is_valid_time_macro() {
        assert!(time("23:59:59")); // Valid time
        assert!(!time("24:00:00")); // Invalid time
        assert!(!time("not_a_number")); // Invalid input
    }

    // Define validation function for iso_8601
    is_valid!(iso_8601, String);

    #[test]
    fn test_is_valid_iso_8601_macro() {
        assert!(iso_8601("2023-05-11T17:30:00Z")); // Valid iso_8601
        assert!(iso_8601("2023-05-11T17:30:00+01:00")); // Valid iso_8601
        assert!(iso_8601("2023-05-11T17:30:00.123456Z")); // Valid iso_8601
        assert!(iso_8601("2023-05-11T17:30:00.123456+01:00")); // Valid iso_8601
        assert!(!iso_8601("not_a_number")); // Invalid input
    }

    is_valid!(unknown, String);

    #[test]
    fn test_unknown() {
        // Since the "unknown" identifier doesn't match any of the known identifiers,
        // it should always return false.
        assert!(!unknown("anything"));
    }

    #[test]
    fn test_parse_error() {
        // Pass an input that cannot be parsed as a string to trigger the Err(_) branch.
        let bad_input = vec![0, 159, 146, 150]; // Invalid UTF-8 sequence
        let bad_string =
            String::from_utf8(bad_input).unwrap_err().to_string();
        assert!(!DateTime::is_valid_day(&bad_string));
        assert!(!DateTime::is_valid_hour(&bad_string));
        assert!(!DateTime::is_valid_minute(&bad_string));
        assert!(!DateTime::is_valid_month(&bad_string));
        assert!(!DateTime::is_valid_second(&bad_string));
        assert!(!DateTime::is_valid_microsecond(&bad_string));
        assert!(!DateTime::is_valid_ordinal(&bad_string));
        assert!(!DateTime::is_valid_time(&bad_string));
        assert!(!DateTime::is_valid_iso_8601(&bad_string));
        assert!(!unknown(&bad_string));
    }

    is_valid!(integer, i32);

    #[test]
    fn test_parse_error_int() {
        // Pass an input that cannot be parsed as an i32 to trigger the Err(_) branch.
        assert!(!integer("not_a_number"));
    }

    #[test]
    fn test_dtt_now() {
        use dtt::dtt_now;
        let now = dtt_now!();
        println!("Current date and time: {}", now.iso_8601);
    }

    #[test]
    fn test_dtt_parse() {
        let input = "2022-01-01T12:00:00+01:00";
        match dtt_parse!(input) {
            Ok(date) => {
                assert_eq!(date.year, 2022);
                assert_eq!(date.month.parse::<u8>().unwrap(), 1);
                assert_eq!(date.day, 1);
                assert_eq!(date.hour, 12);
                assert_eq!(date.minute, 0);
                assert_eq!(date.second, 0);
            }
            Err(err) => panic!("Parsing failed: {}", err),
        }
    }

    #[test]
    fn test_dtt_add_days() {
        let dt = DateTime::parse("2023-01-01T12:00:00+00:00").unwrap();
        let future_date = dtt_add_days!(dt, 5);
        assert_eq!(future_date.day, 30);
    }

    #[test]
    fn test_dtt_diff_hours() {
        let dt1 = "2023-01-01T12:00:00+00:00";
        let dt2 = "2023-01-02T12:00:00+00:00";
        let hours_difference = dtt_diff_hours!(dt1, dt2);
        assert!(hours_difference > 23.to_string());
    }

    #[test]
    fn test_dtt_diff_minutes() {
        let dt1 = "2023-01-01T12:00:00+00:00";
        let dt2 = "2023-01-02T12:00:00+00:00";
        let minutes_difference = dtt_diff_minutes!(dt1, dt2);
        assert!(minutes_difference > 60.to_string());
    }

    #[test]
    fn test_dtt_diff_seconds() {
        let dt1 = "1640995200"; // Unix timestamp for 2023-01-01T12:00:00Z
        let dt2 = "1640995230"; // Unix timestamp for 2023-01-01T12:00:30Z
        let seconds_difference = dtt_diff_seconds!(dt1, dt2);
        assert_eq!(seconds_difference, "30");
    }

    #[test]
    fn test_dtt_diff_days() {
        let dt1 = "1640995200"; // Unix timestamp for 2023-01-01T00:00:00Z
        let dt2 = "1641081600"; // Unix timestamp for 2023-01-02T00:00:00Z
        let days_difference = dtt_diff_days!(dt1, dt2);
        assert_eq!(days_difference, 1);
    }
}
