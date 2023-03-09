# DTT

## A Rust library for parsing, validating, manipulating, and formatting dates and times

[![Made With Love][made-with-rust]][6] [![Crates.io][crates-badge]][8] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][9] [![License][license-badge]][2] [![Codecov][codecov-badge]][11]

![divider][divider]

## Welcome to DTT 👋

![DTT Banner][banner]

<!-- markdownlint-disable MD033 -->
<center>

**[Website][0]
• [Documentation][9]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]**

</center>

<!-- markdownlint-enable MD033 -->

## Overview 📖

The DateTime (DTT) library is a comprehensive and flexible tool that
enables developers to manage dates and times with ease.

It offers a range of functions and data structures that allow you to
perform various date and time operations with ease, such as determining
the day of the month, hour of the day, working with ISO 8601 date and
time formats, and many others.

The library supports the creation of new DateTime objects with either
UTC or custom timezone specifications, ensuring that you always have
accurate and relevant date and time information. Additionally, it
provides a mechanism to validate input dates and times, ensuring that
you always have accurate information to work with.

## Features ✨

The library `DateTime` provides date and time types and methods to make
it easier to manipulate dates and times. It uses the serde library to
derive the Deserialize and Serialize traits to convert the `DateTime`
struct to and from various data formats. It also uses the time and regex
crates to deal with time conversions and regular expressions
respectively.

The `DateTime` struct includes fields such as:

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

The `DateTime` struct has two methods to create instances: `new` and
`new_with_tz`. `new` creates a new `DateTime` object with UTC timezone,
and `new_with_tz` creates a new `DateTime` object with a custom
timezone.

It also includes a method `is_valid_day` which checks if the input
string represents a valid day of the week. It also includes a method
`is_valid_month` which checks if the input string represents a valid
month of the year.

## Getting Started 🚀

It takes just a few seconds to get up and running with `DateTime`.

### Installation

To install `DateTime`, you need to have the Rust toolchain installed on
your machine. You can install the Rust toolchain by following the
instructions on the [Rust website][13].

Once you have the Rust toolchain installed, you can install `DateTime`
using the following command:

```shell
cargo install dtt
```

### Requirements

The minimum supported Rust toolchain version is currently Rust `1.67.1`
or later (stable).

### Platform support

`DateTime` is supported and has been tested on the following platforms:

#### FreeBSD targets 🐬

| Target | Description | Status |
|--------|-------------|--------|
| x86_64-unknown-freebsd | 64-bit FreeBSD on x86-64 | ✅ Tested |

#### Linux targets 🐧

| Target | Description | Status |
|--------|-------------|--------|
| aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture | ✅ Tested |
| aarch64-unknown-linux-musl | 64-bit Linux systems on ARM architecture | ✅ Tested |
| arm-unknown-linux-gnueabi | ARMv6 Linux (kernel 3.2, glibc 2.17) | ✅ Tested |
| armv7-unknown-linux-gnueabihf | ARMv7 Linux, hardfloat (kernel 3.2, glibc 2.17) | ✅ Tested |
| i686-unknown-linux-gnu | 32-bit Linux (kernel 3.2+, glibc 2.17+) | ✅ Tested |
| i686-unknown-linux-musl | 32-bit Linux (kernel 3.2+, musl libc) | ✅ Tested |
| x86_64-unknown-linux-gnu | 64-bit Linux (kernel 2.6.32+, glibc 2.11+) | ✅ Tested |
| x86_64-unknown-linux-musl | 64-bit Linux (kernel 2.6.32+, musl libc) | ✅ Tested |

#### Illumos targets 🌞

| Target | Description | Status |
|--------|-------------|--------|
| x86_64-unknown-illumos | 64-bit Illumos on x86-64 | ✅ Tested |

#### macOS targets 🍎

| Target | Description | Status |
|--------|-------------|--------|
| aarch64-apple-darwin | 64-bit macOS on Apple Silicon | ✅ Tested |
| x86_64-apple-darwin | 64-bit macOS (10.7 Lion or later) | ✅ Tested |

The [GitHub Actions][12] shows the platforms in which the `DateTime`
library tests are run.

Should you encounter any issues with the library on any of the above
platforms, please [report a bug][3]. We will do our best to resolve the
issue as soon as possible. If you would like to contribute to help us to
support additional platforms, please submit a pull request.

### Documentation

> ℹ️ **Info:** Do check out our [website][0] for more information and
find our documentation on [docs.rs][9], [lib.rs][10] and [crates.io][8].

## Usage 📖

To use `DateTime` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
dtt = "0.0.3"
```

Add the following to your `main.rs` file:

```rust
extern crate dtt;
use dtt::*;
```

then you can use the functions in your application code.

### Examples

`DateTime` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example dtt
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `DateTime` follows [semantic versioning][7].

## License 📝

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][1]
- [MIT license][2]

## Contribution 🤝

We welcome all people who want to contribute. Please see the
[contributing instructions][4] for more information.

Contributions in any form (issues, pull requests, etc.) to this project
must adhere to the [Rust's Code of Conduct][14].

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Acknowledgements 💙

A big thank you to all the awesome contributors of [DateTime (DTT)][6]
for their help and support. A special thank you goes to the
[Rust Reddit](https://www.reddit.com/r/rust/) community for providing a
lot of useful suggestions on how to improve this project.

[0]: https://minifunctions.com
[1]: http://www.apache.org/licenses/LICENSE-2.0
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/dtt/issues
[4]: https://raw.githubusercontent.com/sebastienrousseau/dtt/main/CONTRIBUTING.md
[6]: https://github.com/sebastienrousseau/dtt/graphs/contributors
[7]: http://semver.org/
[8]: https://crates.io/crates/dtt
[9]: https://docs.rs/dtt
[10]: https://lib.rs/crates/dtt
[11]: https://codecov.io/github/sebastienrousseau/dtt
[12]: https://github.com/sebastienrousseau/dtt/actions
[13]: https://www.rust-lang.org/learn/get-started
[14]: https://www.rust-lang.org/policies/code-of-conduct

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/dtt/banners/banner-dtt-1597x377.svg "DTT Banner"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&token=X3ZP0K1SGI 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/dtt.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/dtt.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.3-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/dtt.svg?style=for-the-badge 'License'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
