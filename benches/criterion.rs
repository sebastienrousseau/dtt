// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#![allow(missing_docs)]
//! Benchmarks for the DTT (DateTime) library.
//!
//! This module contains benchmarks for various operations provided by the DTT library,
//! including creation of DateTime instances and accessing different components of a DateTime.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dtt::datetime::DateTime;

/// Benchmark the creation of a new DateTime instance.
fn bench_new_datetime(c: &mut Criterion) {
    c.bench_function("new DateTime", |b| b.iter(DateTime::new));
}

/// Benchmark accessing the day of a DateTime instance.
fn bench_get_day(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get day", move |b| b.iter(|| date.day()));
}

/// Benchmark accessing the hour of a DateTime instance.
fn bench_get_hour(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get hour", move |b| b.iter(|| date.hour()));
}

/// Benchmark formatting a DateTime instance to ISO 8601.
fn bench_format_iso_8601(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("format ISO 8601", move |b| {
        b.iter(|| date.format_iso8601().unwrap())
    });
}

/// Benchmark accessing the ISO week of a DateTime instance.
fn bench_get_iso_week(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get ISO week", move |b| b.iter(|| date.iso_week()));
}

/// Benchmark accessing the minute of a DateTime instance.
fn bench_get_minute(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get minute", move |b| b.iter(|| date.minute()));
}

/// Benchmark accessing the month of a DateTime instance.
fn bench_get_month(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get month", move |b| b.iter(|| date.month()));
}

/// Benchmark accessing the offset of a DateTime instance.
fn bench_get_offset(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get offset", move |b| b.iter(|| date.offset()));
}

/// Benchmark accessing the ordinal day of a DateTime instance.
fn bench_get_ordinal(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get ordinal", move |b| b.iter(|| date.ordinal()));
}

/// Benchmark accessing the second of a DateTime instance.
fn bench_get_second(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get second", move |b| b.iter(|| date.second()));
}

/// Benchmark formatting the time of a DateTime instance.
fn bench_format_time(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("format time", move |b| {
        b.iter(|| date.format("[hour]:[minute]:[second]").unwrap())
    });
}

/// Benchmark accessing the weekday of a DateTime instance.
fn bench_get_weekday(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get weekday", move |b| b.iter(|| date.weekday()));
}

/// Benchmark accessing the year of a DateTime instance.
fn bench_get_year(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("get year", move |b| b.iter(|| date.year()));
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
);

// Entry point for running the benchmarks
criterion_main!(benches);