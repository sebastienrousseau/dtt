// main.rs
//
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

//! This is the main entry point for the dtt application.
fn main() {
    // Call the `run()` function from the `DateTime (DTT)` module.
    if let Err(err) = dtt::run() {
        eprintln!("Error running dtt: {}", err);
        std::process::exit(1);
    }
}
