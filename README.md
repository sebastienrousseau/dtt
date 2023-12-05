<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/dtt/images/logos/dtt.svg"
alt="DateTime (DTT) logo" height="261" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# DateTime (DTT)

A Rust library for parsing, validating, manipulating, and formatting dates and times

*Part of the [Mini Functions][0] family of libraries.*

<!-- markdownlint-disable MD033 MD041 -->
<center>
<!-- markdownlint-enable MD033 MD041 -->

![DTT Banner][banner]

[![Made With Rust][made-with-rust-badge]][14] [![Crates.io][crates-badge]][8] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][9] [![License][license-badge]][2] [![Codecov][codecov-badge]][15]

• [Website][1] • [Documentation][9] • [Report Bug][4] • [Request Feature][4] • [Contributing Guidelines][5]

<!-- markdownlint-disable MD033 MD041 -->
</center>
<!-- markdownlint-enable MD033 MD041 -->

![divider][divider]

## Overview 📖

The `DateTime (DTT)` library is a comprehensive and flexible tool that enables developers to manage dates and times with ease.

It offers a range of functions and data structures that allow you to perform various date and time operations with ease, such as determining the day of the month, hour of the day, working with ISO 8601 date and time formats, and many others.

The library supports the creation of new DateTime objects with either UTC or custom timezone specifications, ensuring that you always have accurate and relevant date and time information. Additionally, it provides a mechanism to validate input dates and times, ensuring that you always have accurate information to work with.

## Features ✨

The library `DateTime (DTT)` provides date and time types and methods to make it easier to manipulate dates and times. It uses the serde library to derive the Deserialize and Serialize traits to convert the `DateTime (DTT)` struct to and from various data formats. It also uses the time and regex crates to deal with time conversions and regular expressions respectively.

The `DateTime (DTT)` struct includes fields such as:

| Feature | Description |
| --- | --- |
| `day` | Day of the month: (01-31) |
| `hour` | Hour of the day: (00-23) |
| `iso_8601` | ISO 8601 date and time: (e.g. "2023-01-01T00:00:00+00:00") |
| `iso_week` | ISO week number: (1-53) |
| `microsecond` | Microsecond: (0-999999) |
| `minute` | Minute of the hour: (0-59) |
| `month` | Month: (e.g. "January") |
| `now` | Now object: (e.g. "2023-01-01") |
| `offset` | Offset from UTC: (e.g. "+00:00") |
| `ordinal` | Ordinal date: (1-366) |
| `second` | Second of the minute: (0-59) |
| `time` | Time object: (e.g. "00:00:00") |
| `tz` | Time zone object: (e.g. "UTC") |
| `weekday` | Weekday object: (e.g. "Monday") |
| `year` | Year object: (e.g. "2023") |

Each of which represents different aspects of a date and time.

The `DateTime (DTT)` struct has two methods to create instances: `new` and `new_with_tz`. `new` creates a new `DateTime (DTT)` object with UTC timezone, and `new_with_tz` creates a new `DateTime (DTT)` object with a custom timezone.

It also includes a method `is_valid_day` which checks if the input string represents a valid day of the week. It also includes a method `is_valid_month` which checks if the input string represents a valid month of the year.

## Getting Started 🚀

It takes just a few minutes to get up and running with `DateTime (DTT)`.

### Installation

To install `DateTime (DTT)`, you need to have the Rust toolchain installed on your machine. You can install the Rust toolchain by following the instructions on the [Rust website][14].

Once you have the Rust toolchain installed, you can install `DateTime (DTT)` using the following command:

```shell
cargo install dtt
```

You can then run the help command to see the available options:

```shell
dtt --help
```

### Requirements

The minimum supported Rust toolchain version is currently Rust **1.69.0** or later (stable). It is recommended that you install the latest stable version of Rust.

### Platform support

`DateTime (DTT)` is supported and tested on the following platforms:

#### Tier 1 platforms 🏆

| Operating System | Target | Description |
| --- | --- | --- |
| Linux   | aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture |
| Linux   | i686-unknown-linux-gnu | 32-bit Linux (kernel 3.2+, glibc 2.17+) |
| Linux   | x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) |
| macOS   | x86_64-apple-darwin | 64-bit macOS (10.7 Lion or later) |
| Windows | i686-pc-windows-gnu | 32-bit Windows (7 or later) |
| Windows | i686-pc-windows-msvc | 32-bit Windows (7 or later) |
| Windows | x86_64-pc-windows-gnu | 64-bit Windows (7 or later) |
| Windows | x86_64-pc-windows-msvc | 64-bit Windows (7 or later) |

#### Tier 2 platforms 🥈

