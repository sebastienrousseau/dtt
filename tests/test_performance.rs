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

//! # Performance Assertions
//!
//! Validates standard logic latency stays strictly under 5 milliseconds to
//! ensure system-bypassing performance checks remain un-regressed.

use dtt::DateTime;
use std::time::Instant;

#[test]
fn test_latency_limits_core() {
    let start = Instant::now();

    // Create 10_000 objects utilizing VDSO hooks
    for _ in 0..10_000 {
        let _dt = DateTime::new();
    }

    let diff = start.elapsed();
    // 10 thousand generations should take under 50ms total, meaning << 5ms per op.
    // Tighter bound: Ensure the total time is less than 50 milliseconds.
    assert!(
        diff.as_millis() <= 50,
        "Performance Regression: 10,000 DateTime::new() generated in {}ms (Expected <= 50ms)",
        diff.as_millis()
    );
}

#[cfg(unix)]
#[test]
fn test_latency_limits_mmap_tz() {
    let start = Instant::now();

    // Perform 1000 ZoneInfo Hot-loads utilizing memory mapping
    for _ in 0..1000 {
        let _ = DateTime::new_with_tz("America/New_York");
    }

    let diff = start.elapsed();
    assert!(
        diff.as_millis() <= 50,
        "Performance Regression: 1,000 Memory-Mapped TZ Loads in {}ms (Expected <= 50ms)",
        diff.as_millis()
    );
}
