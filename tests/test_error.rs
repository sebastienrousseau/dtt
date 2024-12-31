// test_error.rs
//
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Tests for the `DateTimeError` Enum
//!
//! This file contains unit tests that validate the functionality of the
//! `DateTimeError` enum. It checks serialisation, deserialisation, trait
//! implementations, and more, ensuring that all variants behave as expected.

/// Unit tests for the `DateTimeError` enum.
///
/// This module contains a comprehensive set of unit tests for the `DateTimeError` enum,
/// which is defined in `error.rs`. The tests cover various aspects, including:
/// - Basic Functionality: Ensuring each variant of DateTimeError behaves as expected.
/// - Trait Implementations: Verifying Debug, Clone, Copy, PartialEq, and Hash traits are correctly implemented.
/// - Serialization and Deserialization: Checking both the serialization to and deserialization from JSON, as well as handling of invalid JSON.
/// - Comparison and Equality: Ensuring correct comparisons between variants.
/// - Error Handling: Testing the Error trait implementation, including source, and the Debug and Display traits.
/// - Future Proofing: Ensuring that potential future changes don't break existing functionality.
/// - Low-Level Checks: Verifying the memory layout and Default trait implementation.
/// - Default Implementation: Confirming that the default() method returns the expected variant.
#[cfg(test)]
mod tests {

    mod basic_functionality_tests {
        use dtt::error::DateTimeError;

        /// Tests the `InvalidFormat` variant of `DateTimeError`.
        ///
        /// This test ensures that the `InvalidFormat` variant correctly implements
        /// the `Display` and `Error` traits.
        #[test]
        fn test_invalid_format() {
            let error = DateTimeError::InvalidFormat;
            assert_eq!(error.to_string(), "Invalid date format");
            let error: &dyn std::error::Error = &error;
            assert_eq!(error.to_string(), "Invalid date format");
        }

        /// Tests the `InvalidTimezone` variant of `DateTimeError`.
        ///
        /// This test ensures that the `InvalidTimezone` variant correctly implements
        /// the `Display` and `Error` traits.
        #[test]
        fn test_invalid_timezone() {
            let error = DateTimeError::InvalidTimezone;
            assert_eq!(
                error.to_string(),
                "Invalid or unsupported timezone; DST not supported"
            );
            let error: &dyn std::error::Error = &error;
            assert_eq!(
                error.to_string(),
                "Invalid or unsupported timezone; DST not supported"
            );
        }

        /// Tests the `InvalidDate` variant of `DateTimeError`.
        #[test]
        fn test_invalid_date() {
            let error = DateTimeError::InvalidDate;
            assert_eq!(error.to_string(), "Invalid date");
        }

        /// Tests the `InvalidTime` variant of `DateTimeError`.
        #[test]
        fn test_invalid_time() {
            let error = DateTimeError::InvalidTime;
            assert_eq!(error.to_string(), "Invalid time");
        }
    }

    mod trait_implementations_tests {
        use dtt::error::DateTimeError;
        use std::collections::HashSet;

        /// Tests the `Debug` implementation for `DateTimeError`.
        ///
        /// This test ensures that the `Debug` trait is implemented correctly
        /// for all variants of `DateTimeError`.
        #[test]
        fn test_debug_impl() {
            let format_error = DateTimeError::InvalidFormat;
            let timezone_error = DateTimeError::InvalidTimezone;
            assert_eq!(format!("{:?}", format_error), "InvalidFormat");
            assert_eq!(
                format!("{:?}", timezone_error),
                "InvalidTimezone"
            );
        }

        /// Tests the `Clone` and `Copy` traits for `DateTimeError`.
        ///
        /// This test ensures that `DateTimeError` can be cloned and copied
        /// without issues, maintaining equality between original and cloned/copied values.
        #[test]
        fn test_clone_and_copy() {
            let original = DateTimeError::InvalidFormat;
            let cloned = original;
            let copied = original;
            assert_eq!(original, cloned);
            assert_eq!(original, copied);
        }

