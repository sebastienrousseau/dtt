// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(unsafe_code)]

use time::{OffsetDateTime, UtcOffset};

/// Platform-native high-resolution time fetch.
///
/// Bypasses `std::time::SystemTime` to directly query `libc::clock_gettime(CLOCK_REALTIME)`
/// on UNIX-like platforms avoiding abstract overheads. On macOS, this takes advantage of VDSO natively.
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
pub fn native_now() -> OffsetDateTime {
    #[cfg(unix)]
    {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: clock_gettime directly writes to standard timespec struct safely
        unsafe {
            let _ = libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts);
        }
        let now_utc = OffsetDateTime::from_unix_timestamp_nanos(
            (ts.tv_sec as i128) * 1_000_000_000 + (ts.tv_nsec as i128),
        )
        .unwrap_or_else(|_| OffsetDateTime::now_utc());
        now_utc
    }
    #[cfg(target_arch = "wasm32")]
    {
        // Use JS native Date.now() which returns milliseconds since UNIX epoch
        let ms = js_sys::Date::now();
        let secs = (ms / 1000.0) as i128;
        let nanos = ((ms % 1000.0) * 1_000_000.0) as i128;
        OffsetDateTime::from_unix_timestamp_nanos(
            (secs * 1_000_000_000) + nanos,
        )
        .unwrap_or_else(|_| OffsetDateTime::now_utc())
    }
    #[cfg(all(not(unix), not(target_arch = "wasm32")))]
    {
        // Fallback for other non-UNIX, non-WASM targets
        OffsetDateTime::now_utc()
    }
}

/// Fetches the platform-native current time and applies a timezone offset.
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
pub fn native_now_with_offset(offset: UtcOffset) -> OffsetDateTime {
    native_now().to_offset(offset)
}
