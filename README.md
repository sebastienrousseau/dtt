<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/dtt/images/logos/dtt.svg"
alt="DateTime (DTT) logo" height="261" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# DateTime (DTT)

A Rust library for parsing, validating, manipulating, and formatting dates and
times

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

The `DateTime (DTT)` library is a comprehensive and flexible tool that enables
developers to manage dates and times with ease. It offers a range of functions
and data structures that allow you to perform various date and time operations
with ease, such as determining the day of the month, hour of the day, working
with ISO 8601 date and time formats, and many others.

The library supports the creation of new `DateTime` objects with either UTC or
custom timezone specifications, ensuring that you always have accurate and
relevant date and time information. Additionally, it provides a mechanism to
validate input dates and times, ensuring that you always have accurate
information to work with.

## Features ✨

The `DateTime (DTT)` struct includes the following fields and methods:

### Fields

| Feature | Description | Type |
| --- | --- | --- |
| `day` | Day of the month: (1-31) | `u8` |
| `hour` | Hour of the day: (0-23) | `u8` |
| `iso_8601` | ISO 8601 date and time: (e.g. "2023-01-01T00:00:00+00:00") | `String` |
| `iso_week` | ISO week number: (1-53) | `u8` |
| `microsecond` | Microsecond: (0-999999) | `u32` |
| `minute` | Minute of the hour: (0-59) | `u8` |
| `month` | Month: (e.g. "January") | `String` |
| `now` | Now object: (e.g. "2023-01-01") | `String` |
| `offset` | Offset from UTC: (e.g. "+00:00") | `String` |
| `ordinal` | Ordinal date: (1-366) | `u16` |
| `second` | Second of the minute: (0-59) | `u8` |
| `time` | Time object: (e.g. "00:00:00") | `String` |
| `tz` | Time zone object: (e.g. "UTC") | `String` |
| `weekday` | Weekday object: (e.g. "Monday") | `String` |
| `year` | Year object: (e.g. "2023") | `i32` |

### Methods

- `new()`: Creates a new `DateTime` object with the current UTC time.
- `new_with_tz(tz: &str)`: Creates a new `DateTime` object with the specified
  timezone.
- `is_valid_day(input: &str)`: Checks if the input represents a valid day of the
  month.
- `is_valid_hour(input: &str)`: Checks if the input represents a valid hour of
  the day.
- `is_valid_second(input: &str)`: Checks if the input represents a valid second
  of the minute.
- `is_valid_minute(input: &str)`: Checks if the input represents a valid minute
  of the hour.
- `is_valid_month(input: &str)`: Checks if the input represents a valid month of
  the year.
- `is_valid_ordinal(input: &str)`: Checks if the input represents a valid
  ordinal date.
- `is_valid_time(input: &str)`: Checks if the input represents a valid time.
- `is_valid_iso_week(input: &str)`: Checks if the input represents a valid ISO
  week number.
- `is_valid_iso_8601(input: &str)`: Checks if the input represents a valid ISO
  8601 date and time.
- `is_valid_microsecond(input: &str)`: Checks if the input represents a valid
  microsecond.
- `update(&mut self)`: Updates the `DateTime` object with the current date and
  time based on the timezone.
- `add_days(&self, days: i32)`: Creates a new `DateTime` object with the
  specified number of days added.
- `next_day(&self)`: Creates a new `DateTime` object representing the next day.
- `previous_day(&self)`: Creates a new `DateTime` object representing the
  previous day.
- `relative_delta(&self)`: Creates a new `DateTime` object with the relative
  delta based on the current date and time.
- `format(&self, format_str: &str)`: Formats the `DateTime` object as a string
  using the specified format.

The library also provides various getter methods to extract the individual
components of the `DateTime` object, such as `year()`, `month()`, `day()`,
`hour()`, `minute()`, `second()`, `microsecond()`, `weekday()`, `ordinal()`,
`iso_8601()`, `iso_week()`, `time()`, `tz()`, and `offset()`.

Additionally, the `DateTime (DTT)` struct implements the `FromStr` trait,
allowing for parsing a string into a `DateTime` object.

## Getting Started 🚀

It takes just a few minutes to get up and running with `DateTime (DTT)`.

### Installation

To install `DateTime (DTT)`, you need to have the Rust toolchain installed on
your machine. You can install the Rust toolchain by following the instructions
on the [Rust website][14].

Once you have the Rust toolchain installed, you can install `DateTime (DTT)`
using the following command:

```shell
cargo install dtt
```

You can then run the help command to see the available options:

```shell
dtt --help
```

### Requirements

The minimum supported Rust toolchain version is currently Rust **1.56.0** or later
(stable). It is recommended that you install the latest stable version of Rust.

### Platform support

`DateTime (DTT)` is supported and tested on a wide range of platforms, including
various Linux distributions, macOS, and Windows.

### Documentation

> ℹ️ **Info:** Please check out our [website][1] for more information.
You can find our documentation on [docs.rs][9], [lib.rs][10] and [crates.io][8].

## Usage 📖

To use the `DateTime (DTT)` library in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
dtt = "0.0.7"
```

Add the following to your `main.rs` file:

```rust
extern crate dtt;
use dtt::*;
```

then you can use the functions in your application code.

### Examples and Test Cases

The library provides several examples and test cases to help you get started.
You can find these in the `examples` and `tests` directories of the project.

#### Example 1: Creating a new DateTime object and printing it

```rust
// Import the DateTime struct and the dtt_print macro
use dtt::DateTime;
use dtt::dtt_print;

