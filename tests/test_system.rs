// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(clippy::all, clippy::pedantic, clippy::nursery, clippy::unwrap_used, clippy::expect_used)]
#![allow(missing_docs, unused_must_use, unused_results, unused_variables, dead_code)]

use dtt::system;
use time::UtcOffset;

#[test]
fn test_native_now_with_offset() {
    // Generate a fixed +2 hours offset
    let offset = UtcOffset::from_hms(2, 0, 0).unwrap();

    // Call the specific system hook
    let dt = system::native_now_with_offset(offset);

    // It should successfully apply the 2 hour positive offset to the native UTC measurement
    assert_eq!(dt.offset(), offset);
}
