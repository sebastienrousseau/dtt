// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used
)]
#![allow(
    missing_docs,
    unused_must_use,
    unused_results,
    unused_variables,
    dead_code
)]

//! # Example: Output Formatting
//!
//! Demonstrates strictly transforming the `DateTime` instance out
//! into arbitrary structured payload variants.

use dtt::DateTime;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== DTT Output Formatting ===");

    let dt = DateTime::new();

    // 1. Strictly RFC 3339 Compliant
    println!("RFC 3339: {}", dt.format("%Y-%m-%dT%H:%M:%S%z").unwrap());

    // 2. Human Readable Date Formats
    println!("Human Date: {}", dt.format("%B %d, %Y").unwrap());

    // 3. Human Readable Time Formats
    println!("Human Time: {}", dt.format("%I:%M %p").unwrap());

    // 4. Custom/Complex string mapping
    println!(
        "Custom String: {}",
        dt.format("Log Entry @ %Y/%m/%d - [%Hh %Mm %Ss]").unwrap()
    );

    // 5. Day name and Week day evaluation
    println!(
        "Contextual: it is {} ({}), ordinal Day {}",
        dt.weekday(),
        dt.month(),
        dt.ordinal()
    );

    Ok(())
}
