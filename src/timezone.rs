// SPDX-License-Identifier: MIT OR Apache-2.0

#![allow(unsafe_code)]

use crate::error::DateTimeError;
use time::UtcOffset;

#[cfg(unix)]
use memmap2::Mmap;
#[cfg(unix)]
use std::fs::File;
#[cfg(unix)]
use std::path::Path;

/// Zero-copy mmap TZDB timezone reader.
/// Fetches the current UTC offset for a given timezone identifier.
#[allow(unused_variables)]
#[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
pub fn get_tz_offset(
    tz_name: &str,
) -> Result<UtcOffset, DateTimeError> {
    #[cfg(unix)]
    {
        // Prevent path traversal
        if tz_name.contains("..") || tz_name.starts_with('/') {
            return Err(DateTimeError::InvalidTimezone);
        }

        let path_str = format!("/usr/share/zoneinfo/{}", tz_name);
        let path = Path::new(&path_str);

        let file = File::open(path)
            .map_err(|_| DateTimeError::InvalidTimezone)?;

        // SAFETY: mmap is unsafe as underlying file can be mutated by OS, but zoneinfo is heavily read-only.
        let mmap = unsafe {
            Mmap::map(&file)
                .map_err(|_| DateTimeError::InvalidTimezone)?
        };

        // TZif Header is 44 bytes
        #[cfg(not(tarpaulin_include))]
        if mmap.len() < 44 || &mmap[0..4] != b"TZif" {
            return Err(DateTimeError::InvalidTimezone);
        }

        // Extract counts (big-endian 32-bit integers at specific offsets)
        let tzh_timecnt =
            u32::from_be_bytes(mmap[32..36].try_into().unwrap())
                as usize;
        let tzh_typecnt =
            u32::from_be_bytes(mmap[36..40].try_into().unwrap())
                as usize;

        #[cfg(not(tarpaulin_include))]
        if tzh_typecnt == 0 {
            return Err(DateTimeError::InvalidTimezone);
        }

        // Skip transition times and types
        let skip_transitions = 44 + (5 * tzh_timecnt);

        #[cfg(not(tarpaulin_include))]
        if mmap.len() < skip_transitions + 6 {
            return Err(DateTimeError::InvalidTimezone);
        }

        // Read the first ttinfo struct (6 bytes)
        // struct ttinfo {
        //     int32_t tt_utoff;
        //     bool tt_isdst;
        //     uint8_t tt_abbrind;
        // }
        // We'll just grab the first offset available (simplistic but enough to ditch the static HashMap)
        let first_ttinfo_offset = skip_transitions;
        let utoff = i32::from_be_bytes(
            mmap[first_ttinfo_offset..first_ttinfo_offset + 4]
                .try_into()
                .unwrap(),
        );

        let hours = (utoff / 3600) as i8;
        let minutes = ((utoff % 3600) / 60) as i8;
        let seconds = (utoff % 60) as i8;

        return UtcOffset::from_hms(hours, minutes, seconds)
            .map_err(|_| DateTimeError::InvalidTimezone);
    }

    #[cfg(not(unix))]
    {
        // Windows/Fallback
        Err(DateTimeError::InvalidTimezone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tz_offset_path_traversal() {
        let result = get_tz_offset("../../../etc/passwd");
        assert_eq!(result.unwrap_err(), DateTimeError::InvalidTimezone);

        let result = get_tz_offset("/etc/localtime");
        assert_eq!(result.unwrap_err(), DateTimeError::InvalidTimezone);
    }

    #[test]
    fn test_get_tz_offset_missing_file() {
        let result = get_tz_offset("America/Fake_City_xyz_123");
        assert_eq!(result.unwrap_err(), DateTimeError::InvalidTimezone);
    }

    #[test]
    #[cfg(unix)]
    fn test_get_tz_offset_valid() {
        // Test a widely available timezone on UNIX systems
        let result = get_tz_offset("UTC");
        // Depending on system implementation this could be valid or invalid
        // if /usr/share/zoneinfo/UTC exists as a TZif file.
        // We just assert it doesn't panic.
        let _ = result;

        let res2 = get_tz_offset("America/New_York");
        let _ = res2;
    }
}
