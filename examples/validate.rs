#![allow(missing_docs)]

// Validate — string-based validators that mirror the builders.

use dtt::prelude::*;

fn main() {
    // Components.
    assert!(DateTime::is_valid_year("2024"));
    assert!(!DateTime::is_valid_year("10000")); // outside time::Date range

    assert!(DateTime::is_valid_month("12"));
    assert!(!DateTime::is_valid_month("13"));

    assert!(DateTime::is_valid_day("31"));
    assert!(!DateTime::is_valid_day("32"));

    assert!(DateTime::is_valid_hour("23"));
    assert!(!DateTime::is_valid_hour("24"));

    assert!(DateTime::is_valid_minute("59"));
    assert!(!DateTime::is_valid_minute("60"));

    assert!(DateTime::is_valid_second("59"));
    assert!(!DateTime::is_valid_second("60"));

    assert!(DateTime::is_valid_microsecond("999999"));
    assert!(!DateTime::is_valid_microsecond("1000000"));

    assert!(DateTime::is_valid_iso_week("53"));
    assert!(!DateTime::is_valid_iso_week("54"));

    assert!(DateTime::is_valid_ordinal("366"));
    assert!(!DateTime::is_valid_ordinal("367"));

    // Time and date strings.
    assert!(DateTime::is_valid_time("23:59:59"));
    assert!(!DateTime::is_valid_time("24:00:00"));

    assert!(DateTime::is_valid_iso_8601("2024-01-15T10:30:00Z"));
    assert!(DateTime::is_valid_iso_8601("2024-01-15"));
    assert!(!DateTime::is_valid_iso_8601("2024-13-01"));

    // Validator/parser symmetry: if `is_valid_iso_8601` says yes,
    // `parse` will succeed, and vice versa.
    let s = "2024-01-15T10:30:00Z";
    assert_eq!(
        DateTime::is_valid_iso_8601(s),
        DateTime::parse(s).is_ok()
    );

    println!("✅ All validators behaved as expected");
}
