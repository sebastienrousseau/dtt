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
use dtt::{
    dtt_days, dtt_hours, dtt_minutes, dtt_months, dtt_now,
    dtt_relative, dtt_tai_now, dtt_years,
};

fn bench_macro_generation(c: &mut Criterion) {
    c.bench_function("macro_dtt_now", |b| {
        b.iter(|| {
            black_box(dtt_now!());
        });
    });

    c.bench_function("macro_dtt_tai_now", |b| {
        b.iter(|| {
            black_box(dtt_tai_now!());
        });
    });

    c.bench_function("macro_duration_helpers", |b| {
        b.iter(|| {
            let _ = black_box(dtt_days!(5));
            let _ = black_box(dtt_hours!(2));
            let _ = black_box(dtt_minutes!(10));
            let _ = black_box(dtt_months!(1));
            let _ = black_box(dtt_years!(2));
        });
    });

    let dt = dtt_now!();
    c.bench_function("macro_dtt_relative", |b| {
        b.iter(|| {
            black_box(dtt_relative!(dt));
        });
    });
}

criterion_group!(benches, bench_macro_generation);
criterion_main!(benches);
