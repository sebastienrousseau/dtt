// test_main.rs
//
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

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
    /// A `Command` output containing the results of the executed binary.
    fn run_dtt_with_test_mode(test_mode: bool) -> std::process::Output {
        let mut cmd = Command::cargo_bin("dtt").unwrap();
        if test_mode {
            let _ = cmd.env("DTT_TEST_MODE", "1");
        }
        cmd.output().expect("Failed to execute command")
    }

    // Helper function to run tests with the DTT test mode and verify the result
    fn run_and_verify_test_mode(is_test_mode: bool, should_fail: bool) {
        let output = run_dtt_with_test_mode(is_test_mode);

        // Assert that the command execution matches the expected result
        assert_eq!(output.status.success(), !should_fail);

        // If the test is expected to fail, verify the error message
        if should_fail {
            let stderr = String::from_utf8(output.stderr).unwrap();
            assert!(
                stderr.contains("Error running dtt: Simulated error")
            );
        }
    }

    #[test]
    fn test_run_with_dtt_test_mode() {
        run_and_verify_test_mode(true, true);
    }

    #[test]
    fn test_run_without_dtt_test_mode() {
        run_and_verify_test_mode(false, false);
    }

    #[test]
    fn test_main() {
        // Test calling the `run_dtt_with_test_mode()` function directly with test mode enabled
        run_and_verify_test_mode(true, true);
    }
}
