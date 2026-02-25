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

//! Tests for the optional `tracing` integration.

#[cfg(feature = "tracing")]
use tracing::info;

#[cfg(feature = "tracing")]
#[test]
fn test_tracing_compilation() {
    // We only test that tracing macros compile correctly.
    // Full subscriber capture requires heavier mocked deps.
    info!("Testing tracing setup");
    let _dt = dtt::DateTime::new();
    let parsed = dtt::parse::parse_datetime(b"2026-02-24T12:00:00Z");
    assert!(parsed.is_ok());

    // Test timezone mmap
    #[cfg(unix)]
    {
        let _ = dtt::timezone::get_tz_offset("America/New_York");
    }
}
