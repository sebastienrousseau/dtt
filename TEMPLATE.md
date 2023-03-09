<!-- markdownlint-disable MD041 -->
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

## Changelog 📚

-

[0]: https://minifunctions.com
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/dtt/issues
[4]: https://raw.githubusercontent.com/sebastienrousseau/dtt/main/CONTRIBUTING.md
[6]: https://github.com/sebastienrousseau/dtt/graphs/contributors
[8]: https://crates.io/crates/dtt
[9]: https://docs.rs/dtt
[10]: https://lib.rs/crates/dtt
[11]: https://codecov.io/github/sebastienrousseau/dtt

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/dtt/banners/banner-dtt-1597x377.svg "DTT Banner"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/dtt?style=for-the-badge&token=X3ZP0K1SGI 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/dtt.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/dtt.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.3-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/dtt.svg?style=for-the-badge 'License'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
