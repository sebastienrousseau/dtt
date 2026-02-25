// SPDX-License-Identifier: MIT OR Apache-2.0

use dtt::{calendar::CalendarDuration, DateTime};
use time::UtcOffset;

#[test]
fn test_calendar_duration_construction() {
    let months = CalendarDuration::months(13);
    assert_eq!(months, CalendarDuration::months(13));

    let years = CalendarDuration::years(2); 
    assert_eq!(years, CalendarDuration::months(24));
}

#[test]
fn test_add_calendar_months_basic() {
    let dt = DateTime::from_components(2024, 1, 15, 12, 0, 0, UtcOffset::UTC).unwrap();
    let next_month = dt.add_calendar(CalendarDuration::months(1)).unwrap();
    assert_eq!(next_month.month() as u8, 2);
    assert_eq!(next_month.year(), 2024);
}

#[test]
fn test_add_calendar_months_overflow_years() {
    let dt = DateTime::from_components(2024, 11, 15, 12, 0, 0, UtcOffset::UTC).unwrap();
    let next = dt.add_calendar(CalendarDuration::months(3)).unwrap(); // Nov + 3 = Feb next year
    assert_eq!(next.month() as u8, 2);
    assert_eq!(next.year(), 2025);
}

#[test]
fn test_add_calendar_months_underflow_years() {
    let dt = DateTime::from_components(2024, 2, 15, 12, 0, 0, UtcOffset::UTC).unwrap();
    let next = dt.add_calendar(CalendarDuration::months(-3)).unwrap(); // Feb - 3 = Nov prev year
    assert_eq!(next.month() as u8, 11);
    assert_eq!(next.year(), 2023);
}

#[test]
fn test_add_calendar_leap_year_clamp() {
    let dt = DateTime::from_components(2024, 1, 31, 12, 0, 0, UtcOffset::UTC).unwrap();
    let next = dt.add_calendar(CalendarDuration::months(1)).unwrap(); // Feb 2024 is leap year, 29 days
    assert_eq!(next.month() as u8, 2);
    assert_eq!(next.day(), 29);
}

#[test]
fn test_add_calendar_non_leap_year_clamp() {
    let dt = DateTime::from_components(2023, 1, 31, 12, 0, 0, UtcOffset::UTC).unwrap();
    let next = dt.add_calendar(CalendarDuration::months(1)).unwrap(); // Feb 2023 is non-leap, 28 days
    assert_eq!(next.month() as u8, 2);
    assert_eq!(next.day(), 28);
}

#[test]
fn test_add_calendar_day_preservation() {
    let dt = DateTime::from_components(2023, 4, 30, 12, 0, 0, UtcOffset::UTC).unwrap();
    let next = dt.add_calendar(CalendarDuration::months(1)).unwrap(); // May has 31 days, so Day 30 is preserved
    assert_eq!(next.month() as u8, 5);
    assert_eq!(next.day(), 30);
}
