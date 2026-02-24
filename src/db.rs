// SPDX-License-Identifier: MIT OR Apache-2.0

//! Native database driver support (`sqlx`) integration.
//!
//! Provides `Type`, `Encode`, and `Decode` traits allowing `dtt::DateTime`
//! to be seamlessly passed directly into sqlx parameterized queries.
//! Active only when the `db-sqlx` feature is enabled.

use crate::DateTime;

#[cfg(feature = "db-sqlx")]
use sqlx::{
    decode::Decode,
    encode::{Encode, IsNull},
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef, Postgres},
    Type,
};

#[cfg(feature = "db-sqlx")]
impl Type<Postgres> for DateTime {
    fn type_info() -> PgTypeInfo {
        <time::OffsetDateTime as Type<Postgres>>::type_info()
    }

    fn compatible(ty: &PgTypeInfo) -> bool {
        <time::OffsetDateTime as Type<Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "db-sqlx")]
impl<'q> Encode<'q, Postgres> for DateTime {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync>> {
        // Convert primitives into standard OffsetDateTime for Pg encoding
        let odt = time::OffsetDateTime::from_unix_timestamp_nanos(
            (self.unix_seconds as i128 * 1_000_000_000) + self.nanoseconds as i128
        ).unwrap_or_else(|_| time::OffsetDateTime::now_utc())
        .to_offset(self.offset());

        <time::OffsetDateTime as Encode<'q, Postgres>>::encode_by_ref(
            &odt, buf,
        )
    }
}

#[cfg(feature = "db-sqlx")]
impl<'r> Decode<'r, Postgres> for DateTime {
    fn decode(
        value: PgValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>>
    {
        let odt =
            <time::OffsetDateTime as Decode<'r, Postgres>>::decode(
                value,
            )?;
        Ok(Self {
            unix_seconds: odt.unix_timestamp(),
            nanoseconds: odt.nanosecond(),
            utc_offset_minutes: (odt.offset().whole_seconds() / 60) as i16,
        })
    }
}
