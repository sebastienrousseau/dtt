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

//! # Example: SIMD Accelerated Parse Validation
//!
//! Demonstrates converting serialized string payloads back into safe
//! structured `DateTime` modules leveraging strict 256-bit SIMD lookups.

use dtt::DateTime;
use std::error::Error;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== DTT SIMD Parser ===");

    let str_target = "2025-05-18T14:30:00Z";
    println!("Input String: {}", str_target);

    // Dtt overrides FromStr using the SIMD core logic automatically
    let parsed_dt = DateTime::from_str(str_target)?;

    println!("Parsed Object:  {}", parsed_dt.format_iso8601().unwrap());
    println!("Extracted Year: {}", parsed_dt.year());
    println!("Extracted Hour: {}", parsed_dt.hour());

    // Handling Invalid strings (Diagnostics available)
    let bad_target = "This is not a date";
    match DateTime::from_str(bad_target) {
        Ok(_) => println!("Should not succeed!"),
        Err(e) => println!("Caught invalid payload correctly: {}", e),
    }

    Ok(())
}
