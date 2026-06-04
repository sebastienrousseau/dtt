#![allow(missing_docs)]

// Builder — the ergonomic, fluent way to construct a `DateTime`.
//
// `DateTimeBuilder` is the alternative to `DateTime::from_components`
// when you want to assemble components incrementally or partially.

use dtt::prelude::*;
use time::UtcOffset;

fn main() -> Result<(), AppError> {
    // ------------------------------------------------------------------
    // 1. Default — equivalent to `1970-01-01T00:00:00+00:00` (Unix epoch).
    // ------------------------------------------------------------------
    let epoch = DateTimeBuilder::default().build()?;
    println!("Epoch          : {epoch}");
    assert_eq!(epoch, DateTime::default());

    // ------------------------------------------------------------------
    // 2. Fully-specified construction.
    // ------------------------------------------------------------------
    let new_year = DateTimeBuilder::new()
        .year(2026)
        .month(1)
        .day(1)
        .hour(0)
        .minute(0)
        .second(0)
        .offset(UtcOffset::UTC)
        .build()?;
    println!("New Year UTC   : {new_year}");

    // ------------------------------------------------------------------
    // 3. Partial — unset fields keep their `new()` defaults
    //    (year=1970, month=1, day=1, time=midnight, offset=UTC).
    // ------------------------------------------------------------------
    let just_year = DateTimeBuilder::new().year(2030).build()?;
    println!("Year-only      : {just_year}"); // 2030-01-01T00:00:00Z

    // ------------------------------------------------------------------
    // 4. Custom offset.
    // ------------------------------------------------------------------
    let ist = DateTimeBuilder::new()
        .year(2024)
        .month(8)
        .day(15)
        .hour(10)
        .minute(30)
        .second(0)
        .offset(UtcOffset::from_hms(5, 30, 0).expect("valid offset"))
        .build()?;
    println!("IST midday     : {ist}");

    // ------------------------------------------------------------------
    // 5. Error path — invalid day surfaces as `InvalidDate`.
    // ------------------------------------------------------------------
    let bad = DateTimeBuilder::new()
        .year(2024)
        .month(2)
        .day(30) // Feb 30 — does not exist
        .build();
    assert!(matches!(bad, Err(DateTimeError::InvalidDate)));
    println!("Invalid Feb 30 : rejected as expected");

    // ------------------------------------------------------------------
    // 6. Builder is `Copy + Clone` — branch a base then specialise.
    // ------------------------------------------------------------------
    let base = DateTimeBuilder::new().year(2024).month(6).day(1);
    let morning = base.hour(8).minute(0).build()?;
    let evening = base.hour(20).minute(0).build()?;
    println!("Same date, AM  : {morning}");
    println!("Same date, PM  : {evening}");
    assert_eq!(morning.day(), evening.day());

    Ok(())
}