| Operating System | Target | Description |
| --- | --- | --- |
| 64-bit Linux | x86_64-unknown-linux-musl | 64-bit Linux (kernel 2.6.32+, musl libc) |
| ARM64 Linux | aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture |
| ARM64 macOS | aarch64-apple-darwin | 64-bit macOS on Apple Silicon |
| ARM64 Windows | aarch64-pc-windows-msvc | 64-bit Windows (aarch64-pc-windows-msvc) |
| ARMv6 Linux | arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) |
| ARMv6 Linux, hardfloat | arm-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| ARMv7 Linux, hardfloat | armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) |
| FreeBSD  | x86_64-unknown-freebsd | 64-bit FreeBSD on x86-64 |
| MIPS (LE) Linux | mipsel-unknown-linux-gnu | MIPSel Linux (kernel 2.6.32+, glibc 2.11+) |
| MIPS Linux | mips-unknown-linux-gnu | MIPS Linux (kernel 2.6.32+, glibc 2.11+) |
| MIPS64 (LE) Linux | mips64el-unknown-linux-gnuabi64 | MIPS64el Linux (kernel 2.6.32+, glibc 2.11+) |
| MIPS64 Linux | mips64-unknown-linux-gnuabi64 | MIPS64 Linux (kernel 2.6.32+, glibc 2.11+) |
| NetBSD  | x86_64-unknown-netbsd | 64-bit NetBSD on x86-64 |
| PowerPC Linux | powerpc-unknown-linux-gnu | PowerPC Linux (kernel 3.2, glibc 2.17) |
| PPC64 Linux | powerpc64-unknown-linux-gnu | PowerPC64 Linux (kernel 3.2, glibc 2.17) |
| PPC64LE Linux | powerpc64le-unknown-linux-gnu | PowerPC64le Linux (kernel 3.2, glibc 2.17) |
| RISC-V Linux | riscv64gc-unknown-linux-gnu | RISC-V Linux (kernel 3.2, glibc 2.17) |
| S390x Linux | s390x-unknown-linux-gnu | s390x Linux (kernel 3.2, glibc 2.17) |

The [GitHub Actions][11] shows the platforms in which the `DateTime (DTT)` library tests are run.

### Documentation

> ℹ️ **Info:** Please check out our [website][1] for more information.
You can find our documentation on [docs.rs][9], [lib.rs][10] and [crates.io][8].

## Usage 📖

To use the `DateTime (DTT)` library in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
dtt = "0.0.5"
```

Add the following to your `main.rs` file:

```rust
extern crate dtt;
use dtt::*;
```

then you can use the functions in your application code.

### Examples

To get started with `DateTime (DTT)`, you can use the examples provided in the `examples` directory of the project.

To run the examples, clone the repository and run the following command in your terminal from the project root directory.

```shell
cargo run --example dtt
```

#### Example 1: Creating a new DateTime object

```rust
use dtt::DateTime;
use dtt::dtt_print;

fn main() {
    // Create a new DateTime object with the current UTC time
    let now = DateTime::new();
    dtt_print!(now);
}
```

#### Example 2: Creating a new DateTime object with a custom timezone

```rust
use dtt::DateTime;
use dtt::dtt_print;

