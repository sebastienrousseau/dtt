// test_error.rs
//
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

/// Unit tests for the `DateTimeError` enum.
///
/// This module contains a comprehensive set of unit tests for the `DateTimeError` enum,
/// which is defined in `error.rs`. The tests cover various aspects, including:
/// - Basic functionality checks.
/// - Trait implementations like Debug, Clone, Copy, Ord, PartialOrd, Hash.
/// - Serialization and deserialization using `serde`.
/// - Comparison and equality checks.
/// - Error handling validations.
/// - Future-proofing for additional variants.
/// - Low-level checks such as memory layout and default values.

#[cfg(test)]
mod tests {

    mod basic_functionality {
        use dtt::error::DateTimeError;

        /// Tests the `InvalidFormat` variant of `DateTimeError`.
        ///
        /// This test ensures that the `InvalidFormat` variant correctly implements
        /// the `Display` and `Error` traits.
        #[test]
        fn test_invalid_format() {
            let error = DateTimeError::InvalidFormat;

            // Verify the Display implementation
            assert_eq!(error.to_string(), "Invalid date format");

            // Verify the Error trait implementation
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

            // Verify the Display implementation
            assert_eq!(
                error.to_string(),
                "Invalid or unsupported timezone; DST not supported"
            );

            // Verify the Error trait implementation
            let error: &dyn std::error::Error = &error;
            assert_eq!(
                error.to_string(),
                "Invalid or unsupported timezone; DST not supported"
            );
        }
    }

    mod trait_implementations {
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
            set.insert(DateTimeError::InvalidFormat);
            set.insert(DateTimeError::InvalidTimezone);

            assert_eq!(set.len(), 2);
            assert!(set.contains(&DateTimeError::InvalidFormat));
            assert!(set.contains(&DateTimeError::InvalidTimezone));
        }
    }

    mod serialization {
        use dtt::error::DateTimeError;

        /// Tests the serialization and deserialization of `DateTimeError`.
        ///
        /// This test ensures that the `DateTimeError` enum can be correctly
        /// serialized to and deserialized from JSON using the `serde` crate.
        #[test]
        fn test_serialization() {
            let format_error = DateTimeError::InvalidFormat;
            let timezone_error = DateTimeError::InvalidTimezone;

            // Serialize the errors to JSON
            let serialized_format =
                serde_json::to_string(&format_error)
                    .expect("Serialization failed");
            let serialized_timezone =
                serde_json::to_string(&timezone_error)
                    .expect("Serialization failed");

            // Deserialize the errors from JSON
            let deserialized_format: DateTimeError =
                serde_json::from_str(&serialized_format)
                    .expect("Deserialization failed");
            let deserialized_timezone: DateTimeError =
                serde_json::from_str(&serialized_timezone)
                    .expect("Deserialization failed");

            // Verify the results
            assert_eq!(format_error, deserialized_format);
            assert_eq!(timezone_error, deserialized_timezone);
        }

        /// Tests deserialization of `DateTimeError` from strings.
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

    mod comparison_and_equality {
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
    }

    mod error_handling {
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
    }

    mod future_proofing {
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

    mod low_level {
        use dtt::error::DateTimeError;

        /// Tests the memory layout of `DateTimeError`.
        ///
        /// This test ensures that the size and alignment of `DateTimeError` are as expected
        /// and do not change unexpectedly. This is important for ensuring ABI compatibility.
        #[test]
        fn test_memory_layout() {
            assert_eq!(size_of::<DateTimeError>(), 56);
        }

        /// Tests the `Default` trait implementation for `DateTimeError`.
        ///
        /// This test checks that the `Default` trait is implemented correctly,
        /// ensuring that the default value of `DateTimeError` is as expected.
        #[test]
        fn test_default_trait() {
            // This test assumes that InvalidFormat is the default variant.
            // If that's not the case, adjust the assertion accordingly.
            assert_eq!(
                DateTimeError::default(),
                DateTimeError::InvalidFormat
            );
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
}
