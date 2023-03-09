//! Macros for the `dtt` crate.
//!
//! This module bundles all macros used across the `dtt` crate.
//! These include macros for validating input, and macros for
//! generating the `DateTime` struct.
//!

/// Macro for validating a string input against a regex.
///
/// This macro generates a function that takes a string as input and returns a boolean.
///
/// It takes two arguments:
///
/// - `$name`: The name of the function to be generated.
/// - `$type`: The type of the input to be validated.
///
/// The function returns `true` if the input can be parsed into the given type, and `false` otherwise.
///
/// The macro can be used to validate multiple types of DateTime methods such as:
///
/// | Macro | Description |
/// |--------|------------|
/// | `is_valid!(day, u32)` | Validates that a `day` is valid. |
/// | `is_valid!(hour, u32)` | Validates that an `hour` is valid. |
/// | `is_valid!(minute, u32)` | Validates that a `minute` is valid. |
/// | `is_valid!(month, u32)` | Validates that a `month` is valid. |
/// | `is_valid!(second, u32)` | Validates that a `second` is valid. |
/// | `is_valid_time!(time, u32)` | Validates that a `time` is valid. |
/// | `is_valid_iso_8601!(iso_8601, u32)` | Validates that a `iso_8601` is valid. |
///
/// # Examples
///
/// ```
/// use dtt::is_valid;
/// use dtt::DateTime;
///
/// // Validation of a day
/// let input = "31".to_string();
/// is_valid!(day, String);
/// let result = day(&input);
/// assert!(result);
///
/// // Validation of an hour
/// let input = "23".to_string();
/// is_valid!(hour, String);
/// let result = hour(&input);
/// assert!(result);
///
///
/// // Validation of a minute
/// let input = "59".to_string();
/// is_valid!(minute, String);
/// let result = minute(&input);
/// assert!(result);
///
/// // Validation of a month
/// let input = "12".to_string();
/// is_valid!(month, String);
/// let result = month(&input);
/// assert!(result);
///
/// // Validation of a second
/// let input = "59".to_string();
/// is_valid!(second, String);
/// let result = second(&input);
/// assert!(result);
///
/// // Validation of a microsecond
/// let input = "999999".to_string();
/// is_valid!(microsecond, String);
/// let result = microsecond(&input);
/// assert!(result);
///
/// // Validation of an ordinal
/// let input = "365".to_string();
/// is_valid!(ordinal, String);
/// let result = ordinal(&input);
/// assert!(result);
///
/// // Validation of a time
/// let input = "23:59:59".to_string();
/// is_valid!(time, String);
/// let result = time(&input);
/// assert!(result);
///
/// // Validation of an iso_8601
/// let input = "2022-01-01T12:00:00+01:00".to_string();
/// is_valid!(iso_8601, String);
/// let result = iso_8601(&input);
/// ```
#[macro_export]
macro_rules! is_valid {
    ( $name:ident, $type:ty ) => {
        fn $name(input: &str) -> bool {
            match input.parse::<$type>() {
                Ok(parsed_val) => match stringify!($name) {
                    "day" => {
                        DateTime::is_valid_day(&parsed_val.to_string())
                    }
                    "hour" => {
                        DateTime::is_valid_hour(&parsed_val.to_string())
                    }
                    "minute" => DateTime::is_valid_minute(
                        &parsed_val.to_string(),
                    ),
                    "month" => DateTime::is_valid_month(
                        &parsed_val.to_string(),
                    ),
                    "second" => DateTime::is_valid_second(
                        &parsed_val.to_string(),
                    ),
                    "microsecond" => DateTime::is_valid_microsecond(
                        &parsed_val.to_string(),
                    ),
                    "ordinal" => DateTime::is_valid_ordinal(
                        &parsed_val.to_string(),
                    ),
                    "time" => {
                        DateTime::is_valid_time(&parsed_val.to_string())
                    }
                    "iso_8601" => DateTime::is_valid_iso_8601(
                        &parsed_val.to_string(),
                    ),
                    _ => false,
                },
                Err(_) => false,
            }
        }
    };
}
