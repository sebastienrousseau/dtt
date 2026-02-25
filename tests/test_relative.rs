// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

use dtt::DateTime;
use time::Duration;

#[test]
fn test_relative_formats_past() {
    let now = DateTime::new();

    let past_1s = now.minus(Duration::seconds(1)).unwrap();
    assert_eq!(past_1s.relative(), "1 second ago");

    let past_5s = now.minus(Duration::seconds(5)).unwrap();
    assert_eq!(past_5s.relative(), "5 seconds ago");

    let past_1m = now.minus(Duration::minutes(1)).unwrap();
    assert_eq!(past_1m.relative(), "1 minute ago");

    let past_5m = now.minus(Duration::minutes(5)).unwrap();
    assert_eq!(past_5m.relative(), "5 minutes ago");

    let past_1h = now.minus(Duration::hours(1)).unwrap();
    assert_eq!(past_1h.relative(), "1 hour ago");

    let past_5h = now.minus(Duration::hours(5)).unwrap();
    assert_eq!(past_5h.relative(), "5 hours ago");

    let past_1d = now.minus(Duration::days(1)).unwrap();
    assert_eq!(past_1d.relative(), "1 day ago");

    let past_5d = now.minus(Duration::days(5)).unwrap();
    assert_eq!(past_5d.relative(), "5 days ago");

    let past_1mo = now.minus(Duration::days(31)).unwrap();
    assert_eq!(past_1mo.relative(), "1 month ago");

    let past_2mo = now.minus(Duration::days(60)).unwrap();
    assert_eq!(past_2mo.relative(), "2 months ago");

    let past_1y = now.minus(Duration::days(366)).unwrap();
    assert_eq!(past_1y.relative(), "1 year ago");

    let past_2y = now.minus(Duration::days(800)).unwrap();
    assert_eq!(past_2y.relative(), "2 years ago");
}

#[test]
fn test_relative_formats_future() {
    let now = DateTime::new();

    let future = now.plus(Duration::hours(5)).unwrap();
    let rel = future.relative();
    assert!(
        rel == "in 5 hours" || rel == "in 4 hours",
        "Future format strict assertions: {}",
        rel
    );

    let precise_now = DateTime::new();
    assert_eq!(precise_now.relative(), "Just now");
}
