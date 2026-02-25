// SPDX-License-Identifier: MIT OR Apache-2.0

use dtt::{calendar::CalendarDuration, DateTime};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== DTT Calendar Demo ===");
    let now = DateTime::new();
    println!("Current Time: {}", now.format_rfc3339());

    // Add 1 month, automatically handling boundary clipping
    let next_month = now.add_calendar(CalendarDuration::months(1))?;
    println!("Next Month:   {}", next_month.format_rfc3339());

    // Add 1 year
    let next_year = now.add_calendar(CalendarDuration::years(1))?;
    println!("Next Year:    {}", next_year.format_rfc3339());

    Ok(())
}
