// test_lib.rs
//
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

/// Unit tests for the `DateTime (DTT)` library.
///
/// This module contains a comprehensive set of unit tests for the library's public interface.
/// These tests ensure that the main entry points and key functionalities of the library work as expected.

#[cfg(test)]
mod tests {
    use dtt::run;

    /// Tests the main `run` function of the library.
    ///
    /// This test ensures that the `run` function executes correctly when not in test mode.
    #[test]
    fn test_run_success() {
        std::env::set_var("DTT_TEST_MODE", "0");
        let result = run();
        assert!(result.is_ok());
    }

    /// Tests the main `run` function of the library in test mode.
    ///
    /// This test ensures that the `run` function returns an error when in test mode.
    #[test]
    fn test_run_test_mode_error() {
        std::env::set_var("DTT_TEST_MODE", "1");
        let result = run();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Simulated error");
    }

    /// Tests the `datetime` module's functionality.
    ///
    /// This test ensures that the `datetime` module is accessible and that key methods work as expected.
    #[test]
    fn test_datetime_module() {
        use dtt::datetime::DateTime;
        let dt = DateTime::new();
        assert!(dt.to_string().contains("T"));
    }

    /// Tests the `error` module's functionality.
    ///
    /// This test ensures that the custom error types in the `error` module work as expected.
    #[test]
    fn test_error_module() {
        use dtt::error::DateTimeError;
        let error = DateTimeError::InvalidFormat;
        assert_eq!(error.to_string(), "Invalid date format");
    }

    // /// Tests the `macros` module's functionality.
    // ///
    // /// This test ensures that macros in the `macros` module can be used as expected.
    // #[test]
    // fn test_macros_module() {
    //     use dtt::macros::date_macro;
    //     let date = date_macro!("2023-01-01");
    //     assert_eq!(date, "2023-01-01");
    // }

    /// Tests the environment variable behavior in `run`.
    ///
    /// This test ensures that the environment variable `DTT_TEST_MODE` is correctly read and used by the `run` function.
    #[test]
    fn test_env_var_handling() {
        std::env::set_var("DTT_TEST_MODE", "0");
        let result = run();
        assert!(result.is_ok());
    }

    /// Tests that the library's metadata is correct.
    ///
    /// This test checks that the library's crate name, type, and documentation URLs are correctly set.
    #[test]
    fn test_library_metadata() {
        let crate_name = env!("CARGO_PKG_NAME");
        assert_eq!(crate_name, "dtt");

        let version = env!("CARGO_PKG_VERSION");
        assert_eq!(version, "0.0.9");

        let homepage = env!("CARGO_PKG_HOMEPAGE");
        assert_eq!(homepage, "https://dttlib.com/");
    }

    /// Tests the library's license information.
    ///
    /// This test checks that the license information for the library is correct.
    #[test]
    fn test_license_information() {
        let license = env!("CARGO_PKG_LICENSE");
        assert_eq!(license, "MIT OR Apache-2.0");
    }
}
