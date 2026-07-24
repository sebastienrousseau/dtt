# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.11] — 2026-07-24


## [0.0.10] — 2026-06-04

`0.0.10` is a correctness + reliability + supply-chain release. It
also raises the MSRV from `1.80.0` to `1.88.0` under the
security-driven patch-release exception documented in
[`docs/msrv-policy.md`](docs/msrv-policy.md) — the `time` security
upgrade pins `time-core = =0.1.8`, which declares
`rust-version = "1.88.0"`. No earlier `time` release contains the
RUSTSEC fix; downgrading would reintroduce the vulnerability.

### Security

- **Bump `time` to 0.3.47** to pull in the upstream fix for
  [RUSTSEC: time vulnerable to stack exhaustion DoS][rustsec-time]
  (medium severity, affects `>= 0.3.6, < 0.3.47`).
- **Migrate `paste 1.0.15` → `pastey 0.2.1`** to close
  RUSTSEC-2024-0436 (unmaintained). Drop-in fork; the
  `dtt_is_valid_function!` macro now references `::pastey::paste!` via
  absolute path so callers no longer need a `use paste::paste;` import.
- `.cargo/audit.toml` — empty ignore list; `unmaintained`,
  `unsound`, and `notice` advisories are hard errors.
- `cargo-audit` gated in CI on every PR.
- CycloneDX SBOM (`dtt-sbom.cdx.json`) generated and uploaded on every PR.
- `docs/compliance/software-validation-procedure.md` — ISO 13485
  § 4.1.6 SVP committed.
- Branch protection `required_signatures` enforced on `main`.

### Added

- `DateTime::iso_year()` companion to `iso_week()`. Returns the ISO 8601
  week-numbering year, which can differ from the calendar year near year
  boundaries (e.g. `2022-01-01` has ISO year `2021`).
- Disambiguated timezone abbreviations: `EST_USA`, `EST_AUS`, `CST_USA`,
  `CST_CHINA`, `IST_INDIA`, `IST_IRELAND`, `IST_ISRAEL`, plus `ACWST`
  (replacing the incorrect `WADT` label for `+08:45`).
- Regression test for `is_valid_iso_8601` ⇔ `parse` symmetry.
- Regression test for mixed-sign rejection in `new_with_custom_offset`.
- Property tests via `proptest 1.5`: `rfc3339_round_trip`,
  `validator_matches_parser`, `add_days_is_reversible`,
  `equal_instants_hash_equally`, `add_months_12_equals_add_years_1`.
  Regression seed for the historical `year = -1` case committed.
- **Optional `serde` feature** (on by default). Disabling it via
  `default-features = false` drops `serde`, `serde_json`, and
  `time/serde` from the dependency graph (~6 fewer transitive crates).
- `[lints.clippy]` and `[lints.rust]` in `Cargo.toml` as the canonical
  source of truth for universal clippy/rust allowances. Library-only
  strict lints (`pedantic`, `nursery`, `cargo`, `unwrap_used`,
  `expect_used`, `panic`, `result_unit_err`, `clone_on_ref_ptr`) remain
  as inner `#![deny(...)]` attributes in `src/lib.rs` so they apply to
  the lib crate only — integration tests, benches, and examples keep
  their freedom to use `unwrap`/`expect`.
- [`docs/msrv-policy.md`](docs/msrv-policy.md) committed (was
  previously untracked); now also documents the security-driven
  patch-release MSRV-bump exception.
- Coverage gate (`cargo llvm-cov --fail-under-lines 99
  --fail-under-functions 100`) and cross-platform matrix
  (`ubuntu × macos × windows × stable + MSRV`) on `feat/**`.
- Functional example gallery: `parse.rs`, `arithmetic.rs`,
  `timezone.rs`, `validate.rs`.
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

### Changed — Encapsulation & Architecture

- `DateTime::{datetime, offset}` are now `pub(crate)`. Use the public
  accessors (`year()`, `month()`, ..., `offset()`) and constructors.
