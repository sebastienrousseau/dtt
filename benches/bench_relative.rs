// SPDX-License-Identifier: MIT OR Apache-2.0

use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
