// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

use dtt::DateTime;

#[test]
fn test_datetime_coverage_endpoints() {
    let dt = DateTime::new();
    let new_dt = dt.set_time(10, 30, 0).unwrap();
    assert_eq!(new_dt.hour(), 10);

    // Date-only ISO 8601 parsing fallback execution
    assert!(DateTime::is_valid_iso_8601("2026-02-24"));

    // Total failure branch bypass
    assert!(!DateTime::is_valid_iso_8601("completely_invalid_string"));

    // Trigger `days_in_month` error via extreme invalid boundaries
    // We already do ComponentRange natively, but we trigger the manual mismatch here
    let res = DateTime::from_components(
        2024,
        15,
        1,
        0,
        0,
        0,
        time::UtcOffset::UTC,
    );
    assert!(res.is_err());
}
