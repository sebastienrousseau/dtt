#![allow(missing_docs)]
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Criterion benchmarks for the DTT (`DateTime`) library.
//!
//! Every public method on `DateTime`, `DateTimeBuilder`, and the
//! `dtt::datetime` free helpers has at least one bench. Trivial
//! `const fn` accessors (`year`, `month`, ..., `weekday`) are grouped
//! into one batched bench (`accessors_all`) because the per-call cost
//! is below criterion's measurement floor.

use criterion::{criterion_group, criterion_main, Criterion};
use dtt::datetime::{
    days_in_month, is_leap_year, DateTime, DateTimeBuilder,
};
use std::hint::black_box;
use time::{Duration, UtcOffset};

// ---------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------

fn bench_new(c: &mut Criterion) {
    let _ = c
        .bench_function("new (current UTC)", |b| b.iter(DateTime::new));
}

fn bench_new_with_tz(c: &mut Criterion) {
    let _ = c.bench_function("new_with_tz (EST_USA)", |b| {
        b.iter(|| {
            let _ = DateTime::new_with_tz(black_box("EST_USA"));
        });
    });
}

fn bench_new_with_custom_offset(c: &mut Criterion) {
    let _ = c.bench_function("new_with_custom_offset (+05:30)", |b| {
        b.iter(|| {
            let _ = DateTime::new_with_custom_offset(
                black_box(5),
                black_box(30),
            );
        });
    });
}

fn bench_from_components(c: &mut Criterion) {
    let _ = c.bench_function("from_components", |b| {
        b.iter(|| {
            let _ = DateTime::from_components(
                black_box(2024),
                black_box(1),
                black_box(15),
                black_box(10),
                black_box(30),
                black_box(0),
                UtcOffset::UTC,
            );
        });
    });
}

fn bench_default(c: &mut Criterion) {
    let _ = c.bench_function("default (Unix epoch)", |b| {
        b.iter(DateTime::default);
    });
}

fn bench_builder(c: &mut Criterion) {
    let _ = c.bench_function("DateTimeBuilder::build", |b| {
        b.iter(|| {
            DateTimeBuilder::new()
                .year(black_box(2024))
                .month(black_box(1))
                .day(black_box(15))
                .hour(black_box(10))
                .minute(black_box(30))
                .second(black_box(0))
                .offset(UtcOffset::UTC)
                .build()
        });
    });
}

// ---------------------------------------------------------------------
// Extreme dates
// ---------------------------------------------------------------------

fn bench_extreme_date_future(c: &mut Criterion) {
    let _ = c.bench_function("from_components (year 9999)", |b| {
        b.iter(|| {
            DateTime::from_components(
                black_box(9999),
                12,
                31,
                23,
                59,
                59,
                UtcOffset::UTC,
            )
        });
    });
}

fn bench_extreme_date_past(c: &mut Criterion) {
    let _ = c.bench_function("from_components (year 1)", |b| {
        b.iter(|| {
            DateTime::from_components(
                black_box(1),
                1,
                1,
                0,
                0,
                0,
                UtcOffset::UTC,
            )
        });
    });
}

// ---------------------------------------------------------------------
// Accessors — batched (each is `#[inline] const fn` returning a u8/u16/i32)
// ---------------------------------------------------------------------

fn bench_accessors_all(c: &mut Criterion) {
    let dt = black_box(
        DateTime::from_components(
            2024,
            6,
            15,
            12,
            30,
            45,
            UtcOffset::UTC,
        )
        .expect("fixed"),
    );
    let _ = c.bench_function("accessors (all 12)", |b| {
        b.iter(|| {
            let _ = dt.year();
            let _ = dt.month();
            let _ = dt.day();
            let _ = dt.hour();
            let _ = dt.minute();
            let _ = dt.second();
            let _ = dt.microsecond();
            let _ = dt.iso_week();
            let _ = dt.iso_year();
            let _ = dt.ordinal();
            let _ = dt.offset();
            let _ = dt.weekday();
        });
    });
}

fn bench_unix_timestamp(c: &mut Criterion) {
    let dt = black_box(DateTime::default());
    let _ = c.bench_function("unix_timestamp", |b| {
        b.iter(|| dt.unix_timestamp());
    });
}

// ---------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------

fn bench_parse_rfc3339(c: &mut Criterion) {
    let s = black_box("2024-01-15T10:30:00Z");
    let _ = c.bench_function("parse (RFC 3339)", |b| {
        b.iter(|| DateTime::parse(s));
    });
}

