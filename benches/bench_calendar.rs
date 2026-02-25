// SPDX-License-Identifier: MIT OR Apache-2.0

use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
