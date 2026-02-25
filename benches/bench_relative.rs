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

use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use dtt::DateTime;
use time::Duration;

fn bench_relative_formatting(c: &mut Criterion) {
    let now = DateTime::new();
    let past = now.minus(Duration::days(5)).unwrap();
    let future = now.plus(Duration::hours(2)).unwrap();

    c.bench_function("relative_past_5_days", |b| {
        b.iter(|| {
            black_box(past.relative());
        });
    });

    c.bench_function("relative_future_2_hours", |b| {
        b.iter(|| {
            black_box(future.relative());
        });
    });
}

criterion_group!(benches, bench_relative_formatting);
criterion_main!(benches);
