// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

//! # Ultimate Benchmarking Suite for DTT
//!
//! Exposes exact matrix metrics pitting the new brutalist implementation against
//! theoretical ceiling capacities to prove ultra-fast latency.

use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use dtt::DateTime;
use std::str::FromStr;

fn bench_instantiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Instantiation Matrix");

    group.bench_function("bench_new_utc", |b| {
        b.iter(|| DateTime::new())
    });
    group.bench_function("bench_tai_now", |b| {
        b.iter(|| DateTime::tai_now())
    });

    #[cfg(unix)]
    group.bench_function("bench_new_tz_mmap", |b| {
        b.iter(|| DateTime::new_with_tz("America/New_York"))
    });

    group.finish();
}

fn bench_simd_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("SIMD Parsing Matrix");
    let target = "2025-05-18T14:30:00Z";

    group.bench_function("bench_parse_iso8601", |b| {
        b.iter(|| DateTime::from_str(black_box(target)))
    });

    group.finish();
}

fn bench_mutations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fluency Mutations Matrix");
    let dt = DateTime::new();

    group.bench_function("bench_plus_duration", |b| {
        b.iter(|| dt.plus(time::Duration::minutes(5)))
    });

    group.bench_function("bench_chaining", |b| {
        b.iter(|| {
            dt.plus(time::Duration::days(1))
                .unwrap()
                .minus(time::Duration::hours(2))
                .unwrap()
                .in_tz("UTC")
        })
    });

    group.finish();
}

fn bench_relative(c: &mut Criterion) {
    let mut group = c.benchmark_group("Relative Output Matrix");
    let dt = DateTime::new().minus(time::Duration::hours(3)).unwrap();

    group.bench_function("bench_relative_3h", |b| {
        b.iter(|| dt.relative())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_instantiation,
    bench_simd_parsing,
    bench_mutations,
    bench_relative
);
criterion_main!(benches);
