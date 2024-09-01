// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! # DTT Library Usage Examples
//!
//! This program demonstrates the comprehensive usage of the DateTime (DTT) library, covering basic
//! and advanced DateTime operations, macro usage, date component handling, error handling
//! scenarios, serialization, and performance considerations. Each function provides examples with
//! explanatory output.

#![allow(missing_docs)]

use dtt::datetime::DateTime;
use dtt::{
    dtt_add_days, dtt_assert, dtt_join, dtt_map, dtt_max, dtt_min,
    dtt_now, dtt_parse, dtt_print_vec, dtt_sub_days, dtt_vec,
};
use std::error::Error;
use std::time::Instant;

/// Entry point for the DTT library usage examples.
///
/// This function orchestrates the execution of various example functions that demonstrate
/// the capabilities of the DTT library.
///
/// # Errors
///
/// Returns a boxed `dyn Error` in case any of the example functions fail.
fn main() -> Result<(), Box<dyn Error>> {
    println!("🦀 Comprehensive DTT Library Usage Examples 🦀\n");

    basic_datetime_examples()?;
    advanced_datetime_operations()?;
    macro_usage_examples()?;
    date_component_examples()?;
    error_handling_examples()?;
    serialization_examples()?;
    performance_examples()?;
    locale_specific_examples()?;

    println!("\n🎉 All examples completed successfully!");

    Ok(())
}

/// Demonstrates basic usage of the `DateTime` struct and associated methods.
///
/// This function covers creating DateTime objects, parsing dates, and formatting.
///
/// # Errors
///
/// Returns a boxed `dyn Error` if any DateTime parsing or creation fails.
fn basic_datetime_examples() -> Result<(), Box<dyn Error>> {
    println!("🦀 Basic DateTime Examples 🦀");

    let now = dtt_now!();
    println!("Current time (using macro): ✅ {:?}", now);

    let parsed_date = dtt_parse!("2023-05-20T15:30:00Z")?;
    println!("Parsed date (using macro): ✅ {:?}", parsed_date);

    let utc_date = DateTime::new();
    println!("UTC Date: ✅ {:?}", utc_date);

    let paris_time = DateTime::new_with_tz("CET")?;
    let tokyo_time = DateTime::new_with_tz("JST")?;
    println!("Paris time: ✅ {:?}", paris_time);
    println!("Tokyo time: ✅ {:?}", tokyo_time);

    let custom_offset_time = DateTime::new_with_custom_offset(5, 30)?;
    println!(
        "Custom offset time (UTC+5:30): ✅ {:?}",
        custom_offset_time
    );

    let rfc3339_date = DateTime::parse("2023-05-20T15:30:00Z")?;
    println!("Parsed RFC3339 date: ✅ {:?}", rfc3339_date);

    let custom_format_date = DateTime::parse_custom_format(
        "20/05/2023 15:30:00",
        "[day]/[month]/[year] [hour]:[minute]:[second]",
    )?;
    println!("Parsed custom format date: ✅ {:?}", custom_format_date);

    Ok(())
}

/// Demonstrates advanced operations using the `DateTime` struct.
///
/// This function includes examples of adding and subtracting days, working with durations,
/// and formatting dates in various formats.
///
/// # Errors
///
/// Returns a boxed `dyn Error` if any advanced operation fails.
fn advanced_datetime_operations() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Advanced DateTime Operations 🦀");

    let utc_date = DateTime::new();

    // Date arithmetic
    println!("\n📅 Date Arithmetic:");
    let future_date = dtt_add_days!(utc_date, 30)?;
    println!("Date after 30 days: ✅ {:?}", future_date);

    let past_date = dtt_sub_days!(utc_date, 15)?;
    println!("Date 15 days ago: ✅ {:?}", past_date);

    let duration = future_date.duration_since(&utc_date);
    let days_between = duration.whole_days();
    println!("Days between dates: ✅ {} days", days_between);

    // Date ranges and comparisons
    println!("\n📊 Date Ranges and Comparisons:");
    let start_date = DateTime::from_components(
        2023,
        1,
        1,
        0,
        0,
        0,
        utc_date.offset(),
    )?;
    let end_date = DateTime::from_components(
        2023,
        12,
        31,
        23,
        59,
        59,
        utc_date.offset(),
    )?;
    let check_date = DateTime::from_components(
        2023,
        6,
        15,
        12,
        0,
        0,
        utc_date.offset(),
    )?;
    let is_in_range =
        check_date.is_within_range(&start_date, &end_date);
    println!("Is 2023-06-15 within 2023? ✅ {}", is_in_range);

    // Timezone conversion
    println!("\n🌍 Timezone Conversion:");
    let nyc_time = utc_date.convert_to_tz("EST")?;
    println!("Current time in New York: ✅ {:?}", nyc_time);

    // Formatting
    println!("\n📝 Date Formatting:");
    let formatted_date = utc_date.format_rfc3339()?;
    println!("RFC3339 formatted date: ✅ {}", formatted_date);

    let iso8601_date = utc_date.format_iso8601()?;
    println!("ISO8601 formatted date: ✅ {}", iso8601_date);

    // Date ranges
    println!("\n📅 Date Ranges:");
    println!("Start of week: ✅ {:?}", utc_date.start_of_week()?);
    println!("End of week: ✅ {:?}", utc_date.end_of_week()?);
    println!("Start of month: ✅ {:?}", utc_date.start_of_month()?);
    println!("End of month: ✅ {:?}", utc_date.end_of_month()?);
    println!("Start of year: ✅ {:?}", utc_date.start_of_year()?);
    println!("End of year: ✅ {:?}", utc_date.end_of_year()?);

    Ok(())
}

