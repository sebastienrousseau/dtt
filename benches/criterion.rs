// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(missing_docs)]
//! Enhanced benchmarks for the DTT (DateTime) library.
//!
//! This module contains an expanded set of benchmarks for various operations provided by the DTT library,
//! including creation, parsing, formatting, arithmetic operations, comparisons, and timezone conversions.

use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use dtt::datetime::DateTime;
use time::Duration;

// Existing benchmarks

/// Benchmark the creation of a new DateTime instance.
fn bench_new_datetime(c: &mut Criterion) {
    let _ = c.bench_function("new DateTime", |b| b.iter(DateTime::new));
}

/// Benchmark accessing the day of a DateTime instance.
fn bench_get_day(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("get day", move |b| b.iter(|| date.day()));
}

/// Benchmark accessing the hour of a DateTime instance.
fn bench_get_hour(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ =
        c.bench_function("get hour", move |b| b.iter(|| date.hour()));
}

/// Benchmark formatting a DateTime instance to ISO 8601.
fn bench_format_iso_8601(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("format ISO 8601", move |b| {
        b.iter(|| date.format_iso8601().unwrap())
    });
}

/// Benchmark accessing the ISO week of a DateTime instance.
fn bench_get_iso_week(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("get ISO week", move |b| {
        b.iter(|| date.iso_week())
    });
}

/// Benchmark accessing the minute of a DateTime instance.
fn bench_get_minute(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("get minute", move |b| {
        b.iter(|| date.minute())
    });
}

/// Benchmark accessing the month of a DateTime instance.
fn bench_get_month(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ =
        c.bench_function("get month", move |b| b.iter(|| date.month()));
}

/// Benchmark accessing the offset of a DateTime instance.
fn bench_get_offset(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("get offset", move |b| {
        b.iter(|| date.offset())
    });
}

/// Benchmark accessing the ordinal day of a DateTime instance.
fn bench_get_ordinal(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("get ordinal", move |b| {
        b.iter(|| date.ordinal())
    });
}

/// Benchmark accessing the second of a DateTime instance.
fn bench_get_second(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("get second", move |b| {
        b.iter(|| date.second())
    });
}

/// Benchmark formatting the time of a DateTime instance.
fn bench_format_time(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("format time", move |b| {
        b.iter(|| date.format("[hour]:[minute]:[second]").unwrap())
    });
}

/// Benchmark accessing the weekday of a DateTime instance.
fn bench_get_weekday(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("get weekday", move |b| {
        b.iter(|| date.weekday())
    });
}

/// Benchmark accessing the year of a DateTime instance.
fn bench_get_year(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ =
        c.bench_function("get year", move |b| b.iter(|| date.year()));
}

// New benchmarks

/// Benchmark parsing a datetime string in ISO 8601 format.
fn bench_parse_iso_8601(c: &mut Criterion) {
    let date_str = black_box("2023-09-01T12:00:00Z");
    let _ = c.bench_function("parse ISO 8601", move |b| {
        b.iter(|| DateTime::parse(date_str).unwrap())
    });
}

/// Benchmark parsing a datetime string with a custom format.
fn bench_parse_custom_format(c: &mut Criterion) {
    let date_str = black_box("01/09/2023 12:00:00");
    let format =
        black_box("[day]/[month]/[year] [hour]:[minute]:[second]");
    let _ = c.bench_function("parse custom format", move |b| {
        b.iter(|| {
            DateTime::parse_custom_format(date_str, format).unwrap()
        })
    });
}

/// Benchmark adding days to a DateTime instance.
fn bench_add_days(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("add days", move |b| {
        b.iter(|| date.add_days(30).unwrap())
    });
}

/// Benchmark subtracting days from a DateTime instance.
fn bench_sub_days(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let _ = c.bench_function("subtract days", move |b| {
        b.iter(|| date.add_days(-30).unwrap())
    });
}

/// Benchmark adding a Duration to a DateTime instance.
fn bench_add_duration(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    let duration = black_box(Duration::hours(48));
    let _ = c.bench_function("add duration", move |b| {
        b.iter(|| (date + duration).unwrap())
    });
}

/// Benchmark comparing two DateTime instances.
fn bench_compare_datetimes(c: &mut Criterion) {
    let date1 = black_box(DateTime::new());
    let date2 = black_box(date1.add_days(1).unwrap());
    let _ = c.bench_function("compare datetimes", move |b| {
        b.iter(|| date1.cmp(&date2))
    });
}

/// Benchmark converting a DateTime to a different timezone.
fn bench_convert_timezone(c: &mut Criterion) {
    let date = black_box(DateTime::new_with_tz("UTC").unwrap());
    let _ = c.bench_function("convert timezone", move |b| {
        b.iter(|| date.convert_to_tz("EST").unwrap())
    });
}

/// Benchmark creating a DateTime for an extreme date (far in the future).
fn bench_extreme_date_future(c: &mut Criterion) {
    let _ = c.bench_function("create extreme future date", move |b| {
        b.iter(|| {
            DateTime::from_components(
                9999,
                12,
                31,
                23,
                59,
                59,
                time::UtcOffset::UTC,
            )
            .unwrap()
        })
    });
}

/// Benchmark creating a DateTime for an extreme date (far in the past).
fn bench_extreme_date_past(c: &mut Criterion) {
    let _ = c.bench_function("create extreme past date", move |b| {
        b.iter(|| {
            DateTime::from_components(
                1,
                1,
                1,
                0,
                0,
                0,
                time::UtcOffset::UTC,
            )
            .unwrap()
        })
    });
}

// Group all benchmarks
criterion_group!(
    benches,
    bench_new_datetime,
    bench_get_day,
    bench_get_hour,
    bench_format_iso_8601,
    bench_get_iso_week,
    bench_get_minute,
    bench_get_month,
    bench_get_offset,
    bench_get_ordinal,
    bench_get_second,
    bench_format_time,
    bench_get_weekday,
    bench_get_year,
    bench_parse_iso_8601,
    bench_parse_custom_format,
    bench_add_days,
    bench_sub_days,
    bench_add_duration,
    bench_compare_datetimes,
    bench_convert_timezone,
    bench_extreme_date_future,
    bench_extreme_date_past,
);

// Entry point for running the benchmarks
criterion_main!(benches);
