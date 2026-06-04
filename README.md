<p align="center">
  <img src="https://cloudcdn.pro/dtt/v1/logos/dtt.svg" alt="DateTime (DTT) logo" width="128" />
</p>

<h1 align="center">DateTime (DTT)</h1>

<p align="center">
  <strong>An ergonomic Rust library for parsing, validating, manipulating, and formatting dates, times, and timezones — with guaranteed round-trip safety and unambiguous timezone codes.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/dtt/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/dtt/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/dtt"><img src="https://img.shields.io/crates/v/dtt.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/dtt"><img src="https://img.shields.io/badge/docs.rs-dtt-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://codecov.io/gh/sebastienrousseau/dtt"><img src="https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&logo=codecov" alt="Coverage" /></a>
  <a href="https://lib.rs/crates/dtt"><img src="https://img.shields.io/badge/lib.rs-v0.0.10-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
</p>

---

## Table of Contents

- [Install](#install)
- [Quick Start](#quick-start)
- [Why DTT?](#why-dtt)
- [Features](#features)
- [Supported Timezone Abbreviations](#supported-timezone-abbreviations)
- [API Highlights](#api-highlights)
- [Development](#development)
- [Troubleshooting](#troubleshooting)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

---

## Install

```bash
cargo add dtt
```

Or add to `Cargo.toml`:

```toml
[dependencies]
dtt = "0.0.10"
```

### Prerequisites

DTT requires **Rust 1.88.0 or later** (pinned by `time = 0.3.47`, which carries the upstream fix for [RUSTSEC stack-exhaustion DoS](https://rustsec.org/) in `time < 0.3.47`).

| Platform | Setup |
|----------|-------|
| **macOS** | `brew install rustup-init && rustup-init -y` |
| **Linux** | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh -s -- -y` |
| **WSL** | Same as Linux, run inside your WSL distribution |
| **Windows** | Download `rustup-init.exe` from [rustup.rs](https://rustup.rs/) |

After install, verify with `rustc --version` (must be ≥ 1.88.0). Upgrade an existing toolchain with `rustup update stable`.

---

## Quick Start

```rust
use dtt::prelude::*;

fn main() -> Result<(), AppError> {
    // Current UTC time
    let now = DateTime::new();
    println!("Current time: {}", now);

    // Parse — strict, offset-required for time-bearing inputs
    let parsed = DateTime::parse("2024-01-15T10:30:00Z")?;
    println!("Parsed: {}", parsed);

    // Round-trip is guaranteed: parse(format(x)) == x
    let s = parsed.format_rfc3339()?;
    assert_eq!(parsed, DateTime::parse(&s)?);

    // Arithmetic
    let next_week = parsed.add_days(7)?;
    let next_year = parsed.add_years(1)?;
    println!("Next week: {next_week}, next year: {next_year}");

    // Timezone conversion (note the explicit USA suffix)
    let est = parsed.convert_to_tz("EST_USA")?;
    println!("In US Eastern: {est}");

    // Validation
    assert!(DateTime::is_valid_iso_8601("2024-01-15T10:30:00Z"));
    assert!(!DateTime::is_valid_year("10000")); // outside time crate range

    Ok(())
}
```

Run the full demo:

```bash
cargo run --example dtt
```

---

## Why DTT?

Most datetime libraries silently produce wrong answers in surprising places. DTT is designed to **fail loudly** rather than guess:

- **Round-trip safety:** `DateTime::parse(&dt.format_rfc3339()?)? == dt` always holds. No silent date-only truncation.
- **Unambiguous timezones:** `IST` could mean Indian (+05:30), Irish (+01:00), or Israel (+02:00). DTT requires explicit suffixes (`IST_INDIA`, `IST_IRELAND`, `IST_ISRAEL`) so you cannot accidentally use the wrong one.
- **Mixed-sign offsets rejected:** `new_with_custom_offset(5, -30)` returns an error instead of silently producing `+05:30`.
- **Deterministic `Default`:** `DateTime::default()` returns the Unix epoch, not wall-clock time, so tests are reproducible.
- **UTC-normalised equality:** Two `DateTime` values that represent the same instant compare equal regardless of which offset they were stored in.
- **Strict validation:** `is_valid_year` is bounded to the actual `time::Date` range (`-9999..=9999`), so the validator and the builder always agree.

---

## Features

| | |
| :--- | :--- |
| **Parsing** | RFC 3339 with offset, ISO 8601 date-only, custom format strings |
| **Formatting** | RFC 3339, custom format descriptors |
| **Validation** | Components, ranges, leap years, ISO 8601, time strings |
| **Arithmetic** | `add_days`, `add_months`, `add_years` (overflow-checked) |
| **Comparisons** | `Eq`, `Ord`, `Hash` — all UTC-normalised |
| **Calendar helpers** | `start_of_week`, `end_of_month`, `iso_week`, `iso_year` |
| **Timezone support** | 22 disambiguated abbreviations + custom offsets |
| **Serialization** | `serde` round-trip via canonical RFC 3339 strings |
| **Cross-platform** | macOS, Linux, WSL, Windows |

---

## Supported Timezone Abbreviations

Common abbreviations are intentionally **disambiguated**. Bare codes like `EST`, `CST`, `IST`, and `WADT` are **not accepted** because they refer to multiple zones in the real world.

| Code | Offset | Region |
|------|-------:|--------|
| `UTC`, `GMT` | +00:00 | Coordinated Universal Time |
| `EST_USA` | −05:00 | US Eastern Standard Time |
| `EDT` | −04:00 | US Eastern Daylight Time |
| `CST_USA` | −06:00 | US Central Standard Time |
| `CDT` | −05:00 | US Central Daylight Time |
| `MST` / `MDT` | −07/−06 | US Mountain |
| `PST` / `PDT` | −08/−07 | US Pacific |
| `CET` / `CEST` | +01/+02 | Central Europe |
| `EET` / `EEST` | +02/+03 | Eastern Europe |
| `IST_IRELAND` | +01:00 | Irish Standard Time |
| `IST_ISRAEL` | +02:00 | Israel Standard Time |
| `IST_INDIA` | +05:30 | Indian Standard Time |
| `JST` | +09:00 | Japan |
| `HKT` | +08:00 | Hong Kong |
| `CST_CHINA` | +08:00 | China Standard Time |
| `EST_AUS` / `AEST` | +10:00 | Australian Eastern |
| `AEDT` | +11:00 | Australian Eastern Daylight |
| `ACWST` | +08:45 | Australian Central Western |

For any other zone, use [`DateTime::new_with_custom_offset(hours, minutes)`](https://docs.rs/dtt/latest/dtt/datetime/struct.DateTime.html#method.new_with_custom_offset).

> **Note:** DST is not handled automatically. Pick the appropriate code (e.g. `EDT` vs `EST_USA`) for the date range you care about.

---

## API Highlights

### Construction

```rust
use dtt::prelude::*;
use time::UtcOffset;

let now    = DateTime::new();                              // current UTC
let utc    = DateTime::new_with_tz("UTC")?;                // explicit
let mumbai = DateTime::new_with_tz("IST_INDIA")?;          // disambiguated
let custom = DateTime::new_with_custom_offset(5, 30)?;     // +05:30
let exact  = DateTime::from_components(2024, 1, 15, 10, 30, 0, UtcOffset::UTC)?;
let epoch  = DateTime::default();                          // 1970-01-01T00:00:00Z

// Builder pattern
let dt = DateTimeBuilder::new()
    .year(2024).month(1).day(15)
    .hour(10).minute(30).second(0)
    .offset(UtcOffset::UTC)
    .build()?;
# Ok::<(), AppError>(())
```

### Parsing & Formatting

```rust
# use dtt::prelude::*;
let dt1 = DateTime::parse("2024-01-15T10:30:00Z")?;
let dt2 = DateTime::parse("2024-01-15T10:30:00+05:30")?;
let dt3 = DateTime::parse("2024-01-15")?;                  // date-only OK

let custom = DateTime::parse_custom_format(
    "15/01/2024 10:30",
    "[day]/[month]/[year] [hour]:[minute]",
)?;

let s: String = dt1.format_rfc3339()?;
let pretty   = dt1.format("[year]-[month]-[day]")?;
# Ok::<(), AppError>(())
```

### Arithmetic & Calendar Math

```rust
# use dtt::prelude::*;
let dt = DateTime::parse("2024-01-31T00:00:00Z")?;

let next_day  = dt.next_day()?;
let prev_day  = dt.previous_day()?;
let next_week = dt.add_days(7)?;
let next_feb  = dt.add_months(1)?;     // → 2024-02-29 (leap year aware)
let next_year = dt.add_years(1)?;

let monday    = dt.start_of_week()?;
let sunday    = dt.end_of_week()?;
let last_day  = dt.end_of_month()?;
# Ok::<(), AppError>(())
```

### Macros

```rust
use dtt::prelude::*;
use dtt::{dtt_now, dtt_parse, dtt_add_days, dtt_diff, dtt_diff_seconds};

let now = dtt_now!();
let dt  = dtt_parse!("2024-01-15T10:30:00Z")?;
let later = dtt_add_days!(dt, 7)?;

let secs: Option<i64> = dtt_diff_seconds!("1609459200", "1609459230");
assert_eq!(secs, Some(30));
# Ok::<(), AppError>(())
```

---

## Development

Clone, build, and verify in under a minute on any platform:

```bash
git clone https://github.com/sebastienrousseau/dtt.git
cd dtt

make verify     # fmt-check + lint + test in one command
make help       # full task list
```

The `Makefile` is a thin wrapper around the underlying Cargo commands,
so the equivalent direct invocations also work:

```bash
cargo build                                # build the library and binary
cargo test                                 # run all 240+ tests
cargo clippy --all-targets -- -D warnings  # lint with strict warnings
cargo fmt --check                          # verify formatting
cargo doc --no-deps --open                 # open API docs in your browser
cargo run --example dtt                    # run the end-to-end demo
cargo bench                                # run criterion benchmarks
```

All commands work identically on macOS, Linux, and WSL. CI exercises
the same matrix on Linux, macOS, **and** Windows on every PR via
[`.github/workflows/cross-platform.yml`](.github/workflows/cross-platform.yml).

---

## Troubleshooting

| Symptom | Likely cause | Fix |
|---------|--------------|-----|
| `feature 'edition2024' is required` from `time-core` | Rust < 1.88.0 | `rustup update stable` |
| `Err(InvalidTimezone)` for `"EST"`, `"CST"`, `"IST"` | Bare ambiguous codes are rejected by design | Use a suffixed form (e.g. `EST_USA`, `IST_INDIA`) |
| `Err(InvalidFormat)` parsing `"2024-01-01T12:00:00"` | RFC 3339 requires an offset | Append `Z` or `+HH:MM` |
| `Err(InvalidTimezone)` from `new_with_custom_offset(5, -30)` | Mixed-sign offsets are rejected | Pass same-sign components, e.g. `(4, 30)` |
| `Err(InvalidDate)` from `from_components(10000, ...)` | `time::Date` only supports `-9999..=9999` | Use a year inside that range |
| Tests fail with environment-variable race | `cargo test` runs tests in parallel | Already mitigated via `serial_test`; use `cargo test -- --test-threads=1` if you have a custom env-var test |

---

## Documentation

- **API reference:** <https://docs.rs/dtt>
- **End-to-end example:** [`examples/dtt.rs`](examples/dtt.rs)
- **Benchmarks:** [`benches/criterion.rs`](benches/criterion.rs) — run with `cargo bench`
- **Changelog:** [`CHANGELOG.md`](CHANGELOG.md)
- **Security policy:** [`.github/SECURITY.md`](.github/SECURITY.md)
- **Code of conduct:** [`.github/CODE-OF-CONDUCT.md`](.github/CODE-OF-CONDUCT.md)

---

## Contributing

Contributions are welcome. Please read [`CONTRIBUTING.md`](CONTRIBUTING.md) — in particular, **all commits must be cryptographically signed** (`git commit -S`).

Quick checklist before opening a PR:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
git commit -S -m "feat(dtt): your conventional commit message"
```

---

**THE ARCHITECT** ᴬ [Sebastien Rousseau](https://sebastienrousseau.com)
**THE ENGINE** ᵞ [EUXIS](https://euxis.co) ᴬ Enterprise Unified Execution Intelligence System

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#datetime-dtt">Back to Top</a></p>
