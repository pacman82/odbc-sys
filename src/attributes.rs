use crate::SQLPOINTER;
use crate::SQLUINTEGER;
use crate::SQL_INFO;
use std::convert::TryFrom;

pub use EnvironmentAttribute::*;
pub use SQL_ATTR_CONNECTION_POOLING::*;
pub use SQL_ATTR_CP_MATCH::*;
pub use SQL_ATTR_ODBC_VERSION::*;

/// Environment attribute value
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EnvAttributeValue(SQLPOINTER);

/// Governs behaviour of EnvironmentAttribute
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnvironmentAttribute {
    SQL_ATTR_ODBC_VERSION = 200,
    SQL_ATTR_CONNECTION_POOLING = 201,
    SQL_ATTR_CP_MATCH = 202,
    // This attribute was commented out because there is no mention of it in the ODBC
    // specification nor does this attribute exist in unixODBC or iODBC implementations.
    // This attribute exists in Microsoft implementation only and it's usage is unclear.
    // For private driver manager
    //SQL_ATTR_APPLICATION_KEY = 203,
    SQL_ATTR_OUTPUT_NTS = 10001,
}

/// ODBC verions
///
/// Possible values for `SQL_ATTR_ODBC_VERSION` attribute set with `SQLSetEnvAttr` to
/// declare ODBC version
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SQL_ATTR_ODBC_VERSION {
    // Not supported by this crate
    // SQL_OV_ODBC2 = 2,
    SQL_OV_ODBC3 = 3,
    #[cfg(feature = "odbc_version_3_80")]
    SQL_OV_ODBC3_80 = 380,
    #[cfg(feature = "odbc_version_4")]
    SQL_OV_ODBC4 = 400,
}
impl SQL_ATTR_ODBC_VERSION {
    #[inline]
    // Must not be public
    fn into_i32(self) -> i32 {
        self as i32
    }
}
impl From<SQL_ATTR_ODBC_VERSION> for EnvAttributeValue {
    fn from(source: SQL_ATTR_ODBC_VERSION) -> Self {
        EnvAttributeValue(source.into_i32() as SQLPOINTER)
    }
}
impl TryFrom<EnvAttributeValue> for SQL_ATTR_ODBC_VERSION {
    type Error = EnvAttributeValue;

    fn try_from(source: EnvAttributeValue) -> Result<Self, Self::Error> {
        match source {
            x if x.0 as i32 == SQL_OV_ODBC3.into_i32() => Ok(SQL_OV_ODBC3),
            #[cfg(feature = "odbc_version_3_80")]
            x if x.0 as i32 == SQL_OV_ODBC3_80.into_i32() => Ok(SQL_OV_ODBC3_80),
            #[cfg(feature = "odbc_version_4")]
            x if x.0 as i32 == SQL_OV_ODBC4.into_i32() => Ok(SQL_OV_ODBC4),

            unknown => Err(unknown),
        }
    }
}

/// Connection pool configuration
///
/// Possible values for `SQL_ATTR_CONNECTION_POOLING` attribute set with `SQLSetEnvAttr` to define
/// which pooling scheme will be used
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SQL_ATTR_CONNECTION_POOLING {
    SQL_CP_OFF = 0,
    SQL_CP_ONE_PER_DRIVER = 1,
    SQL_CP_ONE_PER_HENV = 2,
    SQL_CP_DRIVER_AWARE = 3,
}
impl SQL_ATTR_CONNECTION_POOLING {
    // Must not be public
    #[inline]
    #[allow(non_snake_case)]
    fn into_SQLUINTEGER(self) -> SQLUINTEGER {
        self as SQLUINTEGER
    }
}
/// Connection pool default configuration
pub use SQL_CP_OFF as SQL_CP_DEFAULT;
impl Default for SQL_ATTR_CONNECTION_POOLING {
    fn default() -> Self {
        SQL_CP_DEFAULT
    }
}
impl From<SQL_ATTR_CONNECTION_POOLING> for EnvAttributeValue {
    fn from(source: SQL_ATTR_CONNECTION_POOLING) -> Self {
        EnvAttributeValue(source.into_SQLUINTEGER() as SQLPOINTER)
    }
}
impl TryFrom<EnvAttributeValue> for SQL_ATTR_CONNECTION_POOLING {
    type Error = EnvAttributeValue;