fn bench_parse_iso_8601_date(c: &mut Criterion) {
    let s = black_box("2024-01-15");
    let _ = c.bench_function("parse (ISO 8601 date-only)", |b| {
        b.iter(|| DateTime::parse(s));
    });
}

fn bench_parse_custom_format(c: &mut Criterion) {
    let s = black_box("15/01/2024 10:30:00");
    let fmt =
        black_box("[day]/[month]/[year] [hour]:[minute]:[second]");
    let _ = c.bench_function("parse_custom_format", |b| {
        b.iter(|| DateTime::parse_custom_format(s, fmt));
    });
}

// ---------------------------------------------------------------------
// Formatting
// ---------------------------------------------------------------------

fn bench_format_rfc3339(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("format_rfc3339", |b| {
        b.iter(|| dt.format_rfc3339());
    });
}

fn bench_format_custom(c: &mut Criterion) {
    let dt = DateTime::default();
    let fmt = black_box("[year]-[month]-[day]");
    let _ = c.bench_function("format (custom)", |b| {
        b.iter(|| dt.format(fmt));
    });
}

fn bench_format_time_in_timezone(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("format_time_in_timezone", |b| {
        b.iter(|| {
            dt.format_time_in_timezone(
                black_box("EST_USA"),
                black_box("[hour]:[minute]:[second]"),
            )
        });
    });
}

// ---------------------------------------------------------------------
// Arithmetic
// ---------------------------------------------------------------------

fn bench_add_days(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("add_days(30)", |b| {
        b.iter(|| dt.add_days(black_box(30)));
    });
}

fn bench_add_days_negative(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("add_days(-30) (subtract)", |b| {
        b.iter(|| dt.add_days(black_box(-30)));
    });
}

fn bench_next_previous_day(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("next_day + previous_day", |b| {
        b.iter(|| {
            let _ = dt.next_day();
            let _ = dt.previous_day();
        });
    });
}

fn bench_add_months(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("add_months(6)", |b| {
        b.iter(|| dt.add_months(black_box(6)));
    });
}

fn bench_sub_months(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("sub_months(6)", |b| {
        b.iter(|| dt.sub_months(black_box(6)));
    });
}

fn bench_add_years(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("add_years(5)", |b| {
        b.iter(|| dt.add_years(black_box(5)));
    });
}

fn bench_sub_years(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("sub_years(5)", |b| {
        b.iter(|| dt.sub_years(black_box(5)));
    });
}

fn bench_add_duration(c: &mut Criterion) {
    let dt = DateTime::default();
    let d = Duration::hours(48);
    let _ = c.bench_function("Add<Duration>", |b| {
        b.iter(|| dt + d);
    });
}

fn bench_sub_duration(c: &mut Criterion) {
    let dt = DateTime::default();
    let d = Duration::hours(48);
    let _ = c.bench_function("Sub<Duration>", |b| {
        b.iter(|| dt - d);
    });
}

// ---------------------------------------------------------------------
// Mutation helpers
// ---------------------------------------------------------------------

fn bench_set_time(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("set_time", |b| {
        b.iter(|| {
            dt.set_time(black_box(12), black_box(30), black_box(45))
        });
    });
}

fn bench_set_date(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("set_date", |b| {
        b.iter(|| {
            dt.set_date(black_box(2024), black_box(6), black_box(15))
        });
    });
}

fn bench_update(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("update", |b| {
        b.iter(|| dt.update());
    });
}

// ---------------------------------------------------------------------
// Calendar boundaries
// ---------------------------------------------------------------------

fn bench_calendar_boundaries(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("start_of/end_of week|month|year", |b| {
        b.iter(|| {
            let _ = dt.start_of_week();
            let _ = dt.end_of_week();
            let _ = dt.start_of_month();
            let _ = dt.end_of_month();
            let _ = dt.start_of_year();
            let _ = dt.end_of_year();
        });
    });
}

// ---------------------------------------------------------------------
// Comparison & range
// ---------------------------------------------------------------------

fn bench_compare(c: &mut Criterion) {
    let a = DateTime::default();
    let b_ = a.add_days(1).expect("fixed");
    let _ = c.bench_function("Ord::cmp", |bencher| {
        bencher.iter(|| a.cmp(&b_));
    });
}

fn bench_duration_since(c: &mut Criterion) {
    let a = DateTime::default();
    let b_ = a.add_days(7).expect("fixed");
    let _ = c.bench_function("duration_since", |bencher| {
        bencher.iter(|| b_.duration_since(&a));
    });
}