#[test]
fn example_1() {
    // Create a new DateTime object and print it
    let now = DateTime::new();
    dtt_print!(now);
}
```

#### Example 2: Validate methods for various date and time components

```rust
// Import the DateTime struct and the validation macros
use dtt::DateTime;
use dtt::{
    is_valid_day,
    is_valid_hour,
    is_valid_minute,
    is_valid_month,
    is_valid_second,
    is_valid_microsecond,
    is_valid_ordinal,
    is_valid_time,
    is_valid_iso_8601
};

// Validate microsecond values
assert!(DateTime::is_valid_microsecond("999999"));
assert!(!DateTime::is_valid_microsecond("1000000"));

// Validate second values
assert!(DateTime::is_valid_second("59"));
assert!(!DateTime::is_valid_second("60"));

// Validate minute values
assert!(DateTime::is_valid_minute("59"));
assert!(!DateTime::is_valid_minute("60"));

// Validate hour values
assert!(DateTime::is_valid_hour("23"));
assert!(!DateTime::is_valid_hour("24"));

// Validate month values
assert!(DateTime::is_valid_month("12"));
assert!(!DateTime::is_valid_month("13"));

// Validate year values
assert!(DateTime::is_valid_ordinal("366"));
assert!(!DateTime::is_valid_ordinal("367"));

// Validate time values
assert!(DateTime::is_valid_time("23:59:59"));
assert!(!DateTime::is_valid_time("24:00:00"));

// Validate ISO 8601 values
assert!(DateTime::is_valid_iso_8601("2023-05-11T17:30:00Z"));
assert!(DateTime::is_valid_iso_8601("2023-05-11T17:30:00+01:00"));
```

#### Example 3: Validate string to `DateTime` conversion

```rust
use dtt::DateTime;
use std::str::FromStr;

let date_str = "2022-01-01T12:00:00+01:00";
let mut result: Result<DateTime, dtt::DateTimeError> = DateTime::from_str(date_str);

assert_eq!(result.as_mut().unwrap().iso_8601, date_str);
assert_eq!(result.as_mut().unwrap().year, 2022);
assert_eq!(result.as_mut().unwrap().month, "01");
assert_eq!(result.as_mut().unwrap().day, 1);
assert_eq!(result.as_mut().unwrap().hour, 12);
assert_eq!(result.as_mut().unwrap().minute, 0);
assert_eq!(result.as_mut().unwrap().second, 0);
assert_eq!(result.as_mut().unwrap().offset, "+01:00");
assert_eq!(result.as_mut().unwrap().time, "12:00:00");
assert_eq!(result.as_mut().unwrap().tz, "CET");
assert_eq!(result.as_mut().unwrap().iso_week, 52);
assert_eq!(result.as_mut().unwrap().weekday, "Saturday");
assert_eq!(result.as_mut().unwrap().ordinal, 1);
assert_eq!(result.as_mut().unwrap().microsecond, 0);
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain backward
compatibility, `DateTime (DTT)` follows [semantic versioning][7].

## License 📝

The project is licensed under the terms of both the MIT license and the Apache
License (Version 2.0).

- [Apache License, Version 2.0][2]
- [MIT license][3]

## Contribution 🤝

We welcome all people who want to contribute. Please see the
[contributing instructions][5] for more information.

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to the [Rust's Code of Conduct][12].

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Troubleshooting

If you encounter any issues while using the `DateTime (DTT)` library, please
check the following:

1. Ensure that you have the correct Rust toolchain installed and that it meets
   the minimum version requirement.
2. Verify that you have correctly added the library to your project's
   dependencies.
3. Check the documentation and examples for the specific functionality you're
   trying to use.
4. If the issue persists, please [report a bug][4] so that the project
   maintainers can assist you.

## Acknowledgements 💙

A big thank you to all the awesome contributors of the
[DateTime (DTT) Library][6] for their help and support.

A special thank you goes to the [Rust Reddit][13] community for providing a lot
of useful suggestions on how to improve this project.

[0]: https://minifunctions.com/ "MiniFunctions"
[1]: https://dttlib.com "DateTime (DTT) Library Website"
[2]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[3]: https://opensource.org/licenses/MIT "MIT license"
[4]: https://github.com/sebastienrousseau/dtt/issues "Issues"
[5]: https://github.com/sebastienrousseau/dtt/blob/main/CONTRIBUTING.md "Contributing Instructions"
[6]: https://github.com/sebastienrousseau/dtt/graphs/contributors "Contributors"
[7]: http://semver.org/ "Semantic Versioning"
[8]: https://crates.io/crates/dtt "Crates.io"
[9]: https://docs.rs/dtt "Docs.rs"
[10]: https://lib.rs/crates/dtt "Lib.rs"
[12]: https://www.rust-lang.org/policies/code-of-conduct "Rust's Code of Conduct"
[13]: https://reddit.com/r/rust "Rust Reddit"
[14]: https://www.rust-lang.org "The Rust Programming Language"
[15]: https://codecov.io/gh/sebastienrousseau/dtt "Codecov"

[banner]: https://kura.pro/dtt/images/titles/title-dtt.svg 'DateTime (DTT) banner'
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&token=X3ZP0K1SGI 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/dtt.svg?style=for-the-badge 'Crates.io badge'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/dtt.svg?style=for-the-badge 'Docs.rs badge'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.7-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/dtt.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
