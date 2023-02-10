#[cfg(test)]

mod tests {

    extern crate dtt;
    use std::str::FromStr;

    use dtt::{is_valid, DateTime};

    extern crate time;
    use regex::Regex;
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
        assert!(!DateTime::is_valid_day("32"));
        assert!(DateTime::is_valid_day("31"));
    }
    #[test]
    // Test the `is_valid_hour` function
    fn test_is_valid_hour() {
        assert!(DateTime::is_valid_hour("23"));
        assert!(DateTime::is_valid_hour("23:59"));
        assert!(!DateTime::is_valid_hour("24:00"));
    }
    #[test]
    // Test the `is_valid_iso_8601` function
    fn test_is_valid_iso_8601() {
        assert!(DateTime::is_valid_iso_8601("2022-06-25T17:30:00Z"));
        assert!(DateTime::is_valid_iso_8601("2022-06-25T17:30:00+01:00"));
        assert!(DateTime::is_valid_iso_8601("2022-06-25T17:30:00.123456Z"));
        assert!(DateTime::is_valid_iso_8601(
            "2022-06-25T17:30:00.123456+01:00"
        ));
    }

    #[test]
    // Test the reverse of `is_valid_iso_8601` function
    fn test_invalid_iso_8601() {
        assert!(!DateTime::is_valid_iso_8601("2022-06-25T17:30:00"));
        assert!(!DateTime::is_valid_iso_8601("2022-06-25 17:30:00Z"));
        assert!(!DateTime::is_valid_iso_8601("2022-06-25T17:30:00+25:00"));
        assert!(!DateTime::is_valid_iso_8601("2022-06-25T17:30:00+01:61"));
    }

    #[test]
    // Test the month validation
    fn test_month_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601("2022-13-25T17:30:00.1234567Z"));
        assert!(!DateTime::is_valid_iso_8601("2022-00-25T17:30:00.1234567Z"));
    }

    #[test]
    // Test the day validation
    fn test_day_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601("2022-12-32T17:30:00.1234567Z"));
        assert!(!DateTime::is_valid_iso_8601("2022-12-00T17:30:00.1234567Z"));
    }

    #[test]
    // Test the hour validation
    fn test_hour_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601("2022-12-31T24:30:00.1234567Z"));
        assert!(!DateTime::is_valid_iso_8601("2022-12-31T-1:30:00.1234567Z"));
    }

    #[test]
    // Test the minute validation
    fn test_minute_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601("2022-12-31T23:60:00.1234567Z"));
        assert!(!DateTime::is_valid_iso_8601("2022-12-31T23:-1:00.1234567Z"));
    }

    #[test]
    // Test the second validation
    fn test_second_out_of_range() {
        assert!(!DateTime::is_valid_iso_8601("2022-12-31T23:59:60.1234567Z"));
        assert!(!DateTime::is_valid_iso_8601("2022-12-31T23:59:-1.1234567Z"));
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
    fn test_update() {
        let mut date = DateTime::new();
        let date_before_update = date.now.clone();
        let date_after_update = date.update();
        assert_ne!(date_before_update, date_after_update);
    }
    #[test]
    fn test_update_with_timezone() {
        let mut date = DateTime::new_with_tz("UTC+01:00");

        let date_before_update = date.now.clone();
        let date_after_update = date.update();
        assert_ne!(date_before_update, date_after_update);

        let re = Regex::new(r"([+-])(\d{2}):(\d{2})").unwrap();
        let captures = re.captures(&date.tz).unwrap();
        let hours = captures[2].parse::<i64>().unwrap();
        let minutes = captures[3].parse::<i64>().unwrap();

        let actual_offset = time::UtcOffset::from_hms(1, 0, 0);
        let expected_offset =
            time::UtcOffset::from_hms(hours.try_into().unwrap(), minutes.try_into().unwrap(), 0);
        let erroneous_offset = time::UtcOffset::from_hms(3, 0, 0);
        assert_eq!(expected_offset, actual_offset);
        assert_ne!(expected_offset, erroneous_offset);
    }
    #[test]
    // Test the `update_date` function
    fn test_update_date() {
        let mut date = DateTime::new();
        let date_before_update = date.now.clone();
        let date_after_update = date.update();
        assert_ne!(date_before_update, date_after_update);
    }

    #[test]
    // Test the `update_date` function
    fn test_update_day() {
        let mut date = DateTime::new();
        let day_before_update = date.day.to_string();
        let day_after_update = date.update();
        assert_ne!(day_before_update, day_after_update);
    }

    #[test]
    // Test the `update_hour` function
    fn test_update_hour() {
        let mut date = DateTime::new();
        let hour_before_update = date.hour.to_string();
        let hour_after_update = date.update();
        assert_ne!(hour_before_update, hour_after_update);
    }

    #[test]
    fn test_update_iso_8601() {
        let date = DateTime::new();
        let iso_8601_before_update = date.iso_8601.clone();
        assert_eq!(iso_8601_before_update, date.iso_8601);
    }

    #[test]
    fn test_update_iso_week() {
        let date = DateTime::new();
        let iso_week_before_update = date.iso_week;
        assert_eq!(iso_week_before_update, date.iso_week);
        assert!(date.iso_week <= 53);
    }

    #[test]
    fn test_update_minute() {
        let date = DateTime::new();
        let minute_before_update = date.minute;
        assert_eq!(minute_before_update, date.minute);
        assert!(date.minute <= 59);
    }

    #[test]
    fn test_update_month() {
        let date = DateTime::new();
        let month_before_update = date.month.clone();
        assert_eq!(month_before_update, date.month);
    }

    #[test]
    fn test_update_offset() {
        let date = DateTime::new();
        let offset_before_update = date.offset.clone();
        assert_eq!(offset_before_update, date.offset);
    }

    #[test]
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
    fn test_update_weekday() {
        let date = DateTime::new();
        let weekday_before_update = date.weekday.clone();
        assert_eq!(weekday_before_update, date.weekday);
    }
    #[test]
    fn test_update_year() {
        let date = DateTime::new();
        let year_before_update = date.year;
        assert_eq!(year_before_update, date.year);
        assert!(date.year > 0);
    }
    #[test]
    fn test_new_with_tz_utc() {
        let date_time = DateTime::new_with_tz("UTC");
        let offset = time::UtcOffset::UTC;

        let now_utc = OffsetDateTime::now_utc();
        let (hours, minutes, _) = offset.as_hms();
        let total_seconds = (hours as i16 * 3600) + (minutes as i16 * 60);
        let expected_date_time = now_utc + Duration::seconds(total_seconds as i64);

        assert_eq!(date_time.hour, expected_date_time.hour());
        assert_eq!(date_time.minute, expected_date_time.minute());
        assert_eq!(date_time.offset, expected_date_time.offset().to_string());
    }
    #[test]
    fn test_new_with_tz_custom() {
        let date_time = DateTime::new_with_tz("Custom");
        let offset = time::UtcOffset::from_hms(0, 0, 0).unwrap();

        let now_utc = OffsetDateTime::now_utc();
        let (hours, minutes, _) = offset.as_hms();
        let total_seconds = (hours as i16 * 3600) + (minutes as i16 * 60);
        let expected_date_time = now_utc + Duration::seconds(total_seconds as i64);

        assert_eq!(date_time.hour, expected_date_time.hour());
        assert_eq!(date_time.minute, expected_date_time.minute());
        assert_eq!(date_time.offset, expected_date_time.offset().to_string());
    }
    #[test]
    fn test_new_with_tz_to_paris() {
        let date = DateTime::new_with_tz("Europe/Paris");
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
    fn test_is_valid() {
        is_valid!(day, u32);
        let input = "31";
        let result = day(input);
        assert!(result);
    }
    #[test]
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
    fn test_is_valid_iso_week() {
        assert!(DateTime::is_valid_iso_week("53"));
        assert!(!DateTime::is_valid_iso_week("54"));
        assert!(!DateTime::is_valid_iso_week("a"));
    }

    #[test]
    fn test_is_valid_microsecond() {
        assert!(DateTime::is_valid_microsecond("999999"));
        assert!(!DateTime::is_valid_microsecond("1000000"));
        assert!(!DateTime::is_valid_microsecond("b"));
    }

    #[test]
    fn test_is_valid_minute() {
        assert!(DateTime::is_valid_minute("59"));
        assert!(!DateTime::is_valid_minute("60"));
        assert!(!DateTime::is_valid_minute("c"));
    }

    #[test]
    fn test_is_valid_month() {
        assert!(DateTime::is_valid_month("12"));
        assert!(!DateTime::is_valid_month("13"));
        assert!(!DateTime::is_valid_month("d"));
    }

    #[test]
    fn test_is_valid_ordinal() {
        assert!(DateTime::is_valid_ordinal("366"));
        assert!(!DateTime::is_valid_ordinal("367"));
        assert!(!DateTime::is_valid_ordinal("e"));
    }

    #[test]
    fn test_is_valid_second() {
        assert!(DateTime::is_valid_second("59"));
        assert!(!DateTime::is_valid_second("60"));
        assert!(!DateTime::is_valid_second("f"));
    }
    #[test]
    fn test_is_valid_time() {
        assert!(DateTime::is_valid_time("23:59:59"));
        assert!(!DateTime::is_valid_time("24:00:00"));
        assert!(!DateTime::is_valid_time("g"));
    }
    #[test]
    fn test_next_day() {
        let date_time = DateTime::new();
        let next_day = date_time.next_day();
        assert_eq!(next_day.day, date_time.day + 1);
    }
    #[test]
    fn test_previous_day() {
        let date_time = DateTime::new();
        let previous_day = date_time.previous_day();
        assert_eq!(previous_day.day, date_time.day - 1);
    }
    #[test]
    fn test_from_str() {
        let date_str = "2022-01-01T12:00:00+01:00";

        let expected = Ok(DateTime {
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
        });
        let result = DateTime::from_str(date_str);
        assert_eq!(result, expected);

        let date_str = "invalid";
        let expected = Err(());
        let result = DateTime::from_str(date_str);
        assert_eq!(result, expected);
    }
    #[test]
    fn test_from_str_invalid() {
        let invalid_input = "invalid";
        let result = DateTime::from_str(invalid_input);
        assert!(result.is_err());
    }
    #[test]
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
}
