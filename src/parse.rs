// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::error::DateTimeError;
#[cfg(nightly)]
use std::simd::prelude::*;
use time::{Date, Month, PrimitiveDateTime, Time};

/// Parses an ISO-8601/RFC-3339 datetime byte slice using SIMD vector operations.
///
/// Expected format: `YYYY-MM-DDTHH:MM:SS` (or `YYYY-MM-DD HH:MM:SS`)
/// Returns a `time::PrimitiveDateTime` or `DateTimeError`.
///
/// This is a zero-allocation parser that uses 256-bit vector bounds checking.
///
/// # Errors
/// Returns `DateTimeError::InvalidFormat` if the input cannot be safely parsed.
#[cfg_attr(
    feature = "tracing",
    tracing::instrument(level = "trace", skip_all)
)]
pub fn parse_datetime(
    bytes: &[u8],
) -> Result<PrimitiveDateTime, DateTimeError> {
    #[cfg(nightly)]
    {
        let len = bytes.len();
        if !(10..=32).contains(&len) {
            return fallback_parse(bytes);
        }

        let mut chunk = [b'0'; 32];
        chunk[..len].copy_from_slice(bytes);
        let v = u8x32::from_array(chunk);

        let zero = u8x32::splat(b'0');
        let nine = u8x32::splat(b'9');

        // Bounds check digits using SIMD mask
        let digits_mask = v.simd_ge(zero) & v.simd_le(nine);
        let mask_arr = digits_mask.to_array();

        if bytes[4] != b'-' || bytes[7] != b'-' {
            return fallback_parse(bytes);
        }

        let valid_date_digits = mask_arr[0]
            && mask_arr[1]
            && mask_arr[2]
            && mask_arr[3]
            && mask_arr[5]
            && mask_arr[6]
            && mask_arr[8]
            && mask_arr[9];

        if !valid_date_digits {
            return fallback_parse(bytes);
        }

        let year = ((bytes[0] - b'0') as i32) * 1000
            + ((bytes[1] - b'0') as i32) * 100
            + ((bytes[2] - b'0') as i32) * 10
            + ((bytes[3] - b'0') as i32);
        let month = (bytes[5] - b'0') * 10 + (bytes[6] - b'0');
        let day = (bytes[8] - b'0') * 10 + (bytes[9] - b'0');

        let month_enum = Month::try_from(month)
            .map_err(|_| DateTimeError::InvalidFormat)?;
        let date = Date::from_calendar_date(year, month_enum, day)
            .map_err(|_| DateTimeError::InvalidDate)?;

        if len == 10 {
            return Ok(PrimitiveDateTime::new(date, Time::MIDNIGHT));
        }

        if len >= 19 {
            if (bytes[10] != b'T' && bytes[10] != b' ')
                || bytes[13] != b':'
                || bytes[16] != b':'
            {
                return fallback_parse(bytes);
            }

            let valid_time_digits = mask_arr[11]
                && mask_arr[12]
                && mask_arr[14]
                && mask_arr[15]
                && mask_arr[17]
                && mask_arr[18];

            if !valid_time_digits {
                return fallback_parse(bytes);
            }

            let hour = (bytes[11] - b'0') * 10 + (bytes[12] - b'0');
            let minute = (bytes[14] - b'0') * 10 + (bytes[15] - b'0');
            let second = (bytes[17] - b'0') * 10 + (bytes[18] - b'0');

            if len > 19 {
                if bytes[19] == b'Z' && len == 20 {
                    // Just 'Z', we can safely ignore since PrimitiveDateTime doesn't store TZ.
                } else {
                    // Fractional seconds, arbitrary TZ offsets, or invalid characters: fallback
                    return fallback_parse(bytes);
                }
            }

            let time = Time::from_hms(hour, minute, second)
                .map_err(|_| DateTimeError::InvalidTime)?;

            return Ok(PrimitiveDateTime::new(date, time));
        }
    }

    fallback_parse(bytes)
}

fn fallback_parse(
    bytes: &[u8],
) -> Result<PrimitiveDateTime, DateTimeError> {
    let s = std::str::from_utf8(bytes)
        .map_err(|_| DateTimeError::InvalidFormat)?;
    if let Ok(dt) = PrimitiveDateTime::parse(
        s,
        &time::format_description::well_known::Rfc3339,
    ) {
        return Ok(dt);
    }
    if let Ok(d) = Date::parse(
        s,
        &time::format_description::well_known::Iso8601::DATE,
    ) {
        return Ok(PrimitiveDateTime::new(d, Time::MIDNIGHT));
    }
    Err(DateTimeError::InvalidFormat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_fallback_parse_date() {
        // Trigger line 122 by feeding a raw date directly to fallback
        let res = fallback_parse(b"2026-02-24");
        assert!(res.is_ok());
    }

    #[test]
    fn test_simd_parse_branches() {
        // Fallback length < 10
        assert!(parse_datetime(b"123").is_err());

        // Non-digit year triggering fallback
        let res = parse_datetime(b"ABCD-01-01T12:00:00Z");
        assert!(res.is_err());

        // Exact 10 byte valid date
        let d10 = parse_datetime(b"2024-05-15").unwrap();
        assert_eq!(d10.year(), 2024);

        // Invalid month Enum mapping (e.g. 13)
        let res_month = parse_datetime(b"2024-13-01T12:00:00Z");
        assert!(res_month.is_err());

        // Invalid Time Digits triggering fallback
        let res_time_digits = parse_datetime(b"2024-05-01TXX:00:00");
        assert!(res_time_digits.is_err());

        // Fractional seconds fallback to standard parser
        let res_frac = parse_datetime(b"2024-05-01T12:00:00.123Z");
        assert!(res_frac.is_ok()); // The fallback parser will handle it successfully

        // Fallback for > 20 length but not fractional Z
        let res_long = parse_datetime(b"2024-05-01T12:00:00+05:30");
        assert!(res_long.is_ok());

        // Length > 19 but 'Z'
        let res_z = parse_datetime(b"2024-05-01T12:00:00Z");
        assert!(res_z.is_ok());
    }
}
