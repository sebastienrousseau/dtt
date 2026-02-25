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
fn main() {
    use dtt::DateTime;
    println!("=== DTT SQLx Database Demo ===");
    let dt = DateTime::new();
    println!(
        "DateTime ready for PostgreSQL parameterized struct injection!"
    );
    println!("Time: {}", dt.format_rfc3339().unwrap());
    println!("Note: sqlx Type, Encode, and Decode traits are natively attached.");
}

#[cfg(not(feature = "db-sqlx"))]
fn main() {
    println!(
        "Please run this example with `--features db-sqlx` enabled."
    );
}
