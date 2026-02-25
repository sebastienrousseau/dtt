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
use dtt::{calendar::CalendarDuration, DateTime};

fn bench_calendar_addition(c: &mut Criterion) {
    let dt = DateTime::new();
    let duration = CalendarDuration::months(5);

    c.bench_function("calendar_add_months", |b| {
        b.iter(|| {
            black_box(dt.add_calendar(black_box(duration))).unwrap();
        });
    });
}

criterion_group!(benches, bench_calendar_addition);
criterion_main!(benches);
