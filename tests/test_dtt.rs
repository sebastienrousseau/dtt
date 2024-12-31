// test_dtt.rs
//
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # DTT Test Crate
//!
//! This file provides unit tests for the `DateTime` struct and its associated methods.
//! It verifies various functionalities including parsing, formatting, timezone conversions, and date-time arithmetic.

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
    fn test_create_datetime_with_tz(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let paris_time = DateTime::new_with_tz("CET")?;
        assert_eq!(paris_time.offset().to_string(), "+01:00:00");
        Ok(())
    }

    /// Tests the creation of a `DateTime` instance with a custom offset.
    #[test]
    fn test_create_datetime_with_custom_offset(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let custom_offset_time =
            DateTime::new_with_custom_offset(5, 30)?;
        assert_eq!(
            custom_offset_time.offset().to_string(),
            "+05:30:00"
        );
        Ok(())
    }

    /// Tests parsing a `DateTime` from an RFC3339 formatted string.
    #[test]
    fn test_parse_datetime_rfc3339(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let parsed_date = DateTime::parse("2023-05-20T15:30:00Z")?;
        assert_eq!(parsed_date.year(), 2023);
        assert_eq!(parsed_date.month(), Month::May);
        assert_eq!(parsed_date.day(), 20);
        Ok(())
    }

    /// Tests parsing a `DateTime` from a custom formatted string.
    #[test]
    fn test_parse_datetime_custom_format(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let custom_parsed_date = DateTime::parse_custom_format(
            "2023-05-20 15:30:00",
            "[year]-[month]-[day] [hour]:[minute]:[second]",
        )?;
        assert_eq!(custom_parsed_date.year(), 2023);
        assert_eq!(custom_parsed_date.month(), Month::May);
        assert_eq!(custom_parsed_date.day(), 20);
        Ok(())
    }

    /// Tests adding and subtracting days from a `DateTime` instance.
    #[test]
    fn test_add_subtract_days() -> Result<(), Box<dyn std::error::Error>>
    {
        let date = DateTime::parse("2023-01-01T00:00:00Z")?;

        let future_date = date.add_days(7)?;
        assert_eq!(future_date.day(), 8);
        assert_eq!(future_date.month(), Month::January);

        let previous_day = date.previous_day()?;
        assert_eq!(previous_day.day(), 31);
        assert_eq!(previous_day.month(), Month::December);
        assert_eq!(previous_day.year(), 2022);

        let next_day = date.next_day()?;
        assert_eq!(next_day.day(), 2);
        assert_eq!(next_day.month(), Month::January);

        Ok(())
    }

    /// Tests getting the start and end of the week, month, and year for a `DateTime` instance.
    #[test]
    fn test_start_end_of_week_month_year(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let date = DateTime::parse("2023-09-15T00:00:00Z")?;

        let start_of_week = date.start_of_week()?;
        assert_eq!(start_of_week.weekday(), Weekday::Monday);

        let end_of_week = date.end_of_week()?;
        assert_eq!(end_of_week.weekday(), Weekday::Sunday);

        let start_of_month = date.start_of_month()?;
        assert_eq!(start_of_month.day(), 1);

        let end_of_month = date.end_of_month()?;
        assert_eq!(end_of_month.day(), 30);

        let start_of_year = date.start_of_year()?;
        assert_eq!(start_of_year.month(), Month::January);
        assert_eq!(start_of_year.day(), 1);

        let end_of_year = date.end_of_year()?;
        assert_eq!(end_of_year.month(), Month::December);
        assert_eq!(end_of_year.day(), 31);

        Ok(())
    }

    /// Tests whether a `DateTime` instance is within a specified date range.
    #[test]
    fn test_is_within_range() -> Result<(), Box<dyn std::error::Error>>
    {
        let date = DateTime::new();
        let range_end_date = date.add_days(10)?;
        let is_within_range =
            date.is_within_range(&date, &range_end_date);
        assert!(is_within_range);
        Ok(())
    }

    /// Tests calculating the duration between two `DateTime` instances.
    #[test]
    fn test_duration_since() -> Result<(), Box<dyn std::error::Error>> {
        let date = DateTime::parse("2023-01-02T00:00:00Z")?;
        let previous_day = date.previous_day()?;
        let duration = date.duration_since(&previous_day);
        assert_eq!(duration.whole_days(), 1); // Use whole_days() to get the number of days
        Ok(())
    }

    /// Tests converting a `DateTime` instance to a different timezone.
    #[test]
    fn test_convert_to_timezone(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let date = DateTime::new();
        let converted_to_pst = date.convert_to_tz("PST")?;
        assert_eq!(converted_to_pst.offset().to_string(), "-08:00:00");
        Ok(())
    }

    /// Tests formatting a `DateTime` instance in various formats.
    #[test]
    fn test_formatting_datetime(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let date = DateTime::new();

        let custom_format = date
            .format("[year]-[month]-[day] [hour]:[minute]:[second]")?;
        assert!(
            custom_format.contains('-'),
            "Custom format should contain '-'"
        ); // Basic check to ensure the format is applied

        let rfc3339_format = date.format_rfc3339()?;
        assert!(
            rfc3339_format.contains('T'),
            "RFC3339 format should contain 'T'"
        ); // Use char for single character

        let iso8601_format = date.format_iso8601()?;
        assert!(
            iso8601_format.contains('T'),
            "ISO8601 format should contain 'T'"
        ); // Use char for single character

        Ok(())
    }
}
