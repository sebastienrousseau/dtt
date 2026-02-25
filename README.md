<!-- markdownlint-disable MD033 MD041 -->
<img src="https://kura.pro/dtt/images/logos/dtt.svg"
alt="DateTime (DTT) logo" height="66" align="right" />
<!-- markdownlint-enable MD033 MD041 -->

# DateTime (DTT)

A Rust library for parsing, validating, manipulating, and formatting dates and times.

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

[![Made With Love][made-with-rust]][14] [![Crates.io][crates-badge]][08] [![lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][09] [![Codecov][codecov-badge]][15] [![Build Status][build-badge]][16] [![GitHub][github-badge]][07]

• [Website][01] • [Documentation][09] • [Report Bug][04] • [Request Feature][04] • [Contributing Guidelines][05]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

## Status
**Experimental** — This library is under active development. API may change in future versions.

| Version | Status | Notes |
|---------|--------|-------|
| 0.1.x | Experimental | New core architecture stabilization |
| 0.2.x | Beta (planned) | API stabilization |
| 1.0.x | Stable (planned) | Stable API, semver guarantees |

## Features
- **Performance Architecture** — High-speed, SIMD-accelerated date-time parsing and zero-allocation timezone loading.
- **Hardware Integrations** — Sub-nanosecond time hooks via platform VDSOs and native WebGPU compute shading for extreme parallel processing.
- **Fluent Builder APIs** — Ergonomic, chainable macros and builder components to create precise dates instantly.
- **Database Interoperability** — Out-of-the-box native trait support for `diesel`, `tokio-postgres`, and `sqlx`.
- **Ecosystem Ready** — Support for 100% test coverage, comprehensive observability (`tracing`), and high-diagnostic contexts (`miette`).
- **WebAssembly** — Ready to deploy natively in browser runtimes (`wasm32-unknown-unknown`).

## Installation
Add to `Cargo.toml`:

```toml
[dependencies]
dtt = "0.1.0"
```

## Requirements
- **MSRV**: Rust 1.56.0 or later

## Feature Flags
| Feature | Description | Default |
|---------|-------------|---------|
| `default` | No optional features enabled | ✅ |
| `tracing` | Enable structured span tracing across the engine | ❌ |
| `db-sqlx` | Enable sqlx types integration | ❌ |
| `db-postgres` | Enable tokio-postgres trait support | ❌ |
| `db-diesel` | Enable diesel trait implementations | ❌ |
| `gpu` | Enable the WebGPU vector offloading interface | ❌ |

Enable features in `Cargo.toml`:

```toml
[dependencies]
dtt = { version = "0.1.0", features = ["gpu", "db-diesel"] }
```

## Quick Start
Parse, manipulate, and observe dates:

```rust
use dtt::datetime::DateTime;
use dtt::error::DateTimeError;

fn main() -> Result<(), DateTimeError> {
    // Determine the current system time using minimal bindings
    let now = DateTime::new();
    println!("Current: {}", now);

    // Chain operations dynamically 
    let deadline = now.add_days(7)?.in_tz("EST")?;
    println!("Deadline: {}", deadline);

    Ok(())
}
```

## Examples
Run examples:

```shell
cargo run --example <example_name>
```

Available examples:
- `example_gpu` — Demonstrates billion-row matrix parsing natively on the system GPU bounds using WebGPU compute shaders.
- `example_builder` — Shows how to use the fluent builder API to construct, mutate, and chain datetime offsets.
- `example_macros` — Demonstrates the usage of zero-overhead compile-time macros for rapid date initiation and arithmetic.
- `example_datetime` — Covers standard operations like instantiation, formatting, and manual field extraction.
- `example_timezone` — Highlights how to hot-reload timezone offsets and convert primitives across geographic bounds.

## Known Limitations
- **WASM Timezone Availability** — `tzdata` parsing relies on system mounts and behaves uniquely in isolated wasm runtimes without VFS mappings.
- **GPU Array Matrixing** — The WebGPU shader requires arrays uniformly padded to fixed lengths for the string-compute dispatch map.

## Documentation
Browse complete API reference at [docs.rs/dtt][09].

## Contributing
Read [Contributing Guidelines][05] before submitting changes.

## License
Choose either [Apache 2.0][02] or [MIT][03] license.

---

🎨 Designed by Sebastien Rousseau — <https://sebastienrousseau.com/>

[01]: https://dttlib.com
[02]: https://opensource.org/license/apache-2-0/
[03]: https://opensource.org/licenses/MIT
[04]: https://github.com/sebastienrousseau/dtt/issues
[05]: https://github.com/sebastienrousseau/dtt/blob/main/CONTRIBUTING.md
[07]: https://github.com/sebastienrousseau/dtt 
[08]: https://crates.io/crates/dtt
[09]: https://docs.rs/dtt
[10]: https://lib.rs/crates/dtt
[14]: https://www.rust-lang.org
[15]: https://codecov.io/gh/sebastienrousseau/dtt
[16]: https://github.com/sebastienrousseau/dtt/actions?query=branch%3Amain

[build-badge]: https://img.shields.io/github/actions/workflow/status/sebastienrousseau/dtt/release.yml?branch=main&style=for-the-badge&logo=github
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&token=X3ZP0K1SGI&logo=codecov
[crates-badge]: https://img.shields.io/crates/v/dtt.svg?style=for-the-badge&color=fc8d62&logo=rust
[docs-badge]: https://img.shields.io/badge/docs.rs-dtt-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[github-badge]: https://img.shields.io/badge/github-sebastienrousseau/dtt-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.1.0-orange.svg?style=for-the-badge
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust
