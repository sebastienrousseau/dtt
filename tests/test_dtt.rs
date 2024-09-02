// test_dtt.rs
//
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

/// Unit tests for the `DateTime` struct and associated methods.
#[cfg(test)]
mod tests {
    use dtt::datetime::DateTime;
    use time::{Month, Weekday};

    /// Tests the creation of a new `DateTime` instance using the current UTC time.
    #[test]
    fn test_create_datetime_new() {
        let date = DateTime::new();
        assert!(date.year() > 2000, "Year should be greater than 2000");
    }

    /// Tests the creation of a `DateTime` instance with a specified timezone.
    #[test]
    fn test_create_datetime_with_tz() {
        let paris_time = DateTime::new_with_tz("CET")
            .expect("Failed to create DateTime with CET timezone");
        assert_eq!(paris_time.offset().to_string(), "+01:00:00");
    }

    /// Tests the creation of a `DateTime` instance with a custom offset.
    #[test]
    fn test_create_datetime_with_custom_offset() {
        let custom_offset_time =
            DateTime::new_with_custom_offset(5, 30)
                .expect("Failed to create DateTime with custom offset");
        assert_eq!(
            custom_offset_time.offset().to_string(),
            "+05:30:00"
        );
    }

    /// Tests parsing a `DateTime` from an RFC3339 formatted string.
    #[test]
    fn test_parse_datetime_rfc3339() {
        let parsed_date = DateTime::parse("2023-05-20T15:30:00Z")
            .expect("Failed to parse date");
        assert_eq!(parsed_date.year(), 2023);
        assert_eq!(parsed_date.month(), Month::May);
        assert_eq!(parsed_date.day(), 20);
    }

    /// Tests parsing a `DateTime` from a custom formatted string.
    #[test]
    fn test_parse_datetime_custom_format() {
        let custom_parsed_date = DateTime::parse_custom_format(
            "2023-05-20 15:30:00",
            "[year]-[month]-[day] [hour]:[minute]:[second]",
        )
        .expect("Failed to parse custom format date");
        assert_eq!(custom_parsed_date.year(), 2023);
        assert_eq!(custom_parsed_date.month(), Month::May);
        assert_eq!(custom_parsed_date.day(), 20);
    }

    /// Tests accessing various components of a `DateTime` instance.
    #[test]
    fn test_datetime_components() {
        let date = DateTime::new();
        assert!(matches!(
            date.month(),
            Month::January
                | Month::February
                | Month::March
                | Month::April
                | Month::May
                | Month::June
                | Month::July
                | Month::August
                | Month::September
                | Month::October
                | Month::November
                | Month::December
        ));
        assert!(date.day() <= 31);
        assert!(date.hour() < 24);
        assert!(date.minute() < 60);
        assert!(date.second() < 60);
        assert!(date.microsecond() < 1_000_000);
        assert!(date.ordinal() <= 366);
        assert!(date.iso_week() <= 53); // Accessing the week directly from the tuple
    }

    /// Tests adding and subtracting days from a `DateTime` instance.
    #[test]
    fn test_add_subtract_days() {
        let date = DateTime::parse("2023-01-01T00:00:00Z")
            .expect("Failed to parse date");

        let future_date = date.add_days(7).expect("Adding days failed");
        assert_eq!(future_date.day(), 8);
        assert_eq!(future_date.month(), Month::January);

        let previous_day =
            date.previous_day().expect("Failed to get previous day");
        assert_eq!(previous_day.day(), 31);
        assert_eq!(previous_day.month(), Month::December);
        assert_eq!(previous_day.year(), 2022);

        let next_day = date.next_day().expect("Failed to get next day");
        assert_eq!(next_day.day(), 2);
        assert_eq!(next_day.month(), Month::January);
    }

    /// Tests getting the start and end of the week, month, and year for a `DateTime` instance.
    #[test]
    fn test_start_end_of_week_month_year() {
        let date = DateTime::parse("2023-09-15T00:00:00Z")
            .expect("Failed to parse date");

        let start_of_week =
            date.start_of_week().expect("Failed to get start of week");
        assert_eq!(start_of_week.weekday(), Weekday::Monday);

        let end_of_week =
            date.end_of_week().expect("Failed to get end of week");
        assert_eq!(end_of_week.weekday(), Weekday::Sunday);

        let start_of_month = date
            .start_of_month()
            .expect("Failed to get start of month");
        assert_eq!(start_of_month.day(), 1);

        let end_of_month =
            date.end_of_month().expect("Failed to get end of month");
        assert_eq!(end_of_month.day(), 30);

        let start_of_year =
            date.start_of_year().expect("Failed to get start of year");
        assert_eq!(start_of_year.month(), Month::January);
        assert_eq!(start_of_year.day(), 1);

        let end_of_year =
            date.end_of_year().expect("Failed to get end of year");
        assert_eq!(end_of_year.month(), Month::December);
        assert_eq!(end_of_year.day(), 31);
    }

    /// Tests whether a `DateTime` instance is within a specified date range.
    #[test]
    fn test_is_within_range() {
        let date = DateTime::new();
        let range_end_date =
            date.add_days(10).expect("Adding days failed");
        let is_within_range =
            date.is_within_range(&date, &range_end_date);
        assert!(is_within_range);
    }

    /// Tests calculating the duration between two `DateTime` instances.
    #[test]
    fn test_duration_since() {
        let date = DateTime::parse("2023-01-02T00:00:00Z")
            .expect("Failed to parse date");
        let previous_day =
            date.previous_day().expect("Failed to get previous day");
        let duration = date.duration_since(&previous_day);
        assert_eq!(duration.whole_days(), 1); // Use whole_days() to get the number of days
    }

    /// Tests converting a `DateTime` instance to a different timezone.
    #[test]
    fn test_convert_to_timezone() {
        let date = DateTime::new();
        let converted_to_pst = date
            .convert_to_tz("PST")
            .expect("Failed to convert timezone");
        assert_eq!(converted_to_pst.offset().to_string(), "-08:00:00");
    }

    /// Tests formatting a `DateTime` instance in various formats.
    #[test]
    fn test_formatting_datetime() {
        let date = DateTime::new();
        let custom_format = date
            .format("[year]-[month]-[day] [hour]:[minute]:[second]")
            .expect("Failed to format date");
        assert!(custom_format.contains("-")); // Basic check to ensure the format is applied

        let rfc3339_format = date
            .format_rfc3339()
            .expect("Failed to format RFC3339 date");
        assert!(rfc3339_format.contains("T"));

        let iso8601_format = date
            .format_iso8601()
            .expect("Failed to format ISO8601 date");
        assert!(iso8601_format.contains("T"));
    }
}