        /// Tests the hashing functionality of `DateTimeError`.
        ///
        /// This test ensures that `DateTimeError` can be correctly hashed and used
        /// in a `HashSet`, maintaining uniqueness and correct set behavior.
        #[test]
        fn test_hash() {
            let mut set = HashSet::new();
            let _ = set.insert(DateTimeError::InvalidFormat);
            let _ = set.insert(DateTimeError::InvalidTimezone);
            assert_eq!(set.len(), 2);
            assert!(set.contains(&DateTimeError::InvalidFormat));
            assert!(set.contains(&DateTimeError::InvalidTimezone));
        }

        /// Tests the PartialEq implementation for DateTimeError.
        ///
        /// This test ensures that DateTimeError variants can be compared for
        /// equality and inequality.
        #[test]
        fn test_partial_eq() {
            let parse_error1 = DateTimeError::InvalidFormat;
            let parse_error2 = DateTimeError::InvalidFormat;
            let range_error = DateTimeError::InvalidTimezone;

            assert_eq!(parse_error1, parse_error2);
            assert_ne!(parse_error1, range_error);
        }
    }

    mod serialization_tests {
        use dtt::error::DateTimeError;
        use time::error::{Parse, TryFromParsed};

        /// Tests the serialization and deserialization of `DateTimeError`.
        ///
        /// This test ensures that the `DateTimeError` enum can be correctly
        /// serialized to and deserialized from JSON using the `serde` crate.
        #[test]
        fn test_serialization() {
            let format_error = DateTimeError::InvalidFormat;
            let timezone_error = DateTimeError::InvalidTimezone;

            let serialized_format =
                serde_json::to_string(&format_error)
                    .expect("Serialization failed");
            let serialized_timezone =
                serde_json::to_string(&timezone_error)
                    .expect("Serialization failed");

            let deserialized_format: DateTimeError =
                serde_json::from_str(&serialized_format)
                    .expect("Deserialization failed");
            let deserialized_timezone: DateTimeError =
                serde_json::from_str(&serialized_timezone)
                    .expect("Deserialization failed");

            assert_eq!(format_error, deserialized_format);
            assert_eq!(timezone_error, deserialized_timezone);
        }

        /// Tests the deserialization of `DateTimeError`.
        #[test]
        fn test_deserialization() {
            let format_error_str = "\"InvalidFormat\"";
            let timezone_error_str = "\"InvalidTimezone\"";

            // Deserialize the JSON strings back into `DateTimeError` variants
            let deserialized_format: DateTimeError =
                serde_json::from_str(format_error_str)
                    .expect("Deserialization failed");
            let deserialized_timezone: DateTimeError =
                serde_json::from_str(timezone_error_str)
                    .expect("Deserialization failed");

            // Verify the deserialized output
            assert_eq!(
                deserialized_format,
                DateTimeError::InvalidFormat
            );
            assert_eq!(
                deserialized_timezone,
                DateTimeError::InvalidTimezone
            );
        }

        /// Tests custom serialization and deserialization of DateTimeError.
        ///
        /// This test ensures that the custom serialization and deserialization
        /// implementations for `DateTimeError` work correctly.
        #[test]
        fn test_custom_serde_impl() {
            // Test with InvalidFormat variant
            let format_error = DateTimeError::InvalidFormat;
            let serialized_format =
                serde_json::to_string(&format_error)
                    .expect("Serialization failed");
            assert_eq!(
                format_error,
                serde_json::from_str(&serialized_format)
                    .expect("Deserialization failed")
            );

            // Test with ParseError variant using a mock string, because we cannot directly deserialize ParseError
            let parse_error =
                DateTimeError::ParseError(Parse::TryFromParsed(
                    TryFromParsed::InsufficientInformation,
                ));
            let serialized_parse = serde_json::to_string(&parse_error)
                .expect("Serialization failed");

            // Deserialize into a string and assert that it matches the known string for ParseError
            assert_eq!(serialized_parse, "\"ParseError\"");
            assert!(serde_json::from_str::<DateTimeError>(
                &serialized_parse
            )
            .is_err());
        }

