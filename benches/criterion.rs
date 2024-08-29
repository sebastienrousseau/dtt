// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#![allow(missing_docs)]
use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use dtt::datetime::DateTime;

fn new_date(c: &mut Criterion) {
    c.bench_function("new", |b| b.iter(|| DateTime::new));
}

fn new_day(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("day", move |b| b.iter(|| date.day));
}

fn new_hour(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("hour", move |b| b.iter(|| date.hour));
}

fn new_iso_8601(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("iso_8601", move |b| {
        b.iter(|| date.iso_8601.to_owned())
    });
}
fn new_iso_week(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("iso_week", move |b| b.iter(|| date.iso_week));
}
fn new_minute(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("minute", move |b| b.iter(|| date.minute));
}

fn new_month(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("month", move |b| {
        b.iter(|| date.month.to_owned())
    });
}
fn new_offset(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("offset", move |b| {
        b.iter(|| date.offset.to_owned())
    });
}
fn new_ordinal(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("ordinal", move |b| b.iter(|| date.ordinal));
}
fn new_second(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("second", move |b| b.iter(|| date.second));
}
fn new_time(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("time", move |b| b.iter(|| date.time.to_owned()));
}
fn new_weekday(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("weekday", move |b| {
        b.iter(|| date.weekday.to_owned())
    });
}

fn new_year(c: &mut Criterion) {
    let date = black_box(DateTime::new());
    c.bench_function("year", move |b| b.iter(|| date.year));
}

// Entry point for all benchmarks.
criterion_group!(
    benches,
    new_date,
    new_day,
    new_hour,
    new_iso_8601,
    new_iso_week,
    new_minute,
    new_month,
    new_offset,
    new_ordinal,
    new_second,
    new_time,
    new_weekday,
    new_year,
);

// Run benchmarks.
criterion_main!(benches);
