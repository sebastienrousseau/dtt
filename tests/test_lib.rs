// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]

mod tests {
    use dtt::DateTime;
    use regex::Regex;
    use std::str::FromStr;
    use time::UtcOffset;
    use time::{Duration, OffsetDateTime};

    #[test]
    // Test the `new` function
    fn test_new() {
        let date = DateTime::new();
        assert!(!date.now.is_empty());
        assert!(date.day <= 31);
        assert!(date.hour <= 23);
        assert!(!date.iso_8601.is_empty());
        assert!(date.iso_week > 0 && date.iso_week <= 53);
        assert!(date.minute <= 59);
        assert!(!date.month.is_empty());
        assert!(!date.offset.is_empty());
        assert!(date.ordinal > 0 && date.ordinal <= 366);
        assert!(date.second <= 59);
        assert!(!date.time.is_empty());
        assert!(!date.weekday.is_empty());
        assert!(date.year > 0);
    }
    #[test]
    // Test the `is_valid_day` function
    fn test_is_valid_day() {
        assert!(!DateTime::is_valid_day("0"));
        assert!(DateTime::is_valid_day("31"));
    }
    #[test]
    // Test the `is_valid_hour` function
    fn test_is_valid_hour() {
        assert!(DateTime::is_valid_hour("23:59"));
        assert!(!DateTime::is_valid_hour("24:00"));
        assert!(!DateTime::is_valid_hour("g"));
    }

    #[test]
    // Test the `is_valid_iso_8601` function
    fn test_is_valid_iso_8601() {
        assert!(DateTime::is_valid_iso_8601("2022-06-25T17:30:00Z"));
        assert!(DateTime::is_valid_iso_8601(
            "2022-06-25T17:30:00+01:00"
        ));
        assert!(DateTime::is_valid_iso_8601(
            "2022-06-25T17:30:00.123456Z"
        ));
        assert!(DateTime::is_valid_iso_8601(
            "2022-06-25T17:30:00.123456+01:00"
        ));
    }

    #[test]
    // Test the reverse of `is_valid_iso_8601` function
    fn test_invalid_iso_8601() {
        assert!(!DateTime::is_valid_iso_8601("2022-06-25T17:30:00"));
        assert!(!DateTime::is_valid_iso_8601("2022-06-25 17:30:00Z"));
        assert!(!DateTime::is_valid_iso_8601(
            "2022-06-25T17:30:00+25:00"
        ));
        assert!(!DateTime::is_valid_iso_8601(
            "2022-06-25T17:30:00+01:61"
        ));
    }

    #[test]
    // Test the `is_valid_iso_week` function
    fn test_is_valid_iso_week() {
        assert!(DateTime::is_valid_iso_week("1"));
        assert!(DateTime::is_valid_iso_week("53"));
        assert!(!DateTime::is_valid_iso_week("0"));
        assert!(!DateTime::is_valid_iso_week("54"));
    }

    #[test]
    // Test the reverse of `is_valid_iso_week` function
    fn test_invalid_iso_week() {
        assert!(!DateTime::is_valid_iso_week("0"));
        assert!(!DateTime::is_valid_iso_week("54"));
    }

    #[test]
    // Test the month validation
    fn test_month_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601(
            "2022-13-25T17:30:00.1234567Z"
        ));
        assert!(!DateTime::is_valid_iso_8601(
            "2022-00-25T17:30:00.1234567Z"
        ));
    }

    #[test]
    // Test the day validation
    fn test_day_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601(
            "2022-12-32T17:30:00.1234567Z"
        ));
        assert!(!DateTime::is_valid_iso_8601(
            "2022-12-00T17:30:00.1234567Z"
        ));
    }

    #[test]
    // Test the hour validation
    fn test_hour_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601(
            "2022-12-31T24:30:00.1234567Z"
        ));
        assert!(!DateTime::is_valid_iso_8601(
            "2022-12-31T-1:30:00.1234567Z"
        ));
    }

    #[test]
    // Test the minute validation
    fn test_minute_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601(
            "2022-12-31T23:60:00.1234567Z"
        ));
        assert!(!DateTime::is_valid_iso_8601(
            "2022-12-31T23:-1:00.1234567Z"
        ));
    }

    #[test]
    // Test the second validation
    fn test_second_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601(
            "2022-12-31T23:59:60.1234567Z"
        ));
        assert!(!DateTime::is_valid_iso_8601(
            "2022-12-31T23:59:-1.1234567Z"
        ));
    }
    #[test]
    // Test the timezone validation
    fn test_tz_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601(
            "2022-06-25T17:30:00.123456Z+25:00"
        ));
        assert!(!DateTime::is_valid_iso_8601(
            "2022-06-25T17:30:00.123456Z+24:61"
        ));
        assert!(!DateTime::is_valid_iso_8601(
            "2022-06-25T17:30:00.123456Z+99:59"
        ));
    }
    #[test]
    fn test_update_with_timezone() {
        let mut date = DateTime::new_with_tz("BST");
        assert!(date.is_ok());

        let date_before_update = date.as_ref().unwrap().time.clone();

        // Call the `update` function to update the date and time
        let _ = date.as_mut().unwrap().update();

        let date_after_update = date.as_mut().unwrap().time.clone();
        assert_ne!(date_before_update, date_after_update);

        let re = Regex::new(r"([+-])(\d{2}):(\d{2})").unwrap();
        let binding = date.as_mut().unwrap();
        let captures = re.captures(&binding.offset).unwrap();
        let hours = captures[2].parse::<i64>().unwrap();
        let minutes = captures[3].parse::<i64>().unwrap();

        let actual_offset = UtcOffset::from_hms(1, 0, 0);
        let expected_offset = UtcOffset::from_hms(
            hours.try_into().unwrap(),
            minutes.try_into().unwrap(),
            0,
        );
        let erroneous_offset = UtcOffset::from_hms(3, 0, 0);
        assert_eq!(expected_offset.unwrap(), actual_offset.unwrap());
        assert_ne!(expected_offset.unwrap(), erroneous_offset.unwrap());
    }

    #[test]
    fn test_update() {
        let mut date = DateTime::new();
        let time_before_update = DateTime {
            day: date.day,
            hour: date.hour,
            iso_week: date.iso_week,
            minute: date.minute,
            month: date.month.clone(),
            offset: date.offset.clone(),
            ordinal: date.ordinal,
            second: date.second,
            tz: date.tz.clone(),
            weekday: date.weekday.clone(),
            year: date.year,
            ..Default::default()
        };
        let _ = date.update();
        assert_eq!(
            time_before_update,
            DateTime {
                day: date.day,
                hour: date.hour,
                iso_week: date.iso_week,
                minute: date.minute,
                month: date.month.clone(),
                offset: date.offset.clone(),
                ordinal: date.ordinal,
                second: date.second,
                tz: date.tz.clone(),
                weekday: date.weekday.clone(),
                year: date.year,
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_update_date() {
        let mut date = DateTime::new();
        let time_before_update = DateTime {
            day: date.day,
            hour: date.hour,
            iso_week: date.iso_week,
            minute: date.minute,
            month: date.month.clone(),
            offset: date.offset.clone(),
            ordinal: date.ordinal,
            second: date.second,
            tz: date.tz.clone(),
            weekday: date.weekday.clone(),
            year: date.year,
            ..Default::default()
        };
        let _ = date.update();
        assert_eq!(
            time_before_update,
            DateTime {
                day: date.day,
                hour: date.hour,
                iso_week: date.iso_week,
                minute: date.minute,
                month: date.month.clone(),
                offset: date.offset.clone(),
                ordinal: date.ordinal,
                second: date.second,
                tz: date.tz.clone(),
                weekday: date.weekday.clone(),
                year: date.year,
                ..Default::default()
            }
        );
    }

    #[test]
    // Test the `update_day` function
    fn test_update_day() {
        let mut date = DateTime::new();
        let day_before_update = date.day;
        let _ = date.update();
        let day_after_update = date.day;
        assert_eq!(day_before_update, day_after_update);
    }

    #[test]
    // Test the `update_hour` function
    fn test_update_hour() {
        let mut date = DateTime::new();
        let hour_before_update = date.hour;
        let _ = date.update();
        let hour_after_update = date.hour;
        assert_eq!(hour_before_update, hour_after_update);
    }

    #[test]
    // Test the `update_iso_8601` function
    fn test_update_iso_8601() {
        let date = DateTime::new();
        let iso_8601_before_update = date.iso_8601.clone();
        assert_eq!(iso_8601_before_update, date.iso_8601);
    }

    #[test]
    // Test the `update_iso_week` function
    fn test_update_iso_week() {
        let date = DateTime::new();
        let iso_week_before_update = date.iso_week;
        assert_eq!(iso_week_before_update, date.iso_week);
        assert!(date.iso_week <= 53);
    }

    #[test]
    // Test the `update_minute` function
    fn test_update_minute() {
        let date = DateTime::new();
        let minute_before_update = date.minute;
        assert_eq!(minute_before_update, date.minute);
        assert!(date.minute <= 59);
    }

    #[test]
    // Test the `update_month` function
    fn test_update_month() {
        let date = DateTime::new();
        let month_before_update = date.month.clone();
        assert_eq!(month_before_update, date.month);
    }

    #[test]
    // Test the `update_offset` function
    fn test_update_offset() {
        let date = DateTime::new();
        let offset_before_update = date.offset.clone();
        assert_eq!(offset_before_update, date.offset);
    }

    #[test]
    // Test the `update_ordinal` function
    fn test_update_ordinal() {
        let date = DateTime::new();
        let ordinal_before_update = date.ordinal;
        assert_eq!(ordinal_before_update, date.ordinal);
        assert!(date.ordinal > 0 && date.ordinal <= 366);
    }
    #[test]
    fn test_update_second() {
        let date = DateTime::new();
        let second_before_update = date.second;
        assert_eq!(second_before_update, date.second);
        assert!(date.second <= 59);
    }
    #[test]
    fn test_update_time() {
        let date = DateTime::new();
        let time_before_update = date.time.clone();
        assert_eq!(time_before_update, date.time);
    }

    #[test]
    // Test the `update_weekday` function
    fn test_update_weekday() {
        let date = DateTime::new();
        let weekday_before_update = date.weekday.clone();
        assert_eq!(weekday_before_update, date.weekday);
    }
    #[test]
    // Test the `update_year` function
    fn test_update_year() {
        let date = DateTime::new();
        let year_before_update = date.year;
        assert_eq!(year_before_update, date.year);
        assert!(date.year > 0);
    }
    #[test]
    // Test the `new_with_tz` function
    fn test_new_with_tz_utc() {
        let date_time = DateTime::new_with_tz("UTC");
        let offset = UtcOffset::UTC;

        let now_utc = OffsetDateTime::now_utc();
        let (hours, minutes, _) = offset.as_hms();
        let total_seconds =
            (hours as i16 * 3600) + (minutes as i16 * 60);
        let expected_date_time =
            now_utc + Duration::seconds(total_seconds as i64);

        assert_eq!(
            date_time.as_ref().unwrap().hour,
            expected_date_time.hour()
        );
        assert_eq!(
            date_time.as_ref().unwrap().minute,
            expected_date_time.minute()
        );
        assert_eq!(
            date_time.as_ref().unwrap().offset,
            expected_date_time.offset().to_string()
        );
    }
    #[test]
    // Test the `new_with_tz_custom` function
    fn test_new_with_tz_custom() {
        let date_time = DateTime::new_with_tz("Custom");
        assert!(date_time.is_err());
    }
    #[test]
    // Test the `new_with_tz_to_paris` function
    fn test_new_with_tz_to_paris() {
        let date = DateTime::new_with_tz("Europe/Paris");
        assert!(date.is_err());
    }

    #[test]
    // Test the `display_format` function
    fn test_display_format() {
        let date_time = DateTime::new();
        let formatted = format!("{date_time}");

        assert!(formatted.starts_with("Year:"));
        assert!(formatted.contains("Month:"));
        assert!(formatted.contains("Day:"));
        assert!(formatted.contains("Weekday:"));
        assert!(formatted.contains("Hour:"));
        assert!(formatted.contains("Minute:"));
        assert!(formatted.contains("Second:"));
        assert!(formatted.contains("Microsecond:"));
        assert!(formatted.contains("Ordinal:"));
        assert!(formatted.contains("Iso 8601:"));
        assert!(formatted.contains("Iso Week:"));
        assert!(formatted.contains("Time:"));
        assert!(formatted.contains("TZ:"));
        assert!(formatted.contains("Offset:"));
        assert!(formatted.contains("Now:"));
    }

    #[test]
    // Test the `next_day` function
    fn test_next_day() {
        let date_time = DateTime::new();
        let next_day = date_time.next_day();
        assert_eq!(next_day.day, date_time.day + 1);
    }
    #[test]
    // Test the `previous_day` function
    fn test_previous_day() {
        let date_time = DateTime::new();
        let previous_day = date_time.previous_day();
        assert_eq!(previous_day.day, date_time.day - 1);
    }
    #[test]
    // Test the `from_str_impl` function
    fn test_from_str_impl() {
        let date_str = "2022-01-01T12:00:00+01:00";

        let expected = DateTime {
            day: 1,
            hour: 12,
            iso_8601: date_str.to_owned(),
            iso_week: 0,
            microsecond: 0,
            minute: 0,
            month: "".to_owned(),
            now: "".to_owned(),
            offset: "".to_owned(),
            ordinal: 0,
            second: 0,
            time: "".to_owned(),
            tz: "".to_owned(),
            weekday: "".to_owned(),
            year: 2022,
        };
        let result: Result<DateTime, dtt::DateTimeError> =
            date_str.parse::<DateTime>();
        match result {
            Ok(dt) => assert_eq!(dt, expected),
            Err(err) => panic!("Error: {:?}", err),
        }

        let date_str: &str = "invalid";
        let result: Result<DateTime, dtt::DateTimeError> =
            date_str.parse::<DateTime>();
        assert!(result.is_err());
    }

    #[test]
    // Test the `from_str` function
    fn test_from_str() {
        let date_str = "2022-01-01T12:00:00+01:00";

        let expected = DateTime {
            day: 1,
            hour: 12,
            iso_8601: date_str.to_owned(),
            iso_week: 0,
            microsecond: 0,
            minute: 0,
            month: "".to_owned(),
            now: "".to_owned(),
            offset: "".to_owned(),
            ordinal: 0,
            second: 0,
            time: "".to_owned(),
            tz: "".to_owned(),
            weekday: "".to_owned(),
            year: 2022,
        };
        let result = DateTime::from_str(date_str);
        match result {
            Ok(dt) => assert_eq!(dt, expected),
            Err(err) => panic!("Error: {:?}", err),
        }

        let date_str = "invalid";
        let result = DateTime::from_str(date_str);
        assert!(result.is_err());
    }

    #[test]
    // Test the `from_str_invalid` function
    fn test_from_str_invalid() {
        let date_str = "invalid";
        let result = DateTime::from_str(date_str);
        assert!(result.is_err());
    }
    #[test]
    // Test the `relative_delta` function
    fn test_relative_delta() {
        let dt = DateTime {
            day: 10,
            hour: 12,
            iso_8601: String::from(""),
            iso_week: 5,
            microsecond: 456789,
            minute: 30,
            month: String::from("February"),
            now: String::from(""),
            offset: 1.to_string(),
            ordinal: 0,
            second: 45,
            time: String::from(""),
            tz: String::from("UTC"),
            weekday: String::from("Saturday"),
            year: 2023,
        };

        let new_dt = dt.relative_delta();

        assert_eq!(new_dt.day, 10);
        assert_eq!(new_dt.hour, 12);
        assert_eq!(new_dt.microsecond, 456789);
        assert_eq!(new_dt.minute, 30);
        assert_eq!(new_dt.month, "February");
        assert_eq!(new_dt.second, 45);
        assert_eq!(new_dt.iso_week, 5);
        assert_eq!(new_dt.year, 2023);
        assert!(new_dt.ordinal > 0);
    }
    #[test]
    // Test the `from_str_valid_input` function
    fn test_from_str_valid_input() {
        let input = "12:34:56:78:01:02:03:04:05:06:07:08:09:10:11:12";
        let expected = DateTime {
            day: 1,
            hour: 12,
            iso_8601: "05".to_string(),
            iso_week: 4,
            microsecond: 78,
            minute: 34,
            month: "02".to_string(),
            now: "11".to_string(),
            offset: "10".to_string(),
            ordinal: 6,
            second: 56,
            time: "08".to_string(),
            tz: "09".to_string(),
            weekday: "07".to_string(),
            year: 3,
        };

        let parts: Vec<&str> = input.split(':').collect();

        let hour: u8 = parts[0].parse().unwrap();
        let minute: u8 = parts[1].parse().unwrap();
        let second: u8 = parts[2].parse().unwrap();
        let microsecond: u32 = parts[3].parse().unwrap();
        let day: u8 = parts[4].parse().unwrap();
        let month: String = parts[5].parse().unwrap();
        let year: i32 = parts[6].parse().unwrap();
        let iso_week: u8 = parts[7].parse().unwrap();
        let iso_8601: String = parts[8].parse().unwrap();
        let ordinal: u16 = parts[9].parse().unwrap();
        let weekday: String = parts[10].parse().unwrap();
        let time: String = parts[11].parse().unwrap();
        let tz: String = parts[12].parse().unwrap();
        let offset: String = parts[13].parse().unwrap();
        let now: String = parts[14].parse().unwrap();

        let result = DateTime {
            hour,
            minute,
            second,
            microsecond,
            day,
            month,
            year,
            iso_week,
            iso_8601,
            ordinal,
            weekday,
            time,
            tz,
            offset,
            now,
        };
        assert_eq!(result.hour, expected.hour);
        assert_eq!(result.minute, expected.minute);
        assert_eq!(result.second, expected.second);
        assert_eq!(result.microsecond, expected.microsecond);
        assert_eq!(result.day, expected.day);
        assert_eq!(result.month, expected.month);
        assert_eq!(result.year, expected.year);
        assert_eq!(result.iso_week, expected.iso_week);
        assert_eq!(result.iso_8601, expected.iso_8601);
        assert_eq!(result.ordinal, expected.ordinal);
        assert_eq!(result.weekday, expected.weekday);
        assert_eq!(result.time, expected.time);
        assert_eq!(result.tz, expected.tz);
        assert_eq!(result.offset, expected.offset);
        assert_eq!(result.now, expected.now);
    }
    #[test]
    // Test the `display` function
    fn test_display() {
        let date_time = DateTime::new();
        let expected = format!(
        "Year: {}\nMonth: {}\nDay: {}\nWeekday: {}\nHour: {}\nMinute: {}\nSecond: {}\nMicrosecond: {}\nOrdinal: {}\nIso 8601: {}\nIso Week: {}\nTime: {}\nTZ: {}\nOffset: {}\nNow: {}",
        date_time.year,
        date_time.month,
        date_time.day,
        date_time.weekday,
        date_time.hour,
        date_time.minute,
        date_time.second,
        date_time.microsecond,
        date_time.ordinal,
        date_time.iso_8601,
        date_time.iso_week,
        date_time.time,
        date_time.tz,
        date_time.offset,
        date_time.now
    );
        let result = format!("{}", date_time);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_iso_8601() {
        let valid_date = "2023-03-15T13:45:30+00:00";
        let result: Result<DateTime, dtt::DateTimeError> =
            DateTime::parse(valid_date);
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_date_without_time() {
        let valid_date = "2023-03-15";
        let result: Result<DateTime, dtt::DateTimeError> =
            DateTime::parse(valid_date);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_date_format() {
        let invalid_date = "15-03-2023";
        let result: Result<DateTime, dtt::DateTimeError> =
            DateTime::parse(invalid_date);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_iso8601_format() {
        let invalid_date = "2023-03-15T25:61:61+00:00"; // Invalid time
        let result: Result<DateTime, dtt::DateTimeError> =
            DateTime::parse(invalid_date);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_date() {
        let input: &str = "2023-05-12";
        let result: Result<DateTime, dtt::DateTimeError> =
            DateTime::parse(input);

        assert!(result.is_ok());

        let datetime = result.unwrap();
        assert_eq!(datetime.day, 12);
        assert_eq!(datetime.hour, 0);
        assert_eq!(datetime.iso_8601, "2023-05-12T00:00:00+00:00");
    }

    #[test]
    fn test_parse_invalid_format() {
        let input: &str = "2023-05-12T16:23:45Z"; // Invalid format (missing offset)
        let result: Result<DateTime, dtt::DateTimeError> =
            DateTime::parse(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_input() {
        let input: &str = "2023/05/12"; // Invalid input format
        let result: Result<DateTime, dtt::DateTimeError> =
            DateTime::parse(input);

        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Invalid date format"
        );
    }
}
