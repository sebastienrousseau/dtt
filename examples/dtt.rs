// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#![allow(missing_docs)]

use dtt::datetime::DateTime;

pub fn main() {
    // Creating DateTime objects
    let date = DateTime::new();
    println!("🦀 UTC Date:          ✅ {:?}", date);

    // Creating DateTime with a specific timezone
    let paris_time = DateTime::new_with_tz("CET")
        .expect("Failed to create DateTime with CET timezone");
    println!("🦀 Paris time:        ✅ {:?}", paris_time);

    // Creating DateTime with a custom offset
    let custom_offset_time = DateTime::new_with_custom_offset(5, 30)
        .expect("Failed to create DateTime with custom offset");
    println!("🦀 Custom offset time: ✅ {:?}", custom_offset_time);

    // Parsing DateTime from a string (RFC3339 format)
    let parsed_date = DateTime::parse("2023-05-20T15:30:00Z")
        .expect("Failed to parse date");
    println!("🦀 Parsed RFC3339 date: ✅ {:?}", parsed_date);

    // Parsing DateTime from a string (Custom format)
    let custom_parsed_date = DateTime::parse_custom_format(
        "2023-05-20 15:30:00",
        "[year]-[month]-[day] [hour]:[minute]:[second]",
    )
    .expect("Failed to parse custom format date");
    println!("🦀 Parsed custom date: ✅ {:?}", custom_parsed_date);

    // Displaying individual components of the DateTime
    println!("🦀 Year:              ✅ {:?}", date.year());
    println!("🦀 Month:             ✅ {:?}", date.month());
    println!("🦀 Day:               ✅ {:?}", date.day());
    println!("🦀 Hour:              ✅ {:?}", date.hour());
    println!("🦀 Minute:            ✅ {:?}", date.minute());
    println!("🦀 Second:            ✅ {:?}", date.second());
    println!("🦀 Microsecond:       ✅ {:?}", date.microsecond());
    println!("🦀 Weekday:           ✅ {:?}", date.weekday());
    println!("🦀 Ordinal Date:      ✅ {:?}", date.ordinal());
    println!("🦀 ISO Week Number:   ✅ {:?}", date.iso_week());
    println!("🦀 Offset:            ✅ {:?}", date.offset());

    // Adding and subtracting days
    let future_date = date.add_days(7).expect("Adding days failed");
    println!("🦀 Date after 7 days: ✅ {:?}", future_date);
    let previous_day =
        date.previous_day().expect("Failed to get previous day");
    println!("🦀 Previous day:      ✅ {:?}", previous_day);
    let next_day = date.next_day().expect("Failed to get next day");
    println!("🦀 Next day:          ✅ {:?}", next_day);

    // Start and end of week, month, and year
    let start_of_week =
        date.start_of_week().expect("Failed to get start of week");
    println!("🦀 Start of the week: ✅ {:?}", start_of_week);
    let end_of_week =
        date.end_of_week().expect("Failed to get end of week");
    println!("🦀 End of the week:   ✅ {:?}", end_of_week);

    let start_of_month =
        date.start_of_month().expect("Failed to get start of month");
    println!("🦀 Start of the month: ✅ {:?}", start_of_month);
    let end_of_month =
        date.end_of_month().expect("Failed to get end of month");
    println!("🦀 End of the month:   ✅ {:?}", end_of_month);

    let start_of_year =
        date.start_of_year().expect("Failed to get start of year");
    println!("🦀 Start of the year: ✅ {:?}", start_of_year);
    let end_of_year =
        date.end_of_year().expect("Failed to get end of year");
    println!("🦀 End of the year:   ✅ {:?}", end_of_year);

    // Checking if a date is within a range
    let range_end_date = date.add_days(10).expect("Adding days failed");
    let is_within_range = date.is_within_range(&date, &range_end_date);
    println!("🦀 Is within range:   ✅ {:?}", is_within_range);

    // Duration between two dates
    let duration = date.duration_since(&previous_day);
    println!("🦀 Duration since:    ✅ {:?}", duration);

    // Converting to different timezones
    let converted_to_pst = date
        .convert_to_tz("PST")
        .expect("Failed to convert timezone");
    println!("🦀 Converted to PST:  ✅ {:?}", converted_to_pst);

    // Formatting DateTime
    let custom_format = date
        .format("[year]-[month]-[day] [hour]:[minute]:[second]")
        .expect("Failed to format date");
    println!("🦀 Custom formatted date: ✅ {}", custom_format);

    let rfc3339_format = date
        .format_rfc3339()
        .expect("Failed to format RFC3339 date");
    println!("🦀 RFC3339 formatted date: ✅ {}", rfc3339_format);

    let iso8601_format = date
        .format_iso8601()
        .expect("Failed to format ISO8601 date");
    println!("🦀 ISO8601 formatted date: ✅ {}", iso8601_format);
}
