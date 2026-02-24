// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Example: Complex Timezones
//!
//! Demonstrates evaluating cross-continental datetime structures against the Zero-Copy
//! Posix memory-mapped timezone offsets.

use dtt::DateTime;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== DTT Multi-Timezone Evaluations ===");

    // Initialize root context in UTC
    let source = DateTime::new();
    println!("Global Source: {}", source.format_iso8601().unwrap());

    // Construct localized contextual copies using strict tz strings
    let zones = vec![
        "Europe/London",
        "Europe/Paris",
        "America/New_York",
        "America/Los_Angeles",
        "Asia/Tokyo",
        "Australia/Sydney",
    ];

    println!("\nProjecting to targets:");
    for tz in zones {
        // Will throw an error if the host OS lacks standard tzdb info or mispelled
        match source.in_tz(tz) {
            Ok(local) => println!(
                "  {:<20} -> {}",
                tz,
                local.format_iso8601().unwrap()
            ),
            Err(e) => println!("  {:<20} -> Failed lookup: {}", tz, e),
        }
    }

    Ok(())
}
