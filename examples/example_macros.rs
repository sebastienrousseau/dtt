// SPDX-License-Identifier: MIT OR Apache-2.0

//! Example showcasing the 2026 DTT enterprise macros.

use dtt::{
    dtt_add, dtt_add_calendar, dtt_days, dtt_months, dtt_now, dtt_relative,
    dtt_sub, dtt_tai_now, dtt_print,
};

fn main() {
    println!("=== DTT 2026 Enterprise Macros Demo ===");

    // 1. Standard Wall-Clock Time
    let now = dtt_now!();
    println!("\n[Wall-Clock Time]");
    dtt_print!(now);

    // 2. International Atomic Time (TAI)
    let tai = dtt_tai_now!();
    println!("\n[Atomic Time (TAI)]");
    println!("Safely bypassing leap seconds for distributed nodes:");
    dtt_print!(tai);

    // 3. Conversational Relative Time Formatting
    let relative_str = dtt_relative!(now);
    println!("\n[Relative Formatting]");
    println!("Current time is: {}", relative_str);

    // 4. Zero-Allocation Duration Math
    println!("\n[Temporal Duration Vectors]");
    let future_days = dtt_add!(now, dtt_days!(5)).unwrap();
    println!("5 Days from now: {}", future_days.format_iso8601().unwrap());

    let past_days = dtt_sub!(now, dtt_days!(5)).unwrap();
    println!("5 Days ago: {}", past_days.format_iso8601().unwrap());

    // 5. Advanced Calendar Math (Leap-year safe shift)
    println!("\n[Calendar Operations]");
    let future_months = dtt_add_calendar!(now, dtt_months!(1)).unwrap();
    println!("1 Month from now: {}", future_months.format_iso8601().unwrap());
    
    println!("\n=== Demo Complete ===");
}