    fn try_from(source: EnvAttributeValue) -> Result<Self, Self::Error> {
        match source {
            x if x.0 as SQLUINTEGER == SQL_CP_OFF.into_SQLUINTEGER() => Ok(SQL_CP_OFF),
            x if x.0 as SQLUINTEGER == SQL_CP_ONE_PER_DRIVER.into_SQLUINTEGER() => {
                Ok(SQL_CP_ONE_PER_DRIVER)
            }
            x if x.0 as SQLUINTEGER == SQL_CP_ONE_PER_HENV.into_SQLUINTEGER() => {
                Ok(SQL_CP_ONE_PER_HENV)
            }
            x if x.0 as SQLUINTEGER == SQL_CP_DRIVER_AWARE.into_SQLUINTEGER() => {
                Ok(SQL_CP_DRIVER_AWARE)
            }

            unknown => Err(unknown),
        }
    }
}

/// Matching of pooled connections
///
/// Possible values for `SQL_ATTR_CP_MATCH` attribute set with `SQLSetEnvAttr` to define
/// which connection attributes must match for a connection returned from the pool
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SQL_ATTR_CP_MATCH {
    SQL_CP_STRICT_MATCH = 0,
    SQL_CP_RELAXED_MATCH = 1,
}
impl SQL_ATTR_CP_MATCH {
    // Must not be public
    #[inline]
    #[allow(non_snake_case)]
    fn into_SQLUINTEGER(self) -> SQLUINTEGER {
        self as SQLUINTEGER
    }
}

/// Default matching for connections returned from the pool
pub use SQL_CP_STRICT_MATCH as SQL_CP_MATCH_DEFAULT;
impl Default for SQL_ATTR_CP_MATCH {
    fn default() -> Self {
        SQL_CP_MATCH_DEFAULT
    }
}
impl From<SQL_ATTR_CP_MATCH> for EnvAttributeValue {
    fn from(source: SQL_ATTR_CP_MATCH) -> Self {
        EnvAttributeValue(source.into_SQLUINTEGER() as SQLPOINTER)
    }
}
impl TryFrom<EnvAttributeValue> for SQL_ATTR_CP_MATCH {
    type Error = EnvAttributeValue;

    fn try_from(source: EnvAttributeValue) -> Result<Self, Self::Error> {
        match source {
            x if x.0 as SQLUINTEGER == SQL_CP_STRICT_MATCH.into_SQLUINTEGER() => {
                Ok(SQL_CP_STRICT_MATCH)
            }
            x if x.0 as SQLUINTEGER == SQL_CP_RELAXED_MATCH.into_SQLUINTEGER() => {
                Ok(SQL_CP_RELAXED_MATCH)
            }

            unknown => Err(unknown),
        }
    }
}

/// Determines how the driver returns string data.
///
/// If `SQL_TRUE`, the driver returns string data null-terminated. If `SQL_FALSE`, the driver does
/// not return string data null-terminated.
#[allow(non_camel_case_types)]
pub type SQL_ATTR_OUTPUT_NTS = SQL_INFO;

impl SQL_ATTR_OUTPUT_NTS {
    // Must not be public
    #[inline]
    fn into_i32(self) -> i32 {
        self as i32
    }
}
impl Default for SQL_ATTR_OUTPUT_NTS {
    fn default() -> Self {
        SQL_ATTR_OUTPUT_NTS::SQL_FALSE
    }
}
impl From<SQL_ATTR_OUTPUT_NTS> for EnvAttributeValue {
    fn from(source: SQL_ATTR_OUTPUT_NTS) -> Self {
        EnvAttributeValue(source.into_i32() as SQLPOINTER)
    }
}
impl TryFrom<EnvAttributeValue> for SQL_ATTR_OUTPUT_NTS {
    type Error = EnvAttributeValue;

    fn try_from(source: EnvAttributeValue) -> Result<Self, Self::Error> {
        match source {
            x if x.0 as i32 == SQL_ATTR_OUTPUT_NTS::SQL_FALSE as i32 => {
                Ok(SQL_ATTR_OUTPUT_NTS::SQL_FALSE)
            }
            x if x.0 as i32 == SQL_ATTR_OUTPUT_NTS::SQL_TRUE as i32 => {
                Ok(SQL_ATTR_OUTPUT_NTS::SQL_TRUE)
            }
            unknown => Err(unknown),
        }
    }
}
