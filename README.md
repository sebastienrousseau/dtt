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

The `DateTime (DTT)` library is a comprehensive and flexible tool that enables developers to manage dates and times with ease. It offers a range of functions and data structures that allow you to perform various date and time operations with ease, such as determining the day of the month, hour of the day, working with ISO 8601 date and time formats, and many others.

The library supports the creation of new `DateTime` objects with either UTC or custom timezone specifications, ensuring that you always have accurate and relevant date and time information. Additionally, it provides a mechanism to validate input dates and times, ensuring that you always have accurate information to work with.

## Features ✨

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

It also includes methods to validate various date and time components, such as `is_valid_day`, `is_valid_month`, `is_valid_time`, `is_valid_iso_8601`, and more.

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

`DateTime (DTT)` is supported and tested on a wide range of platforms, including various Linux distributions, macOS, and Windows. You can find the full list of supported platforms in the [Platform support](#platform-support-) section.

### Documentation

> ℹ️ **Info:** Please check out our [website][1] for more information.
You can find our documentation on [docs.rs][9], [lib.rs][10] and [crates.io][8].

## Usage 📖

To use the `DateTime (DTT)` library in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
dtt = "0.0.6"
```

Add the following to your `main.rs` file:

```rust
extern crate dtt;
use dtt::*;
```

then you can use the functions in your application code.

### Examples

The library provides several examples to get you started, covering common use cases such as:

#### Creating and Manipulating DateTime Objects

1. **Creating a new DateTime object**:
   ```rust
   use dtt::DateTime;
   use dtt::dtt_print;

   fn main() {
       // Create a new DateTime object with the current UTC time
       let now = DateTime::new();
       dtt_print!(now);
   }
   ```

2. **Creating a new DateTime object with a custom timezone**:
   ```rust
   use dtt::DateTime;
   use dtt::dtt_print;

   fn main() {
       // Create a new DateTime object with a custom timezone (e.g., CEST)
       let paris_time = DateTime::new_with_tz("CEST");
       dtt_print!(paris_time);
   }
   ```

3. **Formatting a DateTime object**:
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

4. **Parsing a string into a DateTime object**:
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

#### Validating DateTime Components

5. **Validating a day**:
   ```rust
   println!(
       "🦀 Valid day (32):    ❌ {}",
       DateTime::is_valid_day("32")
   );
   println!(
       "🦀 Valid day:         ✅ {}",
       DateTime::is_valid_day(&date.day.to_string())
   );
   ```

6. **Validating an hour**:
   ```rust
   println!(
       "🦀 Valid hour (24):   ❌ {}",
       DateTime::is_valid_hour("24")
   );
   println!(
       "🦀 Valid hour:        ✅ {}",
       DateTime::is_valid_hour(&date.hour.to_string())
   );
   ```

7. **Validating a minute**:
   ```rust
   assert!(DateTime::is_valid_minute("59"));
   assert!(!DateTime::is_valid_minute("60"));
   ```

8. **Validating a month**:
   ```rust
   assert!(DateTime::is_valid_month("12"));
   assert!(!DateTime::is_valid_month("13"));
   ```

9. **Validating a second**:
   ```rust
   assert!(DateTime::is_valid_second("59"));
   assert!(!DateTime::is_valid_second("60"));
   ```

10. **Validating a microsecond**:
    ```rust
    assert!(DateTime::is_valid_microsecond("999999"));
    assert!(!DateTime::is_valid_microsecond("1000000"));
    ```

11. **Validating an ordinal date**:
    ```rust
    assert!(DateTime::is_valid_ordinal("366"));
    assert!(!DateTime::is_valid_ordinal("367"));
    ```

12. **Validating a time**:
    ```rust
    assert!(DateTime::is_valid_time("23:59:59"));
    assert!(!DateTime::is_valid_time("24:00:00"));
    ```

13. **Validating an ISO 8601 date and time**:
    ```rust
    assert!(DateTime::is_valid_iso_8601("2023-05-11T17:30:00Z"));
    assert!(DateTime::is_valid_iso_8601("2023-05-11T17:30:00+01:00"));
    ```

#### Other Examples

14. **Getting the next day**:
    ```rust
    let nd = DateTime::next_day(&date);
    println!("🦀 Next day:          ✅ {}", nd.day);
    ```

15. **Getting the previous day**:
    ```rust
    let pd = DateTime::previous_day(&date);
    println!("🦀 Previous day:      ✅ {}", pd.day);
    ```

16. **Parsing a DateTime from a string**:
    ```rust
    let date_str = "2022-01-01T12:00:00+01:00";
    let result: Result<DateTime, dtt::DateTimeError> =
        DateTime::from_str(date_str);
    println!("🦀 from_str():        ✅ {:?}", result);
    println!("🦀 from_str(day):     ✅ {:?}", result.unwrap().day);
    ```

17. **Calculating a relative delta**:
    ```rust
    let mut dt = DateTime::new();
    dt.day = 11;
    dt.hour = 8;
    dt.iso_week = 19;
    dt.microsecond = 0;
    dt.minute = 8;
    dt.month = String::from("05");
    dt.second = 0;
    dt.year = 1975;

    let new_dt = dt.relative_delta();
    println!("🦀 Rd day:(11)        ✅ {}", new_dt.day);
    println!("🦀 Rd hour:(08)       ✅ {}", new_dt.hour);
    println!("🦀 Rd week:(19)       ✅ {}", new_dt.iso_week);
    println!("🦀 Rd ms:(000000)     ✅ {}", new_dt.microsecond);
    println!("🦀 Rd minute:(08)     ✅ {}", new_dt.minute);
    println!("🦀 Rd month:(05)      ✅ {}", new_dt.month);
    println!("🦀 Rd second:(00)     ✅ {}", new_dt.second);
    println!("🦀 Rd year:(1975)     ✅ {}", new_dt.year);
    ```

You can find these examples in the `examples` directory of the project.

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

## Troubleshooting

If you encounter any issues while using the `DateTime (DTT)` library, please check the following:

1. Ensure that you have the correct Rust toolchain installed and that it meets the minimum version requirement.
2. Verify that you have correctly added the library to your project's dependencies.
3. Check the documentation and examples for the specific functionality you're trying to use.
4. If the issue persists, please [report a bug][4] so that the project maintainers can assist you.

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
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.6-orange.svg?style=for-the-badge 'Lib.rs badge'
[license-badge]: https://img.shields.io/crates/l/dtt.svg?style=for-the-badge 'License badge'
[made-with-rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust badge'
