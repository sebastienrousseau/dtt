// lib.rs
//
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://kura.pro/dtt/images/favicon.ico",
    html_logo_url = "https://kura.pro/dtt/images/logos/dtt.svg",
    html_root_url = "https://docs.rs/dtt"
)]
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

// Re-exports for convenient access
pub use crate::datetime::DateTime;
pub use crate::error::AppError;

/// Core datetime functionality module
pub mod datetime;

/// Error handling types and implementations
pub mod error;

/// Macro definitions and implementations
pub mod macros;

/// Commonly used items exported for convenience
pub mod prelude {
    pub use crate::datetime::DateTime;
    pub use crate::error::{AppError, DateTimeError};
}

use std::env;

/// Library version from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Configuration for test mode environment variable
const TEST_MODE_ENV: &str = "DTT_TEST_MODE";
const TEST_MODE_ENABLED: &str = "1";

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
/// # Errors
///
/// Returns an error if environment variable access fails.
fn is_test_mode() -> bool {
    env::var(TEST_MODE_ENV).unwrap_or_default() == TEST_MODE_ENABLED
}

/// Displays the welcome message with library information.
fn display_welcome_message() {
    println!("Welcome to `DTT` 👋!");
    println!(
        "A Rust library for parsing, validating, manipulating, and formatting dates and times."
    );
    println!("Version: {}", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_run() {
        env::remove_var(TEST_MODE_ENV);
        assert!(run().is_ok());
    }

    #[test]
    fn test_simulated_error() {
        env::set_var(TEST_MODE_ENV, TEST_MODE_ENABLED);
        assert!(matches!(run(), Err(AppError::SimulatedError)));
    }

    #[test]
#[allow(clippy::const_is_empty)]
fn test_version_constant() {
    // Just to demonstrate we have a version string from Cargo
    assert!(!VERSION.is_empty());
}


    #[test]
fn test_is_test_mode() {
    env::remove_var(TEST_MODE_ENV);

    // is_test_mode()? returns a bool or an error. The ? operator
    // propagates any error, meaning we won't panic or unwrap.
    let first_check = is_test_mode();
    assert!(!first_check, "Should not be in test mode by default");

    env::set_var(TEST_MODE_ENV, TEST_MODE_ENABLED);

    let second_check = is_test_mode();
    assert!(second_check, "Should be in test mode after enabling it");
}

}
