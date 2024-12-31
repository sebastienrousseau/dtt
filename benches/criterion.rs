#![allow(missing_docs)]
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Enhanced benchmarks for the DTT (`DateTime`) library.
//!
//! This module contains benchmarks for various operations provided by the DTT library,
//! including creation, parsing, formatting, arithmetic operations, comparisons, and timezone conversions.

use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use dtt::datetime::DateTime;
use time::Duration;

/// Benchmark the creation of a new `DateTime` instance.
fn bench_new_datetime(c: &mut Criterion) {
    let _ =
        c.bench_function("new `DateTime`", |b| b.iter(DateTime::new));
}

/// Benchmark accessing the day of a `DateTime` instance.
fn bench_get_day(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("get day", |b| {
        b.iter(|| {
            let _ = date.day();
        })
    });
}

/// Benchmark accessing the hour of a `DateTime` instance.
fn bench_get_hour(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("get hour", |b| {
        b.iter(|| {
            let _ = date.hour();
        })
    });
}

/// Benchmark formatting a `DateTime` instance to ISO 8601.
fn bench_format_iso_8601(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("format ISO 8601", |b| {
        b.iter(|| {
            if let Ok(output) = date.format_iso8601() {
                let _ = output;
            }
        })
    });
}

/// Benchmark parsing a datetime string in ISO 8601 format.
fn bench_parse_iso_8601(c: &mut Criterion) {
    let date_str = black_box("2023-09-01T12:00:00Z");
    let _ = c.bench_function("parse ISO 8601", |b| {
        b.iter(|| {
            if let Ok(date) = DateTime::parse(date_str) {
                let _ = date;
            }
        })
    });
}

/// Benchmark parsing a datetime string with a custom format.
fn bench_parse_custom_format(c: &mut Criterion) {
    let date_str = black_box("01/09/2023 12:00:00");
    let format =
        black_box("[day]/[month]/[year] [hour]:[minute]:[second]");
    let _ = c.bench_function("parse custom format", |b| {
        b.iter(|| {
            if let Ok(date) =
                DateTime::parse_custom_format(date_str, format)
            {
                let _ = date;
            }
        })
    });
}

/// Benchmark adding days to a `DateTime` instance.
fn bench_add_days(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("add days", |b| {
        b.iter(|| {
            if let Ok(new_date) = date.add_days(30) {
                let _ = new_date;
            }
        })
    });
}

/// Benchmark subtracting days from a `DateTime` instance.
fn bench_sub_days(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("subtract days", |b| {
        b.iter(|| {
            if let Ok(new_date) = date.add_days(-30) {
                let _ = new_date;
            }
        })
    });
}

/// Benchmark adding a `Duration` to a `DateTime` instance.
fn bench_add_duration(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let duration = black_box(Duration::hours(48));
    let _ = c.bench_function("add duration", |b| {
        b.iter(|| {
            if let Ok(new_date) = date + duration {
                let _ = new_date;
            }
        })
    });
}

/// Benchmark comparing two `DateTime` instances.
fn bench_compare_datetimes(c: &mut Criterion) {
    let date1 = black_box(DateTime::new());
    let date2 = match date1.add_days(1) {
        Ok(date) => date,
        Err(_) => return,
    };
    let _ = c.bench_function("compare datetimes", |b| {
        b.iter(|| {
            let _ = date1.cmp(&date2);
        })
    });
}

/// Benchmark creating a `DateTime` for an extreme date (far in the future).
fn bench_extreme_date_future(c: &mut Criterion) {
    let _ = c.bench_function("create extreme future date", |b| {
        b.iter(|| {
            if let Ok(date) = DateTime::from_components(
                9999,
                12,
                31,
                23,
                59,
                59,
                time::UtcOffset::UTC,
            ) {
                let _ = date;
            }
        })
    });
}

/// Benchmark creating a `DateTime` for an extreme date (far in the past).
fn bench_extreme_date_past(c: &mut Criterion) {
    let _ = c.bench_function("create extreme past date", |b| {
        b.iter(|| {
            if let Ok(date) = DateTime::from_components(
                1,
                1,
                1,
                0,
                0,
                0,
                time::UtcOffset::UTC,
            ) {
                let _ = date;
            }
        })
    });
}

/// Benchmark converting a `DateTime` to a different timezone.
fn bench_convert_timezone(c: &mut Criterion) {
    let date = match DateTime::new_with_tz("UTC") {
        Ok(date) => date,
        Err(_) => return,
    };
    let _ = c.bench_function("convert timezone", |b| {
        b.iter(|| {
            if let Ok(new_date) = date.convert_to_tz("EST") {
                let _ = new_date;
            }
        })
    });
}

// Group all benchmarks.
criterion_group!(
    benches,
    bench_new_datetime,
    bench_get_day,
    bench_get_hour,
    bench_format_iso_8601,
    bench_parse_iso_8601,
    bench_parse_custom_format,
    bench_add_days,
    bench_sub_days,
    bench_add_duration,
    bench_compare_datetimes,
    bench_extreme_date_future,
    bench_extreme_date_past,
    bench_convert_timezone,
);

// Entry point for running the benchmarks.
criterion_main!(benches);
