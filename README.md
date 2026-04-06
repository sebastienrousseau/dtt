<p align="center">
  <img src="https://kura.pro/dtt/images/logos/dtt.svg" alt="DateTime (DTT) logo" width="128" />
</p>

<h1 align="center">DateTime (DTT)</h1>

<p align="center">
  <strong>A Rust library for parsing, validating, manipulating, and formatting dates and times.</strong>
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/dtt/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/dtt/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/dtt"><img src="https://img.shields.io/crates/v/dtt.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/dtt"><img src="https://img.shields.io/badge/docs.rs-dtt-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://codecov.io/gh/sebastienrousseau/dtt"><img src="https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&logo=codecov" alt="Coverage" /></a>
  <a href="https://lib.rs/crates/dtt"><img src="https://img.shields.io/badge/lib.rs-v0.0.10-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
</p>

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

You need [Rust](https://rustup.rs/) 1.80.0 or later. Works on macOS, Linux, and Windows.

---

## Overview

DTT provides ergonomic date and time handling in Rust. Parse, validate, manipulate, and format temporal values without the complexity of heavier datetime libraries.

- **Multi-format parsing** from common date/time string patterns
- **Validation** of components, ranges, and leap year logic
- **Arithmetic** — add, subtract, and compare dates and times
- **Custom formatting** to any string pattern

---

## Features

| | |
| :--- | :--- |
| **Parsing** | Parse dates and times from multiple string formats |
| **Validation** | Validate date/time components and ranges |
| **Manipulation** | Add, subtract, and compare dates and times |
| **Formatting** | Format dates and times to custom string patterns |
| **Timezone support** | UTC and timezone-aware operations |
| **Cross-platform** | Works on macOS, Linux, and Windows |

---

## Usage

```rust
use dtt::DateTime;

fn main() {
    let now = DateTime::new();
    println!("Current time: {}", now);

    let parsed = DateTime::parse("2024-01-15T10:30:00Z").unwrap();
    println!("Parsed: {}", parsed);
}
```

---

## Development

```bash
cargo build        # Build the project
cargo test         # Run all tests
cargo clippy       # Lint with Clippy
cargo fmt          # Format with rustfmt
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup, signed commits, and PR guidelines.

---

**THE ARCHITECT** \u1d2b [Sebastien Rousseau](https://sebastienrousseau.com)
**THE ENGINE** \u1d5e [EUXIS](https://euxis.co) \u1d2b Enterprise Unified Execution Intelligence System

---

## License

Dual-licensed under [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0) or [MIT](https://opensource.org/licenses/MIT), at your option.

<p align="right"><a href="#datetime-dtt">Back to Top</a></p>