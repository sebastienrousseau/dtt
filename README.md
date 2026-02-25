<!-- markdownlint-disable MD033 MD041 -->
<img src="https://kura.pro/dtt/images/logos/dtt.svg"
alt="DateTime (DTT) logo" height="66" align="right" />
<!-- markdownlint-enable MD033 MD041 -->

# `DateTime` (DTT)

A Rust library for parsing, validating, manipulating, and formatting dates and times.

[![Made With Love][made-with-rust]][14] [![Crates.io][crates-badge]][08] [![lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![Codecov][codecov-badge]][15] [![Build Status][build-badge]][16] [![Codecov][github-badge]][07]

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

• [Website][01] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Overview

The **`DateTime` (DTT)** library is a comprehensive, flexible toolkit for managing dates and times in Rust. It includes functions, macros, and data structures that enable you to create, parse, validate, and format date-time objects with minimal effort.

You can construct `DateTime` objects in UTC or in a fixed offset timezone, ensuring accurate date and time information for varied use cases. The library also offers validation methods, which help maintain precision and correctness in time-critical operations.

## Features (2026 Apex Standard)

The `DateTime` (DTT) library is engineered to represent the definitive standard for Rust temporal operations:

- **Brutalist Performance (Core Architecture)**
  - **Memory-Mapped Timezones**: Zero-allocation `mmap` timezone hot-reloading from `/usr/share/zoneinfo/`.
  - **Platform-Native Hooks**: Sycall-bypassing monotonic operations via macOS VDSO and `mach_absolute_time`.
  - **SIMD Parsing**: 256-bit Vectorized ISO-8601 parsing (`std::simd`), outperforming legacy routines by orders of magnitude.

- **Horizon 5: Apex Mastery (GPU & Ecosystem)**
  - **WebGPU Compute Pipeline**: Massive hardware parallelization via `wgpu` (`dtt_parse.wgsl`) projecting billion-row/second streaming logic.
  - **Database Ubiquity**: Native trait integrations for the 2026 ecosystem (`diesel`, `tokio-postgres`, `sqlx`) using zero-byte-copy `ToSql`/`FromSql` serialization.

- **Developer Experience (DX) & Tooling**
  - **Zero-Overhead Macros**: Evaluate and map logic ergonomically (`dtt_tai_now!()`, `dtt_add!()`, `dtt_years!()`) at compile time.
  - **Fluent Builder APIs**: `DateTime::new().plus(12.hours()).in_tz("EST")`.
  - **Diagnostic Context**: `miette` derived error spans for precise stack-tracing and observability.
  - **Conversational Time**: `dtt_relative!(dt)` -> `"Just now"`, `"in 3 months"`.

- **Modern Compliance (TAI & WASM)**
  - **Leap-Second Resistance**: High-frequency TAI definitions decouple temporal states from legacy UTC irregularities.
  - **Browser Runtimes**: Out-of-the-box `wasm32-unknown-unknown` integrations binding natively to `js_sys::Date`.

- **Standard Toolkit**
  - Validation (`is_valid_iso_8601`), Calendar operations (`start_of_month()`, `add_calendar()`), and `duration_since()`.
  - `serde`-enabled structures for immediate wire transmissions (`JSON`/`BSON`).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dtt = { version = "0.1.0", features = ["db-diesel", "db-postgres", "gpu"] }
```

Then in your `main.rs` or lib crate:

```rust
extern crate dtt;
use dtt::*;
```

## Usage

Below are some quick examples showing how to use the core features of the `DateTime (DTT)` library.

### Basic Usage

```rust
use dtt::datetime::DateTime;
use dtt::dtt_print;

// Create a new DateTime in UTC
let now = DateTime::new();
dtt_print!(now);

// Create a DateTime in a specified timezone
let ny_time = DateTime::new_with_tz("EST").expect("Valid timezone");
println!("Current time in New York: {}", ny_time);

// Parse a date-time string
let date = DateTime::parse("2023-05-20T15:30:00Z").expect("Valid date and time");
println!("Parsed date: {}", date);

// Add days to a date-time
let future_date = date.add_days(7).expect("Should be valid");
println!("Date after 7 days: {}", future_date);
```

### Macro Usage

The library includes macros that streamline common operations:

```rust
use dtt::{dtt_now, dtt_parse, dtt_add_days};

let now = dtt_now!();
let parsed_date = dtt_parse!("2023-05-20T15:30:00Z").expect("Valid date");
let future_date = dtt_add_days!(parsed_date, 7).expect("Should be valid");

```

### Error Handling

The `DateTimeError` enum is used to handle invalid or out-of-range dates and times:

```rust
use dtt::datetime::DateTime;
use dtt::error::DateTimeError;

fn example_with_error_handling() -> Result<(), DateTimeError> {
    let date = DateTime::parse("2023-05-20T15:30:00Z")?;
    println!("Parsed date: {}", date);

    // Attempt to parse an invalid date string
    let result = DateTime::parse("2023-13-20T15:30:00Z");
    match result {
        Ok(_) => println!("Unexpected success."),
        Err(e) => println!("Failed to parse date-time: {}", e),
    }

    Ok(())
}
```

## Documentation

Comprehensive API documentation is available at:

- [Website][01]  
- [Docs.rs][09]  
- [https://doc.dttlib.com/][17]

## Rust Version Compatibility

Requires **rustc 1.56.0** or above.  

## Contributing

All contributions are appreciated! Please follow our [contributing instructions][05] for details on reporting issues, requesting features, or submitting pull requests. Contributions are subject to [Rust's Code of Conduct][12].

By submitting any contribution, you agree to license your contribution under the same dual licence chosen by this project.

## License

Licensed under either of these:

- [Apache License, Version 2.0][02]
- [MIT license][03]

You may select either licence as needed.

## Credits and Acknowledgements

Many thanks to all the contributors of the [DateTime (DTT) Library][06]. We also extend our gratitude to the [Rust Reddit][13] community for their helpful suggestions and feedback.

[01]: https://dttlib.com "DateTime (DTT) Library Website"
[02]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[03]: https://opensource.org/licenses/MIT "MIT license"
[04]: https://github.com/sebastienrousseau/dtt/issues "Issues"
[05]: https://github.com/sebastienrousseau/dtt/blob/main/CONTRIBUTING.md "Contributing Instructions"
[06]: https://github.com/sebastienrousseau/dtt/graphs/contributors "Contributors"
[07]: https://github.com/sebastienrousseau/dtt "DateTime (DTT) Library"
[08]: https://crates.io/crates/dtt "Crates.io"
[09]: https://docs.rs/dtt "Docs.rs"
[10]: https://lib.rs/crates/dtt "Lib.rs"
[12]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[13]: https://reddit.com/r/rust "Rust Reddit"
[14]: https://www.rust-lang.org "The Rust Programming Language"
[15]: https://codecov.io/gh/sebastienrousseau/dtt "Codecov"
[16]: https://github.com/sebastienrousseau/dtt/actions?query=branch%3Amain "Build Status"
[17]: https://doc.dttlib.com/ "DateTime (DTT) Library Documentation"

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/dtt/release.yml?branch=main&style=for-the-badge&logo=github "Build Status"
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/main-8da0cb?style=for-the-badge&labelColor=555555&logo=github "GitHub"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&token=X3ZP0K1SGI 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/dtt.svg?style=for-the-badge 'Crates.io badge'
[docs-badge]: https://img.shields.io/docsrs/dtt.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.1.0-orange.svg?style=for-the-badge 'Lib.rs badge'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
