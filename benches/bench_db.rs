// SPDX-License-Identifier: MIT OR Apache-2.0

use criterion::{black_box, criterion_group, criterion_main, Criterion};
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
