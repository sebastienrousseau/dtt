// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

//! # Example: Core `DateTime` Instantiation
//!
//! Demonstrates the creation of standard DateTime records using native system UTC
//! hooks and direct parameter overrides.

use dtt::DateTime;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== DTT Core Instantiation ===");

    // 1. Current UTC time (bypasses stdlib via native hooks)
    let now = DateTime::new();
    println!("1. Current UTC: {}", now.format_iso8601().unwrap());

    // 2. Current TAI time (Atomic Monotonic Time)
    let tai = DateTime::tai_now();
    println!("2. Current TAI: {}", tai.format_iso8601().unwrap());

    // 3. Current Time in specific Timezone (Zero-Copy Mmap lookup)
    let tokyo = DateTime::new_with_tz("Asia/Tokyo")?;
    println!(
        "3. Local time (Asia/Tokyo): {}",
        tokyo.format_iso8601().unwrap()
    );

    // 4. Current Time with hardcoded custom offset
    let custom_tz = DateTime::new_with_custom_offset(5, 30)?; // +05:30 (India)
    println!(
        "4. Local time (UTC+5:30): {}",
        custom_tz.format_iso8601().unwrap()
    );

    Ok(())
}
