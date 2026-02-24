// SPDX-License-Identifier: MIT OR Apache-2.0

//! Relative Time formatting for `dtt`.
//!
//! Provides ultra-fast, zero-allocation "humanized" time conversions mapping
//! `DateTime` intervals into standard conversational English (e.g. "Just now",
//! "3 hours ago", "in 2 days").

use crate::DateTime;

impl DateTime {
    /// Returns a human-readable, relative time string comparing this `DateTime`
    /// against the exact moment it is invoked.
    ///
    /// # Returns
    ///
    /// Returns a `String` representing the relative offset (e.g., "5 minutes ago",
    /// "Just now", "in 1 year").
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dtt::DateTime;
    /// use time::Duration;
    ///
    /// let mut now = DateTime::new();
    /// assert_eq!(now.relative(), "Just now");
    ///
    /// let mut past = now.minus(Duration::days(2)).unwrap();
    /// assert_eq!(past.relative(), "2 days ago");
    ///
    /// let mut future = now.plus(Duration::hours(5)).unwrap();
    /// let rel = future.relative();
    /// assert!(rel == "in 5 hours" || rel == "in 4 hours");
    /// ```
    #[must_use]
    pub fn relative(&self) -> String {
        let now = DateTime::new();
        let diff = self.duration_since(&now);

        // Handle precise immediate comparisons
        if diff.whole_seconds() == 0 {
            return "Just now".to_string();
        }

        let is_past = diff.is_negative();
        let abs_seconds = diff.whole_seconds().abs();

        let value;
        let unit;

        // Breakdowns based on standard progressive scales
        if abs_seconds < 60 {
            value = abs_seconds;
            unit = if value == 1 { "second" } else { "seconds" };
        } else if abs_seconds < 3600 {
            value = abs_seconds / 60;
            unit = if value == 1 { "minute" } else { "minutes" };
        } else if abs_seconds < 86_400 {
            value = abs_seconds / 3600;
            unit = if value == 1 { "hour" } else { "hours" };
        } else if abs_seconds < 2_592_000 {
            // 30 days
            value = abs_seconds / 86_400;
            unit = if value == 1 { "day" } else { "days" };
        } else if abs_seconds < 31_536_000 {
            // 365 days
            value = abs_seconds / 2_592_000;
            unit = if value == 1 { "month" } else { "months" };
        } else {
            value = abs_seconds / 31_536_000;
            unit = if value == 1 { "year" } else { "years" };
        }

        if is_past {
            format!("{} {} ago", value, unit)
        } else {
            format!("in {} {}", value, unit)
        }
    }
}