- Custom `serde` impls round-trip `DateTime` via canonical RFC 3339
  strings instead of field-based serialization (cfg-gated behind the
  `serde` feature).
- **Modularised `src/datetime.rs`** (2,492 lines) into
  `src/datetime/{mod,builder,validate,tests}.rs` —
  `mod.rs` (1,645) + `builder.rs` (159) + `validate.rs` (95) +
  `tests.rs` (623). Public API is unchanged; existing
  `dtt::datetime::{DateTime, DateTimeBuilder}` paths still resolve.

### Changed — Build & Tooling

- **MSRV `1.80.0` → `1.88.0`.** Required by `time-core = =0.1.8`
  (transitive of `time 0.3.47`), which declares
  `rust-version = "1.88.0"` and uses Cargo's `edition2024` feature.
  Permitted as a patch-release MSRV bump under the security-driven
  exception in [`docs/msrv-policy.md`](docs/msrv-policy.md).
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
- Removed the unused `[package.metadata.clippy]` block (the canonical
  enforcement is `[lints.clippy]` + the inner attrs in `src/lib.rs`).
- Absorbed dependabot bumps: `serde 1.0.217 → 1.0.228` (PRs #110, #124),
  `serde_json 1.0.135 → 1.0.140` (#107), `thiserror 2.0.11 → 2.0.18`
  (#106, #126); dev: `assert_cmd 2.0.16 → 2.2.0` (#113, #119),
  `criterion 0.5.1 → 0.8.2` (#127; benches switched from
  `criterion::black_box` to `std::hint::black_box`),
  `regex 1.11.1 → 1.12.3` (#121); workflows: `actions/checkout v4 → v6`
  (#118), `actions/upload-artifact v4 → v7` (#120).

### Fixed

- Hardcoded version assertion in `tests/test_lib.rs` (`0.0.9` → `0.0.10`).
- `build.rs` error message referring to `'fd'` instead of `'dtt'`;
  inline format args (`clippy::uninlined_format_args`).
- Brittle `size_of::<DateTimeError>() == 56` ABI assertion replaced with
  a structural smoke test.
- Stale `unused_results` and `missing_fragment_specifier` lints (the
  latter has been removed from rustc).
- Satisfy clippy 1.96 `map_unwrap_or` lint: 10
  `Result.map(...).unwrap_or(false)` sites rewritten to
  `Result.is_ok_and(...)` in `src/datetime/validate.rs` and `src/lib.rs`.
- SBOM job: `cargo-cyclonedx 0.5.x` writes `dtt-sbom.json`; both
  `cross-platform.yml` and `make sbom` now `mv` it to the conventional
  `dtt-sbom.cdx.json` so the `upload-artifact` step finds it.
- 51 `clippy::uninlined_format_args` fixes in `examples/dtt.rs`,
  `tests/{test_main,test_datetime,test_error}.rs` and `src/main.rs`
  for clippy 1.88 (where the lint is default-warn).

### Removed

- `format_iso8601` — duplicate of `format_rfc3339`.
- `dtt_create_vec!` — duplicate of `vec![]`.
- `lazy_static` dependency.
- 7 per-task GitHub Actions workflow files (consolidated into `ci.yml`).
- ~780 lines of duplicate test modules in `tests/test_datetime.rs`.
- Dead `[package.metadata.clippy]` block in `Cargo.toml`.

[rustsec-time]: https://github.com/sebastienrousseau/dtt/security/dependabot/3


## [0.0.9] — 2025-01

- Earlier history; see git log for details.

[Unreleased]: https://github.com/sebastienrousseau/dtt/compare/v0.0.10...HEAD
[0.0.10]: https://github.com/sebastienrousseau/dtt/compare/v0.0.9...v0.0.10
[0.0.9]: https://github.com/sebastienrousseau/dtt/releases/tag/v0.0.9