/// Demonstrates the usage of custom macros provided by the DTT library.
///
/// This function includes examples of creating vectors, maps, assertions, and joining strings.
///
/// # Errors
///
/// Returns a boxed `dyn Error` if any macro operation fails.
#[allow(clippy::eq_op)]
fn macro_usage_examples() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Macro Usage Examples 🦀");

    let vec = dtt_vec![1, 2, 3, 4, 5];
    println!("Vector created with dtt_vec!: ✅ {:?}", vec);

    let map = dtt_map! {"a" => 1, "b" => 2, "c" => 3};
    println!("Map created with dtt_map!: ✅ {:?}", map);

    dtt_assert!(2 + 2 == 4, "Basic arithmetic assertion");
    println!("dtt_assert! passed");

    let min_value = dtt_min!(5, 3, 7, 1, 9);
    println!("Minimum value using dtt_min!: ✅ {}", min_value);

    let max_value = dtt_max!(5, 3, 7, 1, 9);
    println!("Maximum value using dtt_max!: ✅ {}", max_value);

    let joined_string = dtt_join!("Hello", " ", "World", "!");
    println!("Joined string using dtt_join!: ✅ {}", joined_string);

    println!("Printing vector using dtt_print_vec!:");
    dtt_print_vec!([1, 2, 3, 4, 5]);

    Ok(())
}

/// Demonstrates accessing and validating various components of a `DateTime` object.
///
/// This function covers retrieving date and time components and validating them.
///
/// # Errors
///
/// Returns a boxed `dyn Error` if any date component retrieval fails.
fn date_component_examples() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Date Component Examples 🦀");

    let date = DateTime::new();
    println!("\n📊 Date Components:");
    println!("Year: {}", date.year());
    println!("Month: {:?}", date.month());
    println!("Day: {}", date.day());
    println!("Hour: {}", date.hour());
    println!("Minute: {}", date.minute());
    println!("Second: {}", date.second());
    println!("Microsecond: {}", date.microsecond());
    println!("Weekday: {:?}", date.weekday());
    println!("Day of Year: {}", date.ordinal());
    println!("Week of Year: {}", date.iso_week());
    println!("Offset: {:?}", date.offset());

    println!("\n✅ Date Validation Examples:");
    println!("Is 31 a valid day? {}", DateTime::is_valid_day("31"));
    println!("Is 13 a valid month? {}", DateTime::is_valid_month("13"));
    println!("Is 25 a valid hour? {}", DateTime::is_valid_hour("25"));
    println!(
        "Is 2023-13-32 a valid ISO8601 date? {}",
        DateTime::is_valid_iso_8601("2023-13-32")
    );
    println!(
        "Is 61 a valid minute? {}",
        DateTime::is_valid_minute("61")
    );
    println!(
        "Is 60 a valid second? {}",
        DateTime::is_valid_second("60")
    );
    println!(
        "Is 2023 a valid year? {}",
        DateTime::is_valid_year("2023")
    );
    println!(
        "Is 1000000 a valid microsecond? {}",
        DateTime::is_valid_microsecond("1000000")
    );
    println!(
        "Is 366 a valid ordinal day? {}",
        DateTime::is_valid_ordinal("366")
    );
    println!(
        "Is 53 a valid ISO week? {}",
        DateTime::is_valid_iso_week("53")
    );
    println!(
        "Is 12:34:56 a valid time? {}",
        DateTime::is_valid_time("12:34:56")
    );

    Ok(())
}