fn bench_is_within_range(c: &mut Criterion) {
    let mid = DateTime::default();
    let lo = mid.add_days(-1).expect("fixed");
    let hi = mid.add_days(1).expect("fixed");
    let _ = c.bench_function("is_within_range", |b| {
        b.iter(|| mid.is_within_range(&lo, &hi));
    });
}

// ---------------------------------------------------------------------
// Timezone conversion
// ---------------------------------------------------------------------

fn bench_convert_to_tz(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("convert_to_tz (EST_USA)", |b| {
        b.iter(|| dt.convert_to_tz(black_box("EST_USA")));
    });
}

// ---------------------------------------------------------------------
// Validators (batched — each is a few branches over `str::parse`)
// ---------------------------------------------------------------------

fn bench_validators_all(c: &mut Criterion) {
    let _ = c.bench_function("is_valid_* (all 11)", |b| {
        b.iter(|| {
            let _ = DateTime::is_valid_day(black_box("15"));
            let _ = DateTime::is_valid_hour(black_box("10"));
            let _ = DateTime::is_valid_minute(black_box("30"));
            let _ = DateTime::is_valid_second(black_box("45"));
            let _ = DateTime::is_valid_month(black_box("6"));
            let _ = DateTime::is_valid_year(black_box("2024"));
            let _ = DateTime::is_valid_microsecond(black_box("999"));
            let _ = DateTime::is_valid_ordinal(black_box("166"));
            let _ = DateTime::is_valid_iso_week(black_box("25"));
            let _ = DateTime::is_valid_time(black_box("10:30:45"));
            let _ = DateTime::is_valid_iso_8601(black_box(
                "2024-06-15T10:30:45Z",
            ));
        });
    });
}

// ---------------------------------------------------------------------
// Free helpers
// ---------------------------------------------------------------------

fn bench_is_leap_year(c: &mut Criterion) {
    let _ = c.bench_function("is_leap_year", |b| {
        b.iter(|| is_leap_year(black_box(2024)));
    });
}

fn bench_days_in_month(c: &mut Criterion) {
    let _ = c.bench_function("days_in_month", |b| {
        b.iter(|| days_in_month(black_box(2024), black_box(2)));
    });
}

// ---------------------------------------------------------------------
// Serde round-trip (only with the `serde` feature)
// ---------------------------------------------------------------------

#[cfg(feature = "serde")]
fn bench_serde_round_trip(c: &mut Criterion) {
    let dt = DateTime::default();
    let _ = c.bench_function("serde round-trip (JSON)", |b| {
        b.iter(|| {
            let s = serde_json::to_string(&dt).expect("serialise");
            let back: DateTime =
                serde_json::from_str(&s).expect("deserialise");
            black_box(back)
        });
    });
}

#[cfg(not(feature = "serde"))]
fn bench_serde_round_trip(_c: &mut Criterion) {
    // Skipped when the `serde` feature is off.
}

// ---------------------------------------------------------------------
// Group registration
// ---------------------------------------------------------------------

criterion_group!(
    benches,
    // Construction
    bench_new,
    bench_new_with_tz,
    bench_new_with_custom_offset,
    bench_from_components,
    bench_default,
    bench_builder,
    bench_extreme_date_future,
    bench_extreme_date_past,
    // Accessors
    bench_accessors_all,
    bench_unix_timestamp,
    // Parsing
    bench_parse_rfc3339,
    bench_parse_iso_8601_date,
    bench_parse_custom_format,
    // Formatting
    bench_format_rfc3339,
    bench_format_custom,
    bench_format_time_in_timezone,
    // Arithmetic
    bench_add_days,
    bench_add_days_negative,
    bench_next_previous_day,
    bench_add_months,
    bench_sub_months,
    bench_add_years,
    bench_sub_years,
    bench_add_duration,
    bench_sub_duration,
    // Mutation helpers
    bench_set_time,
    bench_set_date,
    bench_update,
    // Calendar boundaries
    bench_calendar_boundaries,
    // Comparison & range
    bench_compare,
    bench_duration_since,
    bench_is_within_range,
    // Timezone
    bench_convert_to_tz,
    // Validators
    bench_validators_all,
    // Free helpers
    bench_is_leap_year,
    bench_days_in_month,
    // Serde
    bench_serde_round_trip,
);

criterion_main!(benches);
