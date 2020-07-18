pub use crate::SQLSMALLINT;
pub use std::convert::TryFrom;

pub use self::Nullable::*;

/// Indicates whether the parameter allows NULL values.
///
/// In their application users should use `Nullable` enum which provides better safety guarantees
/// and only convert to/from `NULLABLE` when interfacing API which consumes `NULLABLE` arguments
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct NULLABLE(SQLSMALLINT);

/// Indicates whether the parameter allows NULL values.
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
            x if x.0 == SQL_NO_NULLS as SQLSMALLINT => Ok(SQL_NO_NULLS),
            x if x.0 == SQL_NULLABLE as SQLSMALLINT => Ok(SQL_NULLABLE),
            x if x.0 == SQL_NULLABLE_UNKNOWN as SQLSMALLINT => Ok(SQL_NULLABLE_UNKNOWN),

            unknown => Err(unknown),
        }
    }
}
