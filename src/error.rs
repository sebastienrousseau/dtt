// error.rs
//
// Copyright © 2025 DateTime (DTT) library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json;
use std::{
    env,
    hash::{Hash, Hasher},
};
use thiserror::Error;
use time::error::{ComponentRange, Parse};

/// Custom error type for the application.
///
/// This error type encapsulates all possible errors that might occur in the application,
/// including simulated errors for testing and environment variable retrieval errors.
#[derive(Error, Debug)]
pub enum AppError {
    /// Error that occurs during datetime operations.
    #[error("DateTime operation error: {0}")]
    DateTimeError(#[from] DateTimeError),

    /// Error that occurs during serialization.
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// General I/O or parsing error.
    #[error("General I/O or parsing error: {0}")]
    GeneralError(#[from] std::io::Error),

    /// Error that occurs during other operations.
    #[error("Other error: {0}")]
    Other(String),

    /// Error for simulating a failure in test mode.
    #[error("Simulated error")]
    SimulatedError,

    /// Error that occurs when retrieving environment variables.
    #[error("Environment variable error: {0}")]
    EnvVarError(#[from] env::VarError),
}

/// Custom error type for the `DateTime` library.
///
/// This enum represents various errors that can occur when working with
/// `DateTime` objects, such as invalid formats, timezones, and component ranges.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Error)]
pub enum DateTimeError {
    /// The provided date format is invalid.
    #[error("Invalid date format")]
    InvalidFormat,

    /// The provided timezone is invalid or not supported. DST is not supported.
    #[error("Invalid or unsupported timezone; DST not supported")]
    InvalidTimezone,

    /// The date is invalid (e.g., February 30).
    #[error("Invalid date")]
    InvalidDate,

    /// The time is invalid (e.g., 25:00).
    #[error("Invalid time")]
    InvalidTime,

    /// An error occurred while parsing the date/time string.
    #[error("Parsing error")]
    ParseError(#[from] Parse),

    /// A component (year, month, day, etc.) is out of the valid range.
    #[error("Component range error")]
    ComponentRange(#[from] ComponentRange),
}

impl Hash for DateTimeError {
    /// Custom implementation of the `Hash` trait for `DateTimeError`.
    ///
    /// This allows `DateTimeError` to be used in hashed collections like `HashSet` and `HashMap`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use the discriminant of the enum as a simple hash value
        std::mem::discriminant(self).hash(state);
    }
}

impl Serialize for DateTimeError {
    /// Serializes the `DateTimeError` into a string representation.
    ///
    /// This is a custom implementation to handle serialization for variants
    /// that contain types (`Parse` and `ComponentRange`) which do not implement
    /// `Serialize`.
    ///
    /// # Errors
    ///
    /// This function will return a serialization error if the process fails.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::InvalidFormat => {
                serializer.serialize_str("InvalidFormat")
            }
            Self::InvalidTimezone => {
                serializer.serialize_str("InvalidTimezone")
            }
            Self::InvalidDate => {
                serializer.serialize_str("InvalidDate")
            }
            Self::InvalidTime => {
                serializer.serialize_str("InvalidTime")
            }
            Self::ParseError(_) => {
                serializer.serialize_str("ParseError")
            }
            Self::ComponentRange(_) => {
                serializer.serialize_str("ComponentRange")
            }
        }
    }
}

impl<'de> Deserialize<'de> for DateTimeError {
    /// Deserializes a string into a `DateTimeError`.
    ///
    /// This is a custom implementation to handle deserialization for variants
    /// that contain types (`Parse` and `ComponentRange`) which do not implement
    /// `Deserialize`.
    ///
    /// # Errors
    ///
    /// This function will return a deserialization error if the input string
    /// does not match any of the known variants.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "InvalidFormat" => Ok(Self::InvalidFormat),
            "InvalidTimezone" => Ok(Self::InvalidTimezone),
            "InvalidDate" => Ok(Self::InvalidDate),
            "InvalidTime" => Ok(Self::InvalidTime),
            "ParseError" => Err(serde::de::Error::custom(
                "Cannot deserialize ParseError directly",
            )),
            "ComponentRange" => Err(serde::de::Error::custom(
                "Cannot deserialize ComponentRange directly",
            )),
            _ => Err(serde::de::Error::unknown_variant(
                s,
                &[
                    "InvalidFormat",
                    "InvalidTimezone",
                    "InvalidDate",
                    "InvalidTime",
                    "ParseError",
                    "ComponentRange",
                ],
            )),
        }
    }
}

impl Default for DateTimeError {
    /// Provides a default value for `DateTimeError`.
    ///
    /// By default, the error is set to `InvalidFormat`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dtt::error::DateTimeError;
    ///
    /// let error = DateTimeError::default();
    /// assert_eq!(error, DateTimeError::InvalidFormat);
    /// ```
    fn default() -> Self {
        Self::InvalidFormat
    }
}
