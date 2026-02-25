// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

use dtt::DateTime;

fn main() {
    println!("=== DTT Error Diagnostics Demo ===");

    // Attempting to construct a DateTime with an invalid month (13).
    // dtt natively maps to `miette::Diagnostic` for beautiful error output.
    let res = DateTime::from_components(
        2024,
        13, // Invalid month
        1,
        12,
        0,
        0,
        time::UtcOffset::UTC,
    );

    match res {
        Ok(_) => println!("Successfully constructed DateTime - this shouldn't happen!"),
        Err(e) => {
            println!("Intercepted Error: {}", e);
            println!("Since `miette` is enabled, this can be beautifully rendered in terminals handling `miette::Result`.");
        }
    }
}