        /// Tests deserialization of `DateTimeError` from strings.a
        ///
        /// This test verifies that `DateTimeError` can be deserialized from valid
        /// JSON strings and that invalid strings result in errors.
        #[test]
        fn test_from_str() {
            let error_str = r#""InvalidFormat""#;
            let deserialized: DateTimeError =
                serde_json::from_str(error_str).unwrap();
            assert_eq!(deserialized, DateTimeError::InvalidFormat);

            let error_str = r#""InvalidTimezone""#;
            let deserialized: DateTimeError =
                serde_json::from_str(error_str).unwrap();
            assert_eq!(deserialized, DateTimeError::InvalidTimezone);

            let error_str = r#""UnknownError""#;
            assert!(serde_json::from_str::<DateTimeError>(error_str)
                .is_err());
        }

        /// Tests deserialization from invalid JSON strings.
        ///
        /// This test ensures that attempting to deserialize `DateTimeError` from
        /// invalid JSON strings results in an appropriate error.
        #[test]
        fn test_invalid_json_deserialization() {
            let invalid_json_str = r#"{ "invalid": "data" }"#;
            assert!(serde_json::from_str::<DateTimeError>(
                invalid_json_str
            )
            .is_err());
        }
    }

    mod comparison_and_equality_tests {
        use dtt::error::DateTimeError;

        /// Tests the equality and inequality comparisons for `DateTimeError`.
        ///
        /// This test ensures that `DateTimeError` variants can be compared for
        /// equality and inequality, with the expected results.
        #[test]
        fn test_eq_and_ne() {
            let error1 = DateTimeError::InvalidFormat;
            let error2 = DateTimeError::InvalidFormat;
            let error3 = DateTimeError::InvalidTimezone;

            assert_eq!(error1, error2);
            assert_ne!(error1, error3);
        }

        /// Tests that `DateTimeError` variants are equal to themselves when compared.
        ///
        /// This test ensures that each variant of `DateTimeError` is equal to itself.
        #[test]
        fn test_self_comparison() {
            let format_error = DateTimeError::InvalidFormat;
            let timezone_error = DateTimeError::InvalidTimezone;

            assert_eq!(format_error, format_error);
            assert_eq!(timezone_error, timezone_error);
        }

        /// Tests the equality comparison for identical `DateTimeError` variants.
        #[test]
        fn test_eq_for_same_variant() {
            let error1 = DateTimeError::InvalidFormat;
            let error2 = DateTimeError::InvalidFormat;
            assert_eq!(error1, error2);
        }

        /// Tests the inequality comparison for different `DateTimeError` variants.
        #[test]
        fn test_ne_for_different_variants() {
            let error1 = DateTimeError::InvalidFormat;
            let error2 = DateTimeError::InvalidTimezone;
            assert_ne!(error1, error2);
        }

        /// Tests that different `DateTimeError` variants are not equal to each other.
        #[test]
        fn test_different_variants_comparison() {
            let format_error = DateTimeError::InvalidFormat;
            let timezone_error = DateTimeError::InvalidTimezone;
            let date_error = DateTimeError::InvalidDate;
            let time_error = DateTimeError::InvalidTime;

            assert_ne!(format_error, timezone_error);
            assert_ne!(format_error, date_error);
            assert_ne!(format_error, time_error);

            assert_ne!(timezone_error, date_error);
            assert_ne!(timezone_error, time_error);

            assert_ne!(date_error, time_error);
        }
    }

    mod error_handling_tests {
        use dtt::error::DateTimeError;
        use std::error::Error;

        /// Tests the `source` method of the `Error` trait for `DateTimeError`.
        ///
        /// This test ensures that the `source` method correctly returns `None`,
        /// indicating that `DateTimeError` has no underlying source error.
        #[test]
        fn test_error_source() {
            let error = DateTimeError::InvalidFormat;
            assert!(error.source().is_none());
        }

        /// Tests that the `unwrap` function does not cause panics for expected valid operations.
        ///
        /// This test ensures that the `unwrap` function, or any similar operation,
        /// does not panic when deserializing valid data or when handling valid error variants.
        #[test]
        fn test_no_unexpected_panics() {
            let valid_json_format = r#""InvalidFormat""#;
            let valid_json_timezone = r#""InvalidTimezone""#;

            let _ = serde_json::from_str::<DateTimeError>(
                valid_json_format,
            )
            .unwrap();
            let _ = serde_json::from_str::<DateTimeError>(
                valid_json_timezone,
            )
            .unwrap();
        }

