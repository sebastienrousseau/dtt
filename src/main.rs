// Copyright © 2023 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This is the main entry point for the dtt application.
fn main() {
    // Call the `run()` function from the `DateTime (DTT)` module.
    if let Err(err) = dtt::run() {
        eprintln!("Error running dtt: {}", err);
        std::process::exit(1);
    }
}
