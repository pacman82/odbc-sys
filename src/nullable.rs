pub use crate::SQLSMALLINT;
pub use std::convert::TryFrom;

pub use self::Nullable::*;

/// Indicates whether the parameter allows NULL values.
///
/// NOT INTENDED TO BE USED DIRECTLY:
/// In their application users are greatly encouraged to use `Nullable` enum which provides better
/// safety guarantees by implementing `TryFrom<NULLABLE>` and only convert to/from `NULLABLE` when
/// interfacing API which consumes `NULLABLE` arguments
pub type NULLABLE = SQLSMALLINT;

/// Indicates whether the parameter allows NULL values.
#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Nullable {
    SQL_NO_NULLS = 0,
    SQL_NULLABLE = 1,
    SQL_NULLABLE_UNKNOWN = 2,
}

impl TryFrom<NULLABLE> for Nullable {
    type Error = NULLABLE;

    fn try_from(source: NULLABLE) -> Result<Self, Self::Error> {
        match source {
            x if x == SQL_NO_NULLS as NULLABLE => Ok(SQL_NO_NULLS),
            x if x == SQL_NULLABLE as NULLABLE => Ok(SQL_NULLABLE),
            x if x == SQL_NULLABLE_UNKNOWN as NULLABLE => Ok(SQL_NULLABLE_UNKNOWN),

            unknown => Err(unknown),
        }
    }
}
