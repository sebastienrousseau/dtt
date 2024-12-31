// test_main.rs
//
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # Integration Tests for the `dtt` Binary
//!
//! This file executes end-to-end integration tests against the `dtt` binary,
//! verifying command-line behaviour, environment variables, and expected output
//! messages when run with and without `DTT_TEST_MODE` set.

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    /// Helper function to run the `dtt` binary with an optional environment variable.
    ///
    /// # Arguments
    ///
    /// * `test_mode` - A boolean indicating whether to run with `DTT_TEST_MODE` enabled.
    ///
    /// # Returns
    ///
    /// A `Result<Output, Box<dyn std::error::Error>>` containing the results of the executed binary.
    fn run_dtt_with_test_mode(
        test_mode: bool,
    ) -> Result<std::process::Output, Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("dtt").map_err(|err| {
            format!("Failed to find binary 'dtt': {:?}", err)
        })?;
        if test_mode {
            let _ = cmd.env("DTT_TEST_MODE", "1");
        }
        let output = cmd.output().map_err(|err| {
            format!("Failed to execute command: {:?}", err)
        })?;
        Ok(output)
    }

    /// Helper function to run tests with the DTT test mode and verify the result.
    ///
    /// # Arguments
    ///
    /// * `is_test_mode` - A boolean indicating whether to run in test mode.
    /// * `should_fail` - A boolean indicating whether the command is expected to fail.
    ///
    /// # Returns
    ///
    /// A `Result<(), Box<dyn std::error::Error>>` indicating the test result.
    fn run_and_verify_test_mode(
        is_test_mode: bool,
        should_fail: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let output = run_dtt_with_test_mode(is_test_mode)?;

        // Assert that the command execution matches the expected result
        assert_eq!(output.status.success(), !should_fail);

        // If the test is expected to fail, verify the error message
        if should_fail {
            let stderr =
                String::from_utf8(output.stderr).map_err(|err| {
                    eprintln!(
                        "Failed to parse stderr as UTF-8: {:?}",
                        err
                    );
                    err
                })?;
            assert!(
                stderr.contains("Error running dtt: Simulated error")
            );
        }

        Ok(())
    }

    #[test]
    fn test_run_with_dtt_test_mode(
    ) -> Result<(), Box<dyn std::error::Error>> {
        run_and_verify_test_mode(true, true)?;
        Ok(())
    }

    #[test]
    fn test_run_without_dtt_test_mode(
    ) -> Result<(), Box<dyn std::error::Error>> {
        run_and_verify_test_mode(false, false)?;
        Ok(())
    }

    #[test]
    fn test_main() -> Result<(), Box<dyn std::error::Error>> {
        // Test calling the `run_dtt_with_test_mode()` function directly with test mode enabled
        run_and_verify_test_mode(true, true)?;
        Ok(())
    }
}
