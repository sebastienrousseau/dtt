// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(feature = "db-sqlx")]
fn main() {
    use dtt::DateTime;
    println!("=== DTT SQLx Database Demo ===");
    let dt = DateTime::new();
    println!("DateTime ready for PostgreSQL parameterized struct injection!");
    println!("Time: {}", dt.format_rfc3339());
    println!("Note: sqlx Type, Encode, and Decode traits are natively attached.");
}

#[cfg(not(feature = "db-sqlx"))]
fn main() {
    println!("Please run this example with `--features db-sqlx` enabled.");
}
