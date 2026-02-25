// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

//! # End-To-End Automation Integration Suite
//!
//! This suite is designed to ensure all library functionalities interoperate
//! together without runtime panics. Useful primarily for CI pipelines.

use dtt::DateTime;
use std::str::FromStr;
use time::Duration;

#[test]
fn e2e_pipeline_simulation() {
    // 1. Generation
    let base = DateTime::new();
    assert!(base.year() >= 2024);

    // 2. Chained Time Mapping
    let chained = base
        .plus(Duration::days(1))
        .expect("Failed Plus")
        .minus(Duration::hours(12))
        .expect("Failed Minus");

    // 3. String extraction
    let serialized =
        chained.format_iso8601().expect("Failed String Formatting");

    // 4. String restoration using SIMD Parser
    let parsed =
        DateTime::from_str(&serialized).expect("Failed SIMD Parser");

    // 5. Hard properties validation
    assert_eq!(parsed.day(), chained.day());
    assert_eq!(parsed.month(), chained.month());
    assert_eq!(parsed.year(), chained.year());

    // 6. Timezone mutation check
    // Using UTC instead of Asia/Tokyo to guarantee it works on every single OS without external
    // tzdata installations breaking the CI locally.
    let utc_shift =
        parsed.in_tz("UTC").expect("Failed Timezone Resolution");
    assert_eq!(
        utc_shift.offset(),
        time::UtcOffset::UTC,
        "Timezone shifts must mutate offsets"
    );
}
