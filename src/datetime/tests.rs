// Copyright © 2025 DateTime (DTT) library.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Unit tests for the `datetime` module.
//!
//! Lifted out of `mod.rs` so the core type and impls are easier to read.
//! The module is gated on `#[cfg(test)]` in `mod.rs` so the test file
//! compiles only under `cargo test`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::*;
use std::str::FromStr;
#[test]
fn test_new() {
    let dt = DateTime::new();
    assert_eq!(dt.offset(), UtcOffset::UTC);
}

#[test]
fn test_new_with_tz() {
    let est_dt = DateTime::new_with_tz("EST_USA").unwrap();
    assert_eq!(est_dt.offset().whole_hours(), -5);

    let invalid = DateTime::new_with_tz("INVALID");
    assert!(matches!(invalid, Err(DateTimeError::InvalidTimezone)));
}

#[test]
fn test_new_with_custom_offset() {
    let dt = DateTime::new_with_custom_offset(5, 30).unwrap();
    assert_eq!(dt.offset().whole_hours(), 5);
    assert_eq!(dt.offset().minutes_past_hour(), 30);

    // Test invalid offsets
    let too_large_hours = DateTime::new_with_custom_offset(24, 0);
    assert!(too_large_hours.is_err());
    let too_large_minutes = DateTime::new_with_custom_offset(0, 60);
    assert!(too_large_minutes.is_err());
}

#[test]
fn test_from_components() {
    let dt_val =
        DateTime::from_components(2024, 1, 1, 12, 0, 0, UtcOffset::UTC)
            .unwrap();
    assert_eq!(dt_val.year(), 2024);
    assert_eq!(dt_val.month(), Month::January);
    assert_eq!(dt_val.day(), 1);
    assert_eq!(dt_val.hour(), 12);
    assert_eq!(dt_val.minute(), 0);
    assert_eq!(dt_val.second(), 0);

    // Test invalid dates
    let invalid_month =
        DateTime::from_components(2024, 13, 1, 0, 0, 0, UtcOffset::UTC);
    assert!(invalid_month.is_err());

    let invalid_day =
        DateTime::from_components(2024, 2, 30, 0, 0, 0, UtcOffset::UTC);
    assert!(invalid_day.is_err());
}

#[test]
fn test_parse() {
    // Test RFC 3339 format
    let dt = DateTime::parse("2024-01-01T12:00:00Z");
    assert!(dt.is_ok());

    // Test ISO 8601 date
    let dt_val = DateTime::parse("2024-01-01").unwrap();
    assert_eq!(dt_val.hour(), 0);
    assert_eq!(dt_val.minute(), 0);

    // Test invalid formats
    let invalid1 = DateTime::parse("invalid");
    assert!(invalid1.is_err());
    let invalid2 = DateTime::parse("2024-13-01");
    assert!(invalid2.is_err());
}

#[test]
fn test_format() {
    let dt = DateTime::new();
    let maybe_formatted = dt.format("[year]-[month]-[day]");
    assert!(maybe_formatted.is_ok());

    let invalid_format = dt.format("[invalid]");
    assert!(invalid_format.is_err());
}

#[test]
fn test_timezone_conversion() {
    let utc = DateTime::new();
    let est_val = utc.convert_to_tz("EST_USA").unwrap();
    assert_eq!(est_val.offset().whole_hours(), -5);

    let invalid = utc.convert_to_tz("INVALID");
    assert!(invalid.is_err());
}

#[test]
fn test_arithmetic() {
    let dt = DateTime::new();

    // Test adding days
    let future = dt.add_days(7);
    assert!(future.is_ok());

    // Test subtracting days (negative)
    let past = dt.add_days(-7);
    assert!(past.is_ok());

    // Test adding months
    let next_month = dt.add_months(1);
    assert!(next_month.is_ok());

    // Test month edge cases
    let jan31_dt =
        DateTime::from_components(2024, 1, 31, 0, 0, 0, UtcOffset::UTC)
            .unwrap();
    let feb_dt = jan31_dt.add_months(1).unwrap();
    // 2024 is a leap year => Feb has 29 days
    assert_eq!(feb_dt.day(), 29);
}

