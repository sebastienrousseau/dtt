#[cfg(test)]
mod tests {
    use dtt::*;
    use std::collections::HashMap;
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
        expected.insert("one", 1);
        expected.insert("two", 2);
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
}
