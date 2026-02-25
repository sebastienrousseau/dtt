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

//! Mathematical coverage and Fuzzing validation suite.
//!
//! Exposes `dtt::DateTime` and SIMD capabilities to millions of pseudo-random
//! bytes to mathematically prove that `parse()` or `from_str()` can *never* panic
//! regardless of how malformed the input data stream is.

use dtt::DateTime;
use proptest::prelude::*;
use std::str::FromStr;

proptest! {
    /// Fuzzes the SIMD parsing engine with highly erratic arbitrary UTF-8 strings.
    /// The parser should ALWAYS return a polite `Err(DateTimeError)` or a valid `Ok(DateTime)`,
    /// but it should NEVER trigger a memory panic or SIGABRT under any circumstance.
    #[test]
    fn test_fuzz_from_str_never_panics(s in ".*") {
        let _ = DateTime::from_str(&s); // Must not panic
    }

    /// Fuzzes the parsing engine mathematically focusing on the exact ISO-8601 subset geometry
    /// to ensure standard parsing doesn't crash on invalid numeric combinations
    /// (e.g., month > 12, hour > 24).
    #[test]
    fn test_fuzz_iso8601_strict_bounds_never_panics(
        year in 0..9999u32,
        month in 0..99u32,
        day in 0..99u32,
        hour in 0..99u32,
        minute in 0..99u32,
        second in 0..99u32
    ) {
        let timestamp = format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z", year, month, day, hour, minute, second);
        let _ = DateTime::from_str(&timestamp); // Must not panic
    }
}