#[test]
fn test_leap_year() {
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(1900));
}

#[test]
fn test_validation() {
    // Test day validation
    assert!(DateTime::is_valid_day("1"));
    assert!(DateTime::is_valid_day("31"));
    assert!(!DateTime::is_valid_day("0"));
    assert!(!DateTime::is_valid_day("32"));
    assert!(!DateTime::is_valid_day("abc"));

    // Test hour validation
    assert!(DateTime::is_valid_hour("0"));
    assert!(DateTime::is_valid_hour("23"));
    assert!(!DateTime::is_valid_hour("24"));

    // Test minute validation
    assert!(DateTime::is_valid_minute("0"));
    assert!(DateTime::is_valid_minute("59"));
    assert!(!DateTime::is_valid_minute("60"));

    // Test time string validation
    assert!(DateTime::is_valid_time("00:00:00"));
    assert!(DateTime::is_valid_time("23:59:59"));
    assert!(!DateTime::is_valid_time("24:00:00"));
    assert!(!DateTime::is_valid_time("23:60:00"));
    assert!(!DateTime::is_valid_time("23:59:60"));

    // Year validation must agree with what `from_components` accepts:
    // the `time::Date` range is -9999..=9999.
    assert!(DateTime::is_valid_year("0"));
    assert!(DateTime::is_valid_year("2024"));
    assert!(DateTime::is_valid_year("9999"));
    assert!(DateTime::is_valid_year("-9999"));
    assert!(!DateTime::is_valid_year("10000"));
    assert!(!DateTime::is_valid_year("-10000"));
    assert!(!DateTime::is_valid_year("99999999"));
    assert!(!DateTime::is_valid_year("abc"));
}

#[test]
fn test_iso_year_at_boundary() {
    // 2022-01-01 is a Saturday and falls in ISO week 52 of 2021,
    // so its ISO year (2021) differs from its calendar year (2022).
    let dt =
        DateTime::from_components(2022, 1, 1, 0, 0, 0, UtcOffset::UTC)
            .unwrap();
    assert_eq!(dt.year(), 2022);
    assert_eq!(dt.iso_week(), 52);
    assert_eq!(dt.iso_year(), 2021);

    // Mid-year: ISO year matches calendar year.
    let mid =
        DateTime::from_components(2024, 6, 15, 0, 0, 0, UtcOffset::UTC)
            .unwrap();
    assert_eq!(mid.year(), mid.iso_year());
}

#[test]
fn test_range_operations() {
    let dt_val = DateTime::from_components(
        2024,
        1,
        15,
        12,
        0,
        0,
        UtcOffset::UTC,
    )
    .unwrap();
    // Test week ranges
    let ws = dt_val.start_of_week().unwrap();
    assert_eq!(ws.weekday(), Weekday::Monday);

    let we = dt_val.end_of_week().unwrap();
    assert_eq!(we.weekday(), Weekday::Sunday);

    // Test month ranges
    let ms = dt_val.start_of_month().unwrap();
    assert_eq!(ms.day(), 1);

    let me = dt_val.end_of_month().unwrap();
    assert_eq!(me.day(), 31);

    // Test year ranges
    let ys = dt_val.start_of_year().unwrap();
    assert_eq!(ys.month(), Month::January);
    assert_eq!(ys.day(), 1);

    let ye = dt_val.end_of_year().unwrap();
    assert_eq!(ye.month(), Month::December);
    assert_eq!(ye.day(), 31);
}

#[test]
fn test_ordering() {
    let dt1 =
        DateTime::from_components(2024, 1, 1, 0, 0, 0, UtcOffset::UTC);
    let dt2 =
        DateTime::from_components(2024, 1, 2, 0, 0, 0, UtcOffset::UTC);

    let a = dt1.unwrap();
    let b = dt2.unwrap();
    assert!(a < b);
    assert!(b > a);
    assert_ne!(a, b);
}

#[test]
fn test_duration() {
    let dt1 =
        DateTime::from_components(2024, 1, 1, 0, 0, 0, UtcOffset::UTC);
    let dt2 =
        DateTime::from_components(2024, 1, 2, 0, 0, 0, UtcOffset::UTC);

    let a = dt1.unwrap();
    let b = dt2.unwrap();
    let duration = b.duration_since(&a);
    assert_eq!(duration.whole_days(), 1);
}

