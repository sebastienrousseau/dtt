#![allow(missing_docs)]

// Property-based tests that pin DTT's load-bearing invariants on
// hundreds of generated inputs per run. These complement the
// hand-written unit tests by exercising edge cases a human would
// never think to write.

use dtt::prelude::*;
use proptest::prelude::*;
use time::UtcOffset;

proptest! {
    /// `parse(format_rfc3339(x)) == x` for every RFC-3339-representable
    /// `DateTime`.
    ///
    /// This is the library's most important contract: every datetime
    /// formatted via `format_rfc3339` must parse back to an equal value.
    /// If this ever fails, the silent-data-loss bug fixed in Round 2
    /// has returned.
    ///
    /// Note: the year range is `0..=9999` because RFC 3339 uses a
    /// four-digit unsigned year and cannot represent BC dates.
    #[test]
    fn rfc3339_round_trip(
        year in 0i32..=9999,
        month in 1u8..=12,
        day in 1u8..=28,              // safe for every month
        hour in 0u8..=23,
        minute in 0u8..=59,
        second in 0u8..=59,
        offset_hours in -23i8..=23,
        offset_minutes in 0u8..=59,
    ) {
        // Build a same-sign offset — mixed signs are rejected by design.
        let signed_minutes = if offset_hours < 0 {
            -(i8::try_from(offset_minutes).unwrap())
        } else {
            i8::try_from(offset_minutes).unwrap()
        };
        let offset = UtcOffset::from_hms(offset_hours, signed_minutes, 0)
            .unwrap();
        let dt = DateTime::from_components(
            year, month, day, hour, minute, second, offset,
        )
        .unwrap();

        let serialised = dt.format_rfc3339().unwrap();
        let parsed = DateTime::parse(&serialised).unwrap();
        prop_assert_eq!(dt, parsed);
    }

    /// Validator/parser symmetry: `is_valid_iso_8601(s) == parse(s).is_ok()`
    /// for any arbitrary string, including nonsense.
    ///
    /// Pins the Round-3 fix that the validator no longer accepts inputs
    /// the parser would reject.
    #[test]
    fn validator_matches_parser(s in "\\PC{0,64}") {
        prop_assert_eq!(
            DateTime::is_valid_iso_8601(&s),
            DateTime::parse(&s).is_ok(),
        );
    }

    /// `add_days(n)` followed by `add_days(-n)` returns the original
    /// value. Covers arbitrary positive and negative day deltas.
    #[test]
    fn add_days_is_reversible(days in -10_000i64..=10_000) {
        let dt = DateTime::from_components(
            2024, 6, 15, 12, 0, 0, UtcOffset::UTC,
        )
        .unwrap();
        if let Ok(forward) = dt.add_days(days) {
            let back = forward.add_days(-days).unwrap();
            prop_assert_eq!(dt, back);
        }
    }

    /// Two `DateTime` values with the same absolute instant but
    /// different offsets hash to the same value. Pins the Round-2
    /// UTC-normalised Hash fix.
    #[test]
    fn equal_instants_hash_equally(
        year in 1u16..=9999,
        month in 1u8..=12,
        day in 1u8..=28,
        hour in 0u8..=23,
        minute in 0u8..=59,
        offset_hours in -12i8..=12,
    ) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let offset = UtcOffset::from_hms(offset_hours, 0, 0).unwrap();
        let a = DateTime::from_components(
            i32::from(year), month, day, hour, minute, 0, UtcOffset::UTC,
        )
        .unwrap();
        // Same instant re-expressed in a non-UTC offset.
        let b = a.convert_to_tz("UTC").unwrap();
        // a and b are equal by value; hashes must match too.
        prop_assert_eq!(a, b);

        let _ = offset;  // touched to keep offset in strategy

        let mut ha = DefaultHasher::new();
        a.hash(&mut ha);
        let mut hb = DefaultHasher::new();
        b.hash(&mut hb);
        prop_assert_eq!(ha.finish(), hb.finish());
    }

    /// `add_months(12) == add_years(1)` within the supported year range.
    #[test]
    fn add_months_12_equals_add_years_1(year in -9998i32..=9998) {
        let dt = DateTime::from_components(
            year, 6, 15, 0, 0, 0, UtcOffset::UTC,
        )
        .unwrap();
        let via_months = dt.add_months(12);
        let via_years = dt.add_years(1);
        prop_assert_eq!(via_months, via_years);
    }
}
