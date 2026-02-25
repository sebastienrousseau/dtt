// SPDX-License-Identifier: MIT OR Apache-2.0

//! Native database driver support ecosystem.
//!
//! Provides `Type`, `Encode`, and `Decode` traits allowing `dtt::DateTime`
//! to fully integrate into massive backend infrastructures.
//!
//! Supported Ecosystems:
//! * `sqlx` (via `db-sqlx`)
//! * `postgres` / `tokio-postgres` (via `db-postgres`)
//! * `diesel` (via `db-diesel`)

use crate::DateTime;

// ============================================================================
// SQLX INTEGRATION
// ============================================================================

#[cfg(feature = "db-sqlx")]
use sqlx::{
    decode::Decode,
    encode::{Encode, IsNull},
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef, Postgres},
    Type,
};

#[cfg(feature = "db-sqlx")]
#[cfg(not(tarpaulin))]
impl Type<Postgres> for DateTime {
    fn type_info() -> PgTypeInfo {
        <time::OffsetDateTime as Type<Postgres>>::type_info()
    }
    fn compatible(ty: &PgTypeInfo) -> bool {
        <time::OffsetDateTime as Type<Postgres>>::compatible(ty)
    }
}

#[cfg(feature = "db-sqlx")]
#[cfg(not(tarpaulin))]
impl<'q> Encode<'q, Postgres> for DateTime {
    fn encode_by_ref(
        &self,
        buf: &mut PgArgumentBuffer,
    ) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync>> {
        let odt = time::OffsetDateTime::from_unix_timestamp_nanos(
            (i128::from(self.unix_seconds) * 1_000_000_000)
                + i128::from(self.nanoseconds),
        )
        .unwrap_or_else(|_| time::OffsetDateTime::now_utc())
        .to_offset(self.offset());
        <time::OffsetDateTime as Encode<'q, Postgres>>::encode_by_ref(
            &odt, buf,
        )
    }
}

#[cfg(feature = "db-sqlx")]
#[cfg(not(tarpaulin))]
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
            #[allow(clippy::cast_possible_truncation)]
            utc_offset_minutes: (odt.offset().whole_seconds() / 60)
                as i16,
        })
    }
}

// ============================================================================
// DIESEL INTEGRATION (Postgres)
// ============================================================================

#[cfg(feature = "db-diesel")]
use diesel::{
    backend::Backend,
    deserialize::{self, FromSql},
    pg::Pg,
    serialize::{self, IsNull as DieselIsNull, Output, ToSql},
    sql_types::Timestamptz,
};
#[cfg(feature = "db-diesel")]
use std::io::Write;

#[cfg(feature = "db-diesel")]
#[cfg(not(tarpaulin))]
impl ToSql<Timestamptz, Pg> for DateTime {
    fn to_sql<'b>(
        &'b self,
        out: &mut Output<'b, '_, Pg>,
    ) -> serialize::Result {
        let odt = time::OffsetDateTime::from_unix_timestamp_nanos(
            (i128::from(self.unix_seconds) * 1_000_000_000)
                + i128::from(self.nanoseconds),
        )
        .unwrap_or_else(|_| time::OffsetDateTime::now_utc())
        .to_offset(self.offset());
        let pg_epoch = time::macros::datetime!(2000-01-01 0:00 UTC);
        let microsecs = (odt - pg_epoch).whole_microseconds() as i64;
        out.write_all(&microsecs.to_be_bytes())?;
        Ok(DieselIsNull::No)
    }
}

#[cfg(feature = "db-diesel")]
#[cfg(not(tarpaulin))]
impl FromSql<Timestamptz, Pg> for DateTime {
    fn from_sql(
        bytes: <Pg as Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let odt = <time::OffsetDateTime as FromSql<Timestamptz, Pg>>::from_sql(bytes)?;
        Ok(Self {
            unix_seconds: odt.unix_timestamp(),
            nanoseconds: odt.nanosecond(),
            #[allow(clippy::cast_possible_truncation)]
            utc_offset_minutes: (odt.offset().whole_seconds() / 60)
                as i16,
        })
    }
}

// ============================================================================
// NATIVE POSTGRES (tokio-postgres / postgres)
// ============================================================================

#[cfg(feature = "db-postgres")]
use bytes::BytesMut;
#[cfg(feature = "db-postgres")]
use postgres_types::{
    FromSql as PgFromSql, IsNull as PgIsNull, ToSql as PgToSql,
    Type as PgType,
};

#[cfg(feature = "db-postgres")]
#[cfg(not(tarpaulin))]
impl<'a> PgFromSql<'a> for DateTime {
    fn from_sql(
        ty: &PgType,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let odt =
            <time::OffsetDateTime as PgFromSql>::from_sql(ty, raw)?;
        Ok(Self {
            unix_seconds: odt.unix_timestamp(),
            nanoseconds: odt.nanosecond(),
            #[allow(clippy::cast_possible_truncation)]
            utc_offset_minutes: (odt.offset().whole_seconds() / 60)
                as i16,
        })
    }
    fn accepts(ty: &PgType) -> bool {
        <time::OffsetDateTime as PgFromSql>::accepts(ty)
    }
}

#[cfg(feature = "db-postgres")]
#[cfg(not(tarpaulin))]
impl PgToSql for DateTime {
    fn to_sql(
        &self,
        ty: &PgType,
        out: &mut BytesMut,
    ) -> Result<PgIsNull, Box<dyn std::error::Error + Sync + Send>>
    {
        let odt = time::OffsetDateTime::from_unix_timestamp_nanos(
            (i128::from(self.unix_seconds) * 1_000_000_000)
                + i128::from(self.nanoseconds),
        )
        .unwrap_or_else(|_| time::OffsetDateTime::now_utc())
        .to_offset(self.offset());
        <time::OffsetDateTime as PgToSql>::to_sql(&odt, ty, out)
    }
    fn accepts(ty: &PgType) -> bool {
        <time::OffsetDateTime as PgToSql>::accepts(ty)
    }
    postgres_types::to_sql_checked!();
}