#[test]
fn test_from_str() {
    let dt = DateTime::from_str("2024-01-01T00:00:00Z");
    assert!(dt.is_ok());
    let invalid = DateTime::from_str("invalid");
    assert!(invalid.is_err());
}

#[test]
fn test_display() {
    let dt_val =
        DateTime::from_components(2024, 1, 1, 0, 0, 0, UtcOffset::UTC)
            .unwrap();
    assert_eq!(dt_val.to_string(), "2024-01-01T00:00:00Z");
}

#[test]
fn test_hash() {
    use std::collections::HashSet;
    let dt1 =
        DateTime::from_components(2024, 1, 1, 0, 0, 0, UtcOffset::UTC);
    let dt2 =
        DateTime::from_components(2024, 1, 1, 0, 0, 0, UtcOffset::UTC);
    let a = dt1.unwrap();
    let b = dt2.unwrap();
    let mut set = HashSet::new();
    assert!(
        set.insert(a),
        "The set should not have contained `a` before"
    );
    assert!(set.contains(&b));
}

#[test]
fn test_builder_pattern() {
    let builder = DateTimeBuilder::new()
        .year(2024)
        .month(1)
        .day(1)
        .hour(12)
        .minute(30)
        .second(45)
        .offset(UtcOffset::UTC);

    let value = builder.build().unwrap();
    assert_eq!(value.year(), 2024);
    assert_eq!(value.month(), Month::January);
    assert_eq!(value.day(), 1);
    assert_eq!(value.hour(), 12);
    assert_eq!(value.minute(), 30);
    assert_eq!(value.second(), 45);
}

// -------------------------------------------------------------------------
// Error-path coverage. These tests pin every fallible production branch
// so that the coverage report stays at (or near) 100% and a future
// refactor cannot silently turn an error into a panic.
// -------------------------------------------------------------------------

#[test]
fn test_set_time_invalid_returns_err() {
    let dt = DateTime::new();
    // Hour out of range — exercises the `Time::from_hms` Err arm.
    assert!(matches!(
        dt.set_time(24, 0, 0),
        Err(DateTimeError::InvalidTime)
    ));
    assert!(matches!(
        dt.set_time(0, 60, 0),
        Err(DateTimeError::InvalidTime)
    ));
    assert!(matches!(
        dt.set_time(0, 0, 60),
        Err(DateTimeError::InvalidTime)
    ));
}

#[test]
fn test_set_date_invalid_month_returns_err() {
    let dt = DateTime::new();
    // Month 13 — exercises the `Month::try_from` Err arm in `set_date`.
    assert!(matches!(
        dt.set_date(2024, 13, 1),
        Err(DateTimeError::InvalidDate)
    ));
    // Day 30 in February — exercises the `Date::from_calendar_date` Err arm.
    assert!(matches!(
        dt.set_date(2024, 2, 30),
        Err(DateTimeError::InvalidDate)
    ));
}

#[test]
fn test_parse_custom_format_invalid_input_returns_err() {
    // Format is valid, input does not match it.
    let result = DateTime::parse_custom_format(
        "not-a-date",
        "[year]-[month]-[day]",
    );
    assert!(matches!(result, Err(DateTimeError::InvalidFormat)));
}

#[test]
fn test_add_months_overflow_returns_err() {
    let dt =
        DateTime::from_components(9999, 12, 1, 0, 0, 0, UtcOffset::UTC)
            .unwrap();
    // i32::MAX months will overflow the year*12 + months calculation,
    // exercising the `checked_*` Err arm.
    assert!(matches!(
        dt.add_months(i32::MAX),
        Err(DateTimeError::InvalidDate)
    ));
    assert!(matches!(
        dt.add_months(i32::MIN),
        Err(DateTimeError::InvalidDate)
    ));
}

