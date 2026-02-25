// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

//! # Example: Fluent Builder API
//!
//! Demonstrates constructing and manipulating a datetime object using standard
//! chaining patterns similar to modern date architecture specs.

use dtt::DateTime;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== DTT Fluent Builder API ===");

    // We can chain methods directly on initialization
    let dt = DateTime::new().in_tz("America/New_York")?;

    println!("Base Time (NY): {}", dt.format_iso8601().unwrap());

    // The builder is immutable, yielding new copies via explicit additions/subtractions
    let next_week = dt.plus(time::Duration::days(7))?;
    println!("Next Week:      {}", next_week.format_iso8601().unwrap());

    let last_month = dt.minus(time::Duration::days(30))?;
    println!(
        "Last Month:     {}",
        last_month.format_iso8601().unwrap()
    );

    // Multi-chaining
    let complex = DateTime::new()
        .in_tz("Europe/Paris")?
        .plus(time::Duration::hours(12))?
        .minus(time::Duration::minutes(30))?;

    println!("Complex Target: {}", complex.format_iso8601().unwrap());

    Ok(())
}