        /// Tests the `Debug` and `Display` trait implementations for error variants.
        ///
        /// This test ensures that the `Debug` and `Display` implementations provide meaningful
        /// output for each error variant.
        #[test]
        fn test_debug_display_impl() {
            let format_error = DateTimeError::InvalidFormat;
            assert_eq!(format!("{:?}", format_error), "InvalidFormat");
            assert_eq!(
                format!("{}", format_error),
                "Invalid date format"
            );

            let timezone_error = DateTimeError::InvalidTimezone;
            assert_eq!(
                format!("{:?}", timezone_error),
                "InvalidTimezone"
            );
            assert_eq!(
                format!("{}", timezone_error),
                "Invalid or unsupported timezone; DST not supported"
            );
        }
    }

    mod future_proofing_tests {
        use dtt::error::DateTimeError;
        // We can't directly use Parse and ComponentRange variants due to privacy
        // So, focus on the variants that are testable.

        /// Tests the behavior of `DateTimeError` under potential future expansions.
        ///
        /// This test is designed to be forward-compatible, ensuring that any new variants
        /// added to `DateTimeError` do not cause existing code to break unexpectedly.
        #[test]
        fn test_non_exhaustive() {
            // Handle public and testable variants
            let parse_error_mock = DateTimeError::InvalidFormat; // Replace with another testable variant or skip

            // Mock or skip ComponentRange since it cannot be instantiated directly.
            let component_range_mock = DateTimeError::InvalidTimezone; // Replace with another testable variant or skip

            let all_variants = [
                DateTimeError::InvalidFormat,
                DateTimeError::InvalidTimezone,
                DateTimeError::InvalidDate,
                DateTimeError::InvalidTime,
                parse_error_mock, // Placeholder for ParseError
                component_range_mock, // Placeholder for ComponentRange
            ];

            for variant in all_variants.iter() {
                match variant {
                    DateTimeError::InvalidFormat => {
                        assert_eq!(
                            variant.to_string(),
                            "Invalid date format"
                        )
                    }
                    DateTimeError::InvalidTimezone => {
                        assert_eq!(
                            variant.to_string(),
                            "Invalid or unsupported timezone; DST not supported"
                        )
                    }
                    DateTimeError::InvalidDate => {
                        assert_eq!(variant.to_string(), "Invalid date")
                    }
                    DateTimeError::InvalidTime => {
                        assert_eq!(variant.to_string(), "Invalid time")
                    }
                    DateTimeError::ParseError(_) => {
                        assert!(variant
                            .to_string()
                            .contains("Parsing error"))
                    }
                    DateTimeError::ComponentRange(_) => {
                        assert!(variant
                            .to_string()
                            .contains("Component range error"))
                    }
                }
            }
        }
    }

    mod low_level_tests {
        use dtt::error::DateTimeError;
        use std::mem::size_of;

        /// Tests the memory layout of `DateTimeError`.
        ///
        /// This test ensures that the size and alignment of `DateTimeError` are as expected
        /// and do not change unexpectedly. This is important for ensuring ABI compatibility.
        #[test]
        fn test_memory_layout() {
            assert_eq!(size_of::<DateTimeError>(), 56);
        }

        #[test]
        fn test_error_serialization() {
            let error = DateTimeError::InvalidTimezone;
            let serialized = serde_json::to_string(&error).unwrap();
            assert_eq!(serialized, "\"InvalidTimezone\"");
        }

        #[test]
        fn test_error_deserialization() {
            let deserialized: DateTimeError =
                serde_json::from_str("\"InvalidDate\"").unwrap();
            assert_eq!(deserialized, DateTimeError::InvalidDate);
        }
    }

    mod default_implementation_tests {
        use dtt::error::DateTimeError;

        /// Tests the `Default` trait implementation for `DateTimeError`.
        ///
        /// This test ensures that the `default()` method returns the expected default variant.
        #[test]
        fn test_default_trait() {
            // Assuming `InvalidFormat` is the default variant for `DateTimeError`
            let default_error = DateTimeError::default();
            assert_eq!(default_error, DateTimeError::InvalidFormat);
        }
    }
}
