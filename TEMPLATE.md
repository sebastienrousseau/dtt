<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/dtt/images/logos/dtt.svg"
alt="DateTime (DTT) logo" height="261" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# DateTime (DTT)

## A Rust library for parsing, validating, manipulating, and formatting dates and times

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

[0]: https://minifunctions.com/ "MiniFunctions"
[1]: https://dttlib.one "DateTime (DTT) Library Website"
[2]: https://opensource.org/license/apache-2-0/ "Apache License, Version 2.0"
[4]: https://github.com/sebastienrousseau/dtt/issues "Issues"
[5]: https://github.com/sebastienrousseau/dtt/blob/main/CONTRIBUTING.md "Contributing Instructions"
[8]: https://crates.io/crates/dtt "Crates.io"
[9]: https://docs.rs/dtt "Docs.rs"
[10]: https://lib.rs/crates/dtt "Lib.rs"
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

## Changelog 📚
