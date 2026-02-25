// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

use dtt::parse::parse_datetime;

#[test]
fn test_parse_datetime_fallback_short() {
    let res = parse_datetime(b"2026-02-24");
    assert!(res.is_ok());
}

#[test]
fn test_parse_datetime_fallback_no_t() {
    // Length >= 19, but invalid separator -> Hits line 71
    let _ = parse_datetime(b"2026-02-24 12-00-00");
}

#[test]
fn test_parse_datetime_fallback_len_18() {
    // Length > 10 and Length < 19 -> Falls through to line 104
    let _ = parse_datetime(b"2026-02-24T12:00:0");
}
