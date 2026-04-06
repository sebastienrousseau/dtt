#![allow(missing_docs)]

// Arithmetic — date and time math, leap-year aware.

use dtt::prelude::*;

fn main() -> Result<(), AppError> {
    let dt = DateTime::parse("2024-01-31T00:00:00Z")?;
    println!("Start          : {dt}");

    println!("+ 1 day        : {}", dt.next_day()?);
    println!("- 1 day        : {}", dt.previous_day()?);
    println!("+ 7 days       : {}", dt.add_days(7)?);
    println!("+ 1 month      : {}", dt.add_months(1)?); // → Feb 29 (leap)
    println!("+ 1 year       : {}", dt.add_years(1)?);
    println!("- 1 year       : {}", dt.sub_years(1)?);

    println!("Start of week  : {}", dt.start_of_week()?);
    println!("End of week    : {}", dt.end_of_week()?);
    println!("Start of month : {}", dt.start_of_month()?);
    println!("End of month   : {}", dt.end_of_month()?);
    println!("Start of year  : {}", dt.start_of_year()?);
    println!("End of year    : {}", dt.end_of_year()?);

    let later = dt.add_days(10)?;
    let diff = later.duration_since(&dt);
    println!("Days between   : {}", diff.whole_days());

    Ok(())
}
