// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! DateTime (DTT) is a comprehensive library for date and time manipulation.
//!
//! # Overview
//!
//! This crate provides robust tools for handling dates, times, and timezones in Rust,
//! with a focus on correctness, ergonomics, and performance.
//!
//! # Features
//!
//! - Create and parse dates in multiple formats
//! - Timezone conversions and handling
//! - Date and time arithmetic
//! - Formatting and serialization
//!
//! # Error Handling
//!
//! This crate uses the `thiserror` crate for error handling. All errors are
//! properly typed and implement standard error traits. Operations that may fail
//! return `Result<T, AppError>`.
//!
//! # Examples
//!
//! ```rust
//! use dtt::DateTime;
//!
//! let now = DateTime::new();
//! println!("Current time: {}", now);
//! ```

#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/dtt/images/favicon.ico",
    html_logo_url = "https://kura.pro/dtt/images/logos/dtt.svg",
    html_root_url = "https://docs.rs/dtt"
)]
// Linting configuration
#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    rustdoc::broken_intra_doc_links,
    missing_docs,
    unsafe_code
)]
#![allow(clippy::module_name_repetitions)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Standard library imports
use std::env;

/// Library constants and configuration values
pub mod constants {
    /// Current version of the library from Cargo.toml
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    /// Environment variable controlling test mode
    pub const TEST_MODE_ENV: &str = "DTT_TEST_MODE";

    /// Value indicating test mode is enabled
    pub const TEST_MODE_ENABLED: &str = "1";

    /// Welcome message displayed during initialization
    pub const WELCOME_MSG: &str = "Welcome to `DTT` 👋!";

    /// Library description displayed during initialization
    pub const DESCRIPTION: &str = 
        "A Rust library for parsing, validating, manipulating, and formatting dates and times.";
}

// Re-exports with inline documentation
#[doc(inline)]
pub use crate::datetime::DateTime;
#[doc(inline)]
pub use crate::error::AppError;

/// Core datetime functionality and operations.
///
/// This module contains the primary `DateTime` type and associated functionality
/// for date and time manipulation.
pub mod datetime;

/// Error handling types and implementations.
///
/// Provides custom error types for handling various error conditions that may
/// occur during datetime operations.
pub mod error;

/// Macro definitions for common operations.
///
/// Contains utility macros to simplify common datetime operations and reduce
/// boilerplate code.
pub mod macros;

/// Commonly used types and traits.
///
/// Provides a convenient way to import commonly used types with a single use statement.
pub mod prelude {
    pub use crate::datetime::DateTime;
    pub use crate::error::{AppError, DateTimeError};
}

/// Runs the main library functionality with proper error handling.
///
/// This function initializes the library and performs basic setup operations.
/// It checks for test mode and returns appropriate results based on the
/// environment configuration.
///
/// # Errors
///
/// Returns `AppError::SimulatedError` in the following cases:
/// - When the `DTT_TEST_MODE` environment variable is set to "1"
/// - When environment variable access fails
///
/// # Examples
///
/// ```rust
/// use dtt::prelude::*;
///
/// fn main() -> Result<(), AppError> {
///     match dtt::run() {
///         Ok(()) => println!("Library initialized successfully"),
///         Err(e) => eprintln!("Error during initialization: {}", e),
///     }
///     Ok(())
/// }
/// ```
pub fn run() -> Result<(), AppError> {
    if is_test_mode() {
        return Err(AppError::SimulatedError);
    }

    display_welcome_message();
    Ok(())
}

/// Checks if the library is running in test mode.
///
/// Examines the environment variable `DTT_TEST_MODE` to determine if the library
/// should operate in test mode.
fn is_test_mode() -> bool {
    env::var(constants::TEST_MODE_ENV)
        .map(|val| val == constants::TEST_MODE_ENABLED)
        .unwrap_or(false)
}

/// Displays the welcome message with library information.
///
/// Prints a welcome message along with the library description and current version.
fn display_welcome_message() {
    println!("{}", constants::WELCOME_MSG);
    println!("{}", constants::DESCRIPTION);
    println!("Version: {}", constants::VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    mod initialization {
        use super::*;

        #[test]
        fn test_normal_run() {
            env::remove_var(constants::TEST_MODE_ENV);
            assert!(run().is_ok());
        }

        #[test]
        fn test_simulated_error() {
            env::set_var(
                constants::TEST_MODE_ENV,
                constants::TEST_MODE_ENABLED,
            );
            assert!(matches!(run(), Err(AppError::SimulatedError)));
        }
    }

    mod configuration {
        use super::*;

        #[test]
        #[allow(clippy::const_is_empty)]
        fn test_version_constant() {
            assert!(
                !constants::VERSION.is_empty(),
                "Version string should not be empty"
            );
        }

        #[test]
        fn test_is_test_mode() {
            env::remove_var(constants::TEST_MODE_ENV);
            let first_check = is_test_mode();
            assert!(
                !first_check,
                "Should not be in test mode by default"
            );

            env::set_var(
                constants::TEST_MODE_ENV,
                constants::TEST_MODE_ENABLED,
            );
            let second_check = is_test_mode();
            assert!(
                second_check,
                "Should be in test mode after enabling it"
            );
        }
    }
}