/// Demonstrates error handling using the `DateTime` struct and associated methods.
///
/// This function covers scenarios where errors are expected and how they can be handled gracefully.
///
/// # Errors
///
/// Returns a boxed `dyn Error` if any unexpected behavior occurs during error handling examples.
fn error_handling_examples() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Error Handling Examples 🦀");

    // Invalid timezone
    match DateTime::new_with_tz("InvalidTZ") {
        Ok(_) => println!("Unexpected: InvalidTZ was accepted"),
        Err(e) => {
            println!("Expected error with invalid timezone: {}", e)
        }
    }

    // Invalid custom offset
    match DateTime::new_with_custom_offset(25, 0) {
        Ok(_) => println!("Unexpected: Invalid offset was accepted"),
        Err(e) => println!("Expected error with invalid offset: {}", e),
    }

    // Invalid date parsing
    match DateTime::parse("not-a-date") {
        Ok(_) => println!("Unexpected: Invalid date string was parsed"),
        Err(e) => {
            println!("Expected error parsing invalid date: {}", e)
        }
    }

    // Date overflow
    let max_date = DateTime::from_components(
        9999,
        12,
        31,
        23,
        59,
        59,
        DateTime::new().offset(),
    )?;
    match dtt_add_days!(max_date, 1) {
        Ok(_) => println!("Unexpected: Date overflow was allowed"),
        Err(e) => println!("Expected error with date overflow: {}", e),
    }

    // ISO 8601 Parsing Example
    println!("\n🦀 ISO 8601 Self-Parsing Example 🦀");

    let dt1 = dtt_now!();
    println!("Original dt1: {:?}", dt1);

    let iso8601_string = dt1.format_iso8601()?;
    println!("dt1 ISO 8601 string: {}", iso8601_string);

    match DateTime::parse(&iso8601_string) {
        Ok(dt2) => {
            println!("Parsed dt2: {:?}", dt2);
            println!(
                "Successfully parsed dt2 ISO 8601: {}",
                dt2.format_iso8601()?
            );
            println!("dt1 and dt2 are equal: {}", dt1 == dt2);
            println!(
                "Difference in microseconds: {}",
                dt1.duration_since(&dt2).whole_microseconds()
            );
            // Note: There might be a small difference due to microsecond precision
        }
        Err(e) => println!("Error parsing ISO 8601 string: {}", e),
    }

    Ok(())
}

/// Demonstrates serialization and deserialization of DateTime objects.
///
/// This function shows how to convert DateTime objects to and from JSON representations.
///
/// # Errors
///
/// Returns a boxed `dyn Error` if serialization or deserialization fails.
fn serialization_examples() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Serialization Examples 🦀");

    let dt = DateTime::new();

    // Serialization
    let serialized = serde_json::to_string(&dt)?;
    println!("Serialized DateTime: {}", serialized);

    // Deserialization
    let deserialized: DateTime = serde_json::from_str(&serialized)?;
    println!("Deserialized DateTime: {:?}", deserialized);

    // Verify equality
    println!(
        "Original and deserialized are equal: {}",
        dt == deserialized
    );

    Ok(())
}

/// Demonstrates performance considerations when using the DateTime library.
///
/// This function includes examples of measuring the performance of various DateTime operations.
///
/// # Errors
///
/// Returns a boxed `dyn Error` if any performance measurement fails.
fn performance_examples() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Performance Examples 🦀");

    // Measure creation performance
    let start = Instant::now();
    for _ in 0..10000 {
        let _ = DateTime::new();
    }
    let duration = start.elapsed();
    println!("Time to create 10,000 DateTime objects: {:?}", duration);

    // Measure parsing performance
    let start = Instant::now();
    for _ in 0..10000 {
        let _ = DateTime::parse("2023-09-01T12:00:00Z")?;
    }
    let duration = start.elapsed();
    println!("Time to parse 10,000 ISO8601 strings: {:?}", duration);

    // Measure formatting performance
    let dt = DateTime::new();
    let start = Instant::now();
    for _ in 0..10000 {
        let _ = dt.format_iso8601()?;
    }
    let duration = start.elapsed();
    println!(
        "Time to format 10,000 DateTime objects to ISO8601: {:?}",
        duration
    );

    // Measure arithmetic performance
    let dt = DateTime::new();
    let start = Instant::now();
    for i in 0..10000 {
        let _ = dt.add_days(i)?;
    }
    let duration = start.elapsed();
    println!("Time to perform 10,000 date additions: {:?}", duration);

    Ok(())
}

/// Demonstrates locale-specific formatting capabilities of the library.
///
/// This function provides information about the library's support (or lack thereof)
/// for locale-specific date formatting.
///
/// # Errors
///
/// This function does not return any errors as it only prints information.
fn locale_specific_examples() -> Result<(), Box<dyn Error>> {
    println!("\n🦀 Locale-Specific Formatting Information 🦀");

    println!("Note: The current version of the DateTime library does not support locale-specific formatting.");
    println!("For formatting dates, you can use the standard formatting methods like `format_iso8601` or `format_rfc3339`.");

    let dt = DateTime::new();
    println!("Example ISO 8601 format: {}", dt.format_iso8601()?);
    println!("Example RFC 3339 format: {}", dt.format_rfc3339()?);

    println!("\nIf you need locale-specific formatting, consider using additional libraries or implementing custom formatting functions.");

    Ok(())
}
