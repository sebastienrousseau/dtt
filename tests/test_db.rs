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

#[cfg(feature = "db-sqlx")]
mod db_tests {
    use dtt::DateTime;
    use sqlx::{Postgres, Type};

    #[test]
    fn test_type_info_and_compatibility() {
        let ty = <DateTime as Type<Postgres>>::type_info();

        // Assert dtt::DateTime natively maps to Postgres SQL types correctly
        assert!(<DateTime as Type<Postgres>>::compatible(&ty));
    }
}
