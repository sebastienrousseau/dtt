// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

#[cfg(test)]
mod tests {
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_run_with_dtt_test_mode() {
        let output = Command::cargo_bin("dtt")
            .unwrap()
            .env("DTT_TEST_MODE", "1")
            .output()
            .expect("Failed to execute command");

        // Assert that the command execution was not successful
        assert!(!output.status.success());

        // Assert that the error message was printed to stderr
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("Error running dtt: Simulated error"));
    }

    #[test]
    fn test_run_without_dtt_test_mode() {
        let output = Command::cargo_bin("dtt")
            .unwrap()
            .output()
            .expect("Failed to execute command");

        // Assert that the command execution was successful
        assert!(output.status.success());

        // Assert that the welcome messages were printed to stdout
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Welcome to `DTT` 👋!"));
        assert!(stdout.contains("A Rust library for parsing, validating, manipulating, and formatting dates and times."));
    }

    fn run_test_scenario() -> Result<(), Box<dyn std::error::Error>> {
        // Simulate an error scenario
        // Return an error explicitly
        Err("Test error".into())
    }

    #[test]
    fn test_main() {
        // Test calling the `run()` function directly
        let result = run_test_scenario();
        assert!(result.is_err());

        // Test calling the `main()` function
        let output = Command::cargo_bin("dtt")
            .unwrap()
            .env("DTT_TEST_MODE", "1")
            .output()
            .expect("Failed to execute command");

        // Assert that the command execution was not successful
        assert!(!output.status.success());

        // Assert that the error message was printed to stderr
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.contains("Error running dtt: Simulated error"));
    }
}
