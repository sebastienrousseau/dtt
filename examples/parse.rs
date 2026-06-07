#![allow(missing_docs)]

// Parse — minimalist demo of every supported parsing path.

use dtt::prelude::*;

fn main() -> Result<(), AppError> {
    // RFC 3339 with explicit offset.
    let utc = DateTime::parse("2024-01-15T10:30:00Z")?;
    println!("RFC 3339 UTC      : {utc}");

    // RFC 3339 with non-UTC offset (preserved).
    let mumbai = DateTime::parse("2024-01-15T10:30:00+05:30")?;
    println!("RFC 3339 +05:30   : {mumbai}");

    // ISO 8601 date-only (defaults to midnight UTC).
    let date_only = DateTime::parse("2024-01-15")?;
    println!("ISO 8601 date-only: {date_only}");

    // Custom format.
    let custom = DateTime::parse_custom_format(
        "15/01/2024 10:30:00",
        "[day]/[month]/[year] [hour]:[minute]:[second]",
    )?;
    println!("Custom format     : {custom}");

    // Round-trip guarantee.
    let s = utc.format_rfc3339()?;
    let utc2 = DateTime::parse(&s)?;
    assert_eq!(utc, utc2);
    println!("Round-trip OK     : {s}");

    Ok(())
}