fn main() {
    // Create a new DateTime object with a custom timezone (e.g., CEST)
    let paris_time = DateTime::new_with_tz("CEST");
    dtt_print!(paris_time);
}
```

Custom timezones supported by `DateTime (DTT)` are:

| Abbreviation | UtcOffset                           | Time Zone Description                    |
|--------------|-------------------------------------|------------------------------------------|
| ACDT         | `UtcOffset::from_hms(10, 30, 0)`    | Australian Central Daylight Time         |
| ACST         | `UtcOffset::from_hms(9, 30, 0)`     | Australian Central Standard Time         |
| ADT          | `UtcOffset::from_hms(-3, 0, 0)`     | Atlantic Daylight Time                    |
| AEDT         | `UtcOffset::from_hms(11, 0, 0)`     | Australian Eastern Daylight Time          |
| AEST         | `UtcOffset::from_hms(10, 0, 0)`     | Australian Eastern Standard Time          |
| AKDT         | `UtcOffset::from_hms(-8, 0, 0)`     | Alaska Daylight Time                      |
| AKST         | `UtcOffset::from_hms(-9, 0, 0)`     | Alaska Standard Time                      |
| AST          | `UtcOffset::from_hms(-4, 0, 0)`     | Atlantic Standard Time                    |
| AWST         | `UtcOffset::from_hms(8, 0, 0)`      | Australian Western Standard Time          |
| BST          | `UtcOffset::from_hms(1, 0, 0)`      | British Summer Time                       |
| CDT          | `UtcOffset::from_hms(-5, 0, 0)`     | Central Daylight Time                      |
| CEST         | `UtcOffset::from_hms(2, 0, 0)`      | Central European Summer Time              |
| CET          | `UtcOffset::from_hms(1, 0, 0)`      | Central European Time                     |
| CST          | `UtcOffset::from_hms(-6, 0, 0)`     | Central Standard Time                     |
| ECT          | `UtcOffset::from_hms(-4, 0, 0)`     | Eastern Caribbean Time                    |
| EDT          | `UtcOffset::from_hms(-4, 0, 0)`     | Eastern Daylight Time                      |
| EEST         | `UtcOffset::from_hms(3, 0, 0)`      | Eastern European Summer Time              |
| EET          | `UtcOffset::from_hms(2, 0, 0)`      | Eastern European Time                     |
| EST          | `UtcOffset::from_hms(-5, 0, 0)`     | Eastern Standard Time                     |
| GMT          | `UtcOffset::from_hms(0, 0, 0)`      | Greenwich Mean Time                       |
| HADT         | `UtcOffset::from_hms(-9, 0, 0)`     | Hawaii-Aleutian Daylight Time              |
| HAST         | `UtcOffset::from_hms(-10, 0, 0)`    | Hawaii-Aleutian Standard Time              |
| HKT          | `UtcOffset::from_hms(8, 0, 0)`      | Hong Kong Time                            |
| IST          | `UtcOffset::from_hms(5, 30, 0)`     | Indian Standard Time                      |
| IDT          | `UtcOffset::from_hms(3, 0, 0)`      | Israel Daylight Time                       |
| JST          | `UtcOffset::from_hms(9, 0, 0)`      | Japan Standard Time                       |
| KST          | `UtcOffset::from_hms(9, 0, 0)`      | Korean Standard Time                      |
| MDT          | `UtcOffset::from_hms(-6, 0, 0)`     | Mountain Daylight Time                    |
| MST          | `UtcOffset::from_hms(-7, 0, 0)`     | Mountain Standard Time                    |
| NZDT         | `UtcOffset::from_hms(13, 0, 0)`     | New Zealand Daylight Time                 |
| NZST         | `UtcOffset::from_hms(12, 0, 0)`     | New Zealand Standard Time                 |
| PDT          | `UtcOffset::from_hms(-7, 0, 0)`     | Pacific Daylight Time                      |
| PST          | `UtcOffset::from_hms(-8, 0, 0)`     | Pacific Standard Time                      |
| UTC          | `UtcOffset::from_hms(0, 0, 0)`      | Coordinated Universal Time                |
| WADT         | `UtcOffset::from_hms(8, 45, 0)`     | West Australian Daylight Time             |
| WAST         | `UtcOffset::from_hms(7, 0, 0)`      | West Australian Standard Time             |
| WEDT         | `UtcOffset::from_hms(1, 0, 0)`      | Western European Daylight Time            |
| WEST         | `UtcOffset::from_hms(1, 0, 0)`      | Western European Summer Time              |
| WET          | `UtcOffset::from_hms(0, 0, 0)`      | Western European Time                     |
| WST          | `UtcOffset::from_hms(8, 0, 0)`      | Western Standard Time                     |

#### Example 3: Formatting a DateTime object

```rust
use dtt::DateTime;
use dtt::dtt_print;

fn main() {
    // Create a new DateTime object with the current UTC time
    let now = DateTime::new();

    // Format the DateTime object as a string
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S");
    dtt_print!("Formatted time: {}", formatted_time);
}
```

#### Example 4: Parsing a string into a DateTime object

```rust
use dtt::DateTime;
use dtt::dtt_print;

fn main() {
    // Parse a string into a DateTime object
    let date_string = "2023-05-12T12:00:00+00:00";
    match DateTime::parse(date_string) {
        Ok(datetime) => dtt_print!("Parsed DateTime: {}", datetime),
        Err(err) => dtt_print!("Error parsing DateTime: {}", err),
    }
}
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain backward compatibility, `DateTime (DTT)` follows [semantic versioning][7].

## License 📝

The project is licensed under the terms of both the MIT license and the Apache License (Version 2.0).

- [Apache License, Version 2.0][2]
- [MIT license][3]

## Contribution 🤝

We welcome all people who want to contribute. Please see the [contributing instructions][5] for more information.

Contributions in any form (issues, pull requests, etc.) to this project must adhere to the [Rust's Code of Conduct][12].

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgements 💙

A big thank you to all the awesome contributors of the [DateTime (DTT) Library][6] for their help and support.

A special thank you goes to the [Rust Reddit][13] community for providing a lot of useful suggestions on how to improve this project.

[0]: https://minifunctions.com/ "MiniFunctions"
[1]: https://dttlib.one "DateTime (DTT) Library Website"
[2]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[3]: https://opensource.org/licenses/MIT "MIT license"
[4]: https://github.com/sebastienrousseau/dtt/issues "Issues"
[5]: https://github.com/sebastienrousseau/dtt/blob/main/CONTRIBUTING.md "Contributing Instructions"
[6]: https://github.com/sebastienrousseau/dtt/graphs/contributors "Contributors"
[7]: http://semver.org/ "Semantic Versioning"
[8]: https://crates.io/crates/dtt "Crates.io"
[9]: https://docs.rs/dtt "Docs.rs"
[10]: https://lib.rs/crates/dtt "Lib.rs"
[11]: https://github.com/sebastienrousseau/dtt/actions "GitHub Actions"
[12]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[13]: https://reddit.com/r/rust "Rust Reddit"
[14]: https://www.rust-lang.org "The Rust Programming Language"
[15]: https://codecov.io/gh/sebastienrousseau/dtt "Codecov"

[banner]: https://kura.pro/dtt/images/titles/title-dtt.svg 'DateTime (DTT) banner'
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&token=X3ZP0K1SGI 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/dtt.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/dtt.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.4-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/dtt.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
