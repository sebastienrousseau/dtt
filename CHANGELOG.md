# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] — 2026-06-04

This release was originally prepared as `0.0.10` but is shipping as
`0.1.0` because the `time` security upgrade pins a transitive
dependency (`time-core = =0.1.8`) that requires Rust `1.88.0`. Per
[`docs/msrv-policy.md`](docs/msrv-policy.md), MSRV may only move in a
MINOR release pre-1.0.

### Security

- **Bump `time` to 0.3.47** to pull in the upstream fix for
  [RUSTSEC: time vulnerable to stack exhaustion DoS][rustsec-time]
  (medium severity, affects `>= 0.3.6, < 0.3.47`).

### Added

- **Optional `serde` feature** (on by default). Disabling it via
  `default-features = false` drops `serde`, `serde_json`, and `time/serde`
  from the dependency graph (~6 fewer transitive crates).
- `[lints.clippy]` and `[lints.rust]` in `Cargo.toml` as the canonical
  source of truth for universal clippy/rust allowances. Library-only
  strict lints (`pedantic`, `nursery`, `cargo`, `unwrap_used`,
  `expect_used`, `panic`, `result_unit_err`, `clone_on_ref_ptr`) remain
  as inner `#![deny(...)]` attributes in `src/lib.rs` so they apply to
  the lib crate only — integration tests, benches, and examples keep
  their freedom to use `unwrap`/`expect`.
- [`docs/msrv-policy.md`](docs/msrv-policy.md) committed to the
  repository (was previously untracked).

### Changed

- **MSRV bumped `1.80.0` → `1.88.0`.** Required by `time-core = =0.1.8`
  (transitive of `time 0.3.47`), which declares `rust-version = "1.88.0"`
  and uses Cargo's `edition2024` feature. There is no `time` release
  containing the RUSTSEC fix with a lower MSRV; downgrading would
  reintroduce the vulnerability.
- **Modularised `src/datetime.rs`** (2492 → 1645 + 159 + 95 + 623 lines)
  into `src/datetime/{mod,builder,validate,tests}.rs`. Public API is
  unchanged; existing `dtt::datetime::DateTime` and
  `dtt::datetime::DateTimeBuilder` paths still resolve.
- Removed the unused `[package.metadata.clippy]` block (the canonical
  enforcement is `[lints.clippy]` + the inner attrs in `src/lib.rs`).
- Bump `serde` 1.0.217 → 1.0.228 (absorbs dependabot PRs #110, #124).
- Bump `serde_json` 1.0.135 → 1.0.140 (absorbs dependabot PR #107).
- Bump `thiserror` 2.0.11 → 2.0.18 (absorbs dependabot PRs #106, #126).
- Bump `assert_cmd` (dev) 2.0.16 → 2.2.0 (absorbs dependabot PRs #113, #119).
- Bump `criterion` (dev) 0.5.1 → 0.8.2 (absorbs dependabot PR #127); switch
  benches from `criterion::black_box` (deprecated) to `std::hint::black_box`.
- Bump `regex` (dev) 1.11.1 → 1.12.3 (absorbs dependabot PR #121).
- Bump `actions/checkout` v4 → v6 in `cross-platform.yml`
  (absorbs dependabot PR #118).
- Bump `actions/upload-artifact` v4 → v7 in `cross-platform.yml`
  (absorbs dependabot PR #120).

### Fixed

- Satisfy clippy 1.96 `map_unwrap_or` lint: 10 `Result.map(...).unwrap_or(false)`
  sites rewritten to `Result.is_ok_and(...)` in `src/datetime/validate.rs` and
  `src/lib.rs`.
- SBOM job: `cargo-cyclonedx 0.5.x` writes `dtt-sbom.json`; both
  `cross-platform.yml` and `make sbom` now `mv` it to the conventional
  `dtt-sbom.cdx.json` so the `upload-artifact` step finds it.
- `build.rs`: inline format args (clippy::uninlined_format_args).

[rustsec-time]: https://github.com/sebastienrousseau/dtt/security/dependabot/3


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

[Unreleased]: https://github.com/sebastienrousseau/dtt/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/sebastienrousseau/dtt/compare/v0.0.9...v0.1.0
[0.0.10]: https://github.com/sebastienrousseau/dtt/compare/v0.0.9...v0.0.10
[0.0.9]: https://github.com/sebastienrousseau/dtt/releases/tag/v0.0.9