#[test]
fn test_add_months_year_out_of_range_returns_err() {
    let dt =
        DateTime::from_components(9999, 12, 1, 0, 0, 0, UtcOffset::UTC)
            .unwrap();
    // 12 months past Dec 9999 lands in year 10000, which `time::Date`
    // refuses by default. Exercises the final `Date::from_calendar_date`
    // Err arm in `add_months`.
    assert!(matches!(
        dt.add_months(12),
        Err(DateTimeError::InvalidDate)
    ));
}

#[test]
fn test_add_years_overflow_returns_err() {
    let dt = DateTime::new();
    // `i32::MAX` years overflows the `checked_add` in `add_years`.
    assert!(matches!(
        dt.add_years(i32::MAX),
        Err(DateTimeError::InvalidDate)
    ));
}

#[test]
fn test_add_years_out_of_range_returns_err() {
    let dt =
        DateTime::from_components(9999, 6, 15, 0, 0, 0, UtcOffset::UTC)
            .unwrap();
    // year 10000 is outside `time::Date::MAX`.
    assert!(matches!(dt.add_years(1), Err(DateTimeError::InvalidDate)));
}

#[test]
fn test_is_valid_time_wrong_part_count() {
    // Two parts — exercises the `parts.len() != 3` early return.
    assert!(!DateTime::is_valid_time("12:00"));
    // Four parts — same path.
    assert!(!DateTime::is_valid_time("12:00:00:00"));
    // Empty.
    assert!(!DateTime::is_valid_time(""));
    // Just colons.
    assert!(!DateTime::is_valid_time("::"));
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_round_trip() {
    let dt = DateTime::from_components(
        2024,
        6,
        15,
        12,
        30,
        45,
        UtcOffset::UTC,
    )
    .unwrap();
    let json = serde_json::to_string(&dt).unwrap();
    let parsed: DateTime = serde_json::from_str(&json).unwrap();
    assert_eq!(dt, parsed);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_deserialize_rejects_invalid_string() {
    // Exercises the deserialize Err arm by feeding garbage.
    let result: Result<DateTime, _> =
        serde_json::from_str("\"not-a-datetime\"");
    assert!(result.is_err());
}

#[test]
fn test_format_time_in_timezone_invalid_tz_returns_err() {
    let dt = DateTime::new();
    let result =
        dt.format_time_in_timezone("NOPE_ZONE", "[hour]:[minute]");
    assert!(matches!(result, Err(DateTimeError::InvalidTimezone)));
}

#[test]
fn test_set_time_ok_path() {
    let dt =
        DateTime::from_components(2024, 6, 15, 0, 0, 0, UtcOffset::UTC)
            .unwrap();
    let updated = dt.set_time(10, 30, 45).unwrap();
    assert_eq!(updated.hour(), 10);
    assert_eq!(updated.minute(), 30);
    assert_eq!(updated.second(), 45);
    // Date and offset preserved.
    assert_eq!(updated.year(), 2024);
    assert_eq!(updated.month(), Month::June);
    assert_eq!(updated.day(), 15);
    assert_eq!(updated.offset(), UtcOffset::UTC);
}

#[test]
fn test_is_valid_iso_week() {
    assert!(DateTime::is_valid_iso_week("1"));
    assert!(DateTime::is_valid_iso_week("52"));
    assert!(DateTime::is_valid_iso_week("53"));
    assert!(!DateTime::is_valid_iso_week("0"));
    assert!(!DateTime::is_valid_iso_week("54"));
    assert!(!DateTime::is_valid_iso_week("not-a-number"));
}

#[test]
fn test_datetime_builder_default() {
    // Exercises `impl Default for DateTimeBuilder { fn default() }`.
    let builder = DateTimeBuilder::default();
    let dt = builder.build().unwrap();
    assert_eq!(dt.year(), 1970);
    assert_eq!(dt.month(), Month::January);
    assert_eq!(dt.day(), 1);
    assert_eq!(dt.offset(), UtcOffset::UTC);
}

#[test]
fn test_days_in_month_invalid_month() {
    // Exercises the catch-all `_ => Err(...)` arm in `days_in_month`.
    assert!(matches!(
        days_in_month(2024, 0),
        Err(DateTimeError::InvalidDate)
    ));
    assert!(matches!(
        days_in_month(2024, 13),
        Err(DateTimeError::InvalidDate)
    ));
}
