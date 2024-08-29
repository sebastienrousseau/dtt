// error.rs
//
// Copyright © 2023-2024 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// See LICENSE-APACHE.md and LICENSE-MIT.md in the repository root for full license information.

use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

/// Custom error type for DateTime parsing.
#[derive(
    Copy,
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub enum DateTimeError {
    /// Invalid date format.
    InvalidFormat,
    /// Invalid timezone.
    InvalidTimezone,
}

impl fmt::Display for DateTimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DateTimeError::InvalidFormat => {
                write!(f, "Invalid date format")
            }
            DateTimeError::InvalidTimezone => {
                write!(f, "Invalid timezone")
            }
        }
    }
}

impl Error for DateTimeError {}
