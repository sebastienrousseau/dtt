#![allow(missing_docs)]

// Macros — ergonomic shortcuts for common datetime operations.
//
// The DTT crate exposes 19 `dtt_*!` macros via `#[macro_export]`.
// The four already covered by `examples/dtt.rs` (`dtt_now!`,
// `dtt_parse!`, `dtt_add_days!`, `dtt_sub_days!`, the min/max/join/map
// helpers) are not repeated here; this example focuses on the rest:
//
//   - `dtt_print!`         simple stdout helper
//   - `dtt_new_with_tz!`   shorthand for `DateTime::new_with_tz`
//   - `dtt_diff!`          generic diff returning `Option<i64>`
//   - `dtt_diff_seconds!`  diff in seconds
//   - `dtt_diff_days!`     diff in whole days
//   - `dtt_clone!`         explicit clone (useful in macros)
//   - `dtt_format!`        format with a pattern string
//
// Also demonstrates the two free helper functions in `dtt::datetime`:
// `is_leap_year` and `days_in_month`, plus `DateTime::default` and
// `unix_timestamp`.

use dtt::datetime::{days_in_month, is_leap_year};
use dtt::prelude::*;
use dtt::{
    dtt_clone, dtt_diff, dtt_diff_days, dtt_diff_seconds, dtt_format,
    dtt_new_with_tz, dtt_print,
};
// `dtt_format!` expands references to `time::Month`, so it must be
// in scope at the call site.
use time::Month;

fn main() -> Result<(), AppError> {
    // ------------------------------------------------------------------
    // dtt_print! — terse stdout helper.
    // ------------------------------------------------------------------
    dtt_print!("== DTT macros showcase ==");

    // ------------------------------------------------------------------
    // dtt_new_with_tz! — shorthand for `DateTime::new_with_tz` that
    // unwraps the Result and panics on an unknown zone. Use the
    // function form (`DateTime::new_with_tz`) when you need the error.
    // ------------------------------------------------------------------
    let mumbai = dtt_new_with_tz!("IST_INDIA");
    println!("Mumbai now     : {mumbai}");

    // ------------------------------------------------------------------
    // dtt_clone! — explicit clone. `DateTime` is `Copy` so this is
    // semantically a no-op for `DateTime`, but the macro is useful when
    // composing with other macros that need an owned value.
    // ------------------------------------------------------------------
    let original = DateTime::parse("2024-01-15T10:30:00Z")?;
    let copy = dtt_clone!(original);
    assert_eq!(original, copy);

    // ------------------------------------------------------------------
    // dtt_format! — format a DateTime via `{name}` placeholders.
    // The template MUST reference every placeholder the macro provides
    // (year, month, day, hour, minute, second, microsecond,
    // offset_sign, offset_hour, offset_minute) or `format!` rejects
    // the unused-named-arg.
    // ------------------------------------------------------------------
    let pretty = dtt_format!(
        original,
        "{year}-{month}-{day}T{hour}:{minute}:{second}.{microsecond}{offset_sign}{offset_hour}:{offset_minute}"
    );
    println!("Formatted      : {pretty}");
    assert_eq!(pretty, "2024-01-15T10:30:00.000000+00:00");

    // The generic `dtt_diff!` with an explicit unit divisor (seconds).
    let diff: Option<i64> = dtt_diff!("1609459200", "1609459260", 1);
    assert_eq!(diff, Some(60));

    // ------------------------------------------------------------------
    // dtt_diff_seconds! / dtt_diff_days! — diffs over Unix-timestamp
    // strings. Return `Option<i64>` — `None` on parse failure, never
    // panic.
    // ------------------------------------------------------------------
    // 2021-01-01T00:00:00Z and 2021-01-01T00:00:30Z
    let secs: Option<i64> =
        dtt_diff_seconds!("1609459200", "1609459230");
    println!("Seconds diff   : {:?}", secs);
    assert_eq!(secs, Some(30));

    // 2021-01-01 vs 2021-01-08 — exactly 7 days.
    let days: Option<i64> = dtt_diff_days!("1609459200", "1610064000");
    println!("Days diff      : {:?}", days);
    assert_eq!(days, Some(7));

    // Invalid input → None (no panic).
    let bad: Option<i64> = dtt_diff_seconds!("not-a-timestamp", "0");
    assert_eq!(bad, None);

    // ------------------------------------------------------------------
    // Free helper functions: leap-year + days-in-month.
    // ------------------------------------------------------------------
    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
    assert!(is_leap_year(2000)); // divisible by 400
    assert!(!is_leap_year(1900)); // divisible by 100 but not 400
    println!("is_leap(2024)  : true");

    assert_eq!(days_in_month(2024, 2)?, 29); // leap
    assert_eq!(days_in_month(2023, 2)?, 28);
    assert_eq!(days_in_month(2024, 4)?, 30);
    assert_eq!(days_in_month(2024, 12)?, 31);
    assert!(days_in_month(2024, 13).is_err());
    println!("Days in Feb 24 : 29");

    // ------------------------------------------------------------------
    // `DateTime::default` — deterministic Unix epoch (not wall-clock).
    // ------------------------------------------------------------------
    let epoch = DateTime::default();
    println!("Epoch (default): {epoch}");
    assert_eq!(epoch.unix_timestamp(), 0);

    // ------------------------------------------------------------------
    // `unix_timestamp` round-trip via from_components.
    // ------------------------------------------------------------------
    let ts = DateTime::parse("2024-01-15T10:30:00Z")?.unix_timestamp();
    println!("Unix timestamp : {ts}");

    Ok(())
}
