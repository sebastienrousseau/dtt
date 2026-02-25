// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

use criterion::{
    black_box, criterion_group, criterion_main, Criterion,
};
use dtt::DateTime;

#[cfg(feature = "db-sqlx")]
use sqlx::{Postgres, Type};

fn bench_db_type_info(c: &mut Criterion) {
    #[cfg(feature = "db-sqlx")]
    {
        c.bench_function("db_sqlx_type_info", |b| {
            b.iter(|| {
                black_box(<DateTime as Type<Postgres>>::type_info());
            });
        });
    }
    #[cfg(not(feature = "db-sqlx"))]
    {
        let _ = c;
    }
}

criterion_group!(benches, bench_db_type_info);
criterion_main!(benches);
