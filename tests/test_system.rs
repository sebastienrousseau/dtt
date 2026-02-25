// SPDX-License-Identifier: MIT OR Apache-2.0

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
