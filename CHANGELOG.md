# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.10] — 2026-04-06

### Added

- `DateTime::iso_year()` companion to `iso_week()`. Returns the ISO 8601
  week-numbering year, which can differ from the calendar year near year
  boundaries (e.g. `2022-01-01` has ISO year `2021`).
- Disambiguated timezone abbreviations: `EST_USA`, `EST_AUS`, `CST_USA`,
  `CST_CHINA`, `IST_INDIA`, `IST_IRELAND`, `IST_ISRAEL`, plus `ACWST`
  (replacing the incorrect `WADT` label for `+08:45`).
- Regression test for `is_valid_iso_8601` ⇔ `parse` symmetry.
- Regression test for mixed-sign rejection in `new_with_custom_offset`.
- `CHANGELOG.md` (this file).

### Changed — Correctness

- **`Eq` / `Ord` / `Hash` are now UTC-normalised.** Two `DateTime` values
  representing the same instant compare equal regardless of which offset
  they were stored in. Previously the derived impls compared offset and
  primitive datetime independently, violating the
  `a == b ⇒ a.cmp(&b) == Equal` contract.
- **`DateTime::parse` is now strict for time-bearing inputs.** Strings
  containing `T` or a space must include an offset; the parser no longer
  silently truncates `2024-01-01T12:00:00` to a date-only midnight.
- **`format_rfc3339` round-trip is guaranteed.** `parse(format(x)) == x`
  for any well-formed `DateTime`.
- **`add_months` no longer overflows or returns wrong results for negative
  totals.** Year/month arithmetic uses `checked_*` and `div_euclid` /
  `rem_euclid`.
- **`is_valid_iso_8601` mirrors `parse` exactly.** The validator no longer
  accepts strings the parser would reject.
- **`is_valid_year` is bounded to `-9999..=9999`** (the actual `time::Date`
  range), so the validator and `from_components` always agree.
- **`new_with_custom_offset` rejects mixed-sign inputs.** Calls like
  `(5, -30)` now return `InvalidTimezone` instead of silently producing
  `+05:30`.
- **`DateTime::default()` returns the Unix epoch** (`1970-01-01T00:00:00Z`)
  instead of the current wall-clock time, making it deterministic.
- **`dtt_diff!` returns `Option<i64>`** instead of `panic!`-ing on parse
  failure. `dtt_diff_seconds!` and `dtt_diff_days!` propagate this.
- **`format_iso8601` removed** — it was a byte-identical duplicate of
  `format_rfc3339` after the round-trip fix. Use `format_rfc3339`.
- **`dtt_create_vec!` removed** — it was a redundant alias for `vec![]`.

### Changed — Encapsulation

- `DateTime::{datetime, offset}` are now `pub(crate)`. Use the public
  accessors (`year()`, `month()`, ..., `offset()`) and constructors.
- Custom `serde` impls round-trip `DateTime` via canonical RFC 3339
  strings instead of field-based serialization.

### Changed — Build & Tooling

- **MSRV bumped to 1.80.0** (required by `std::sync::LazyLock`).
- Replaced `lazy_static` with `std::sync::LazyLock`; dropped the
  `lazy_static` dependency entirely.
- Dropped the unused `time` `macros` feature.
- Moved `regex` from `[dependencies]` to `[dev-dependencies]` (test-only).
- `unsafe_code` is now `forbid`-ed crate-wide via `Cargo.toml` lints,
  matching the existing `#![forbid(unsafe_code)]` in `lib.rs`.
- Env-var-touching tests are now serial via `serial_test` to avoid the
  unsound parallel `env::set_var` race.
- Consolidated 7 GitHub Actions workflows into a single reusable
  `ci.yml` that delegates to `sebastienrousseau/pipelines`.

### Fixed

- Hardcoded version assertion in `tests/test_lib.rs` (`0.0.9` → `0.0.10`).
- `build.rs` error message referring to `'fd'` instead of `'dtt'`.
- Brittle `size_of::<DateTimeError>() == 56` ABI assertion replaced with
  a structural smoke test.
- Stale `unused_results` and `missing_fragment_specifier` lints (the
  latter has been removed from rustc).

### Removed

- `format_iso8601` — duplicate of `format_rfc3339`.
- `dtt_create_vec!` — duplicate of `vec![]`.
- `lazy_static` dependency.
- 7 per-task GitHub Actions workflow files (consolidated into `ci.yml`).
- ~780 lines of duplicate test modules in `tests/test_datetime.rs`.

## [0.0.9] — 2025-01

- Earlier history; see git log for details.

[Unreleased]: https://github.com/sebastienrousseau/dtt/compare/v0.0.10...HEAD
[0.0.10]: https://github.com/sebastienrousseau/dtt/compare/v0.0.9...v0.0.10
[0.0.9]: https://github.com/sebastienrousseau/dtt/releases/tag/v0.0.9
