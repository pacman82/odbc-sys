use crate::SQLPOINTER;

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
pub use EnvironmentAttribute::*;

/// ODBC verions
///
/// Possible values for `SQL_ATTR_ODBC_VERSION` attribute set with `SQLSetEnvAttr` to
/// declare ODBC version
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OdbcVersion {
    SQL_OV_ODBC2 = 2,
    SQL_OV_ODBC3 = 3,
    #[cfg(feature = "odbc_version_3_80")]
    SQL_OV_ODBC3_80 = 380,
    #[cfg(feature = "odbc_version_4")]
    SQL_OV_ODBC4 = 400,
}
pub use OdbcVersion::*;

impl From<OdbcVersion> for SQLPOINTER {
    fn from(source: OdbcVersion) -> SQLPOINTER {
        source as i32 as SQLPOINTER
    }
}

/// Connection pool configuration
///
/// Possible values for `SQL_ATTR_CONNECTION_POOLING` attribute set with `SQLSetEnvAttr` to define
/// which pooling scheme will be used
#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnPoolConfig {
    SQL_CP_OFF = 0,
    SQL_CP_ONE_PER_DRIVER = 1,
    SQL_CP_ONE_PER_HENV = 2,
    SQL_CP_DRIVER_AWARE = 3,
}
pub use ConnPoolConfig::*;

/// Connection pool default configuration
pub const SQL_CP_DEFAULT: ConnPoolConfig = ConnPoolConfig::SQL_CP_OFF;

impl From<ConnPoolConfig> for SQLPOINTER {
    fn from(source: ConnPoolConfig) -> SQLPOINTER {
        source as u32 as SQLPOINTER
    }
}

/// Matching of pooled connections
///
/// Possible values for `SQL_ATTR_CP_MATCH` attribute set with `SQLSetEnvAttr` to define
/// which connection attributes must match for a connection returned from the pool
#[allow(non_camel_case_types)]
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ConnPoolMatch {
    SQL_CP_STRICT_MATCH = 0,
    SQL_CP_RELAXED_MATCH = 1,
}
pub use ConnPoolMatch::*;

/// Default matching for connections returned from the pool
pub const SQL_CP_MATCH_DEFAULT: ConnPoolMatch = ConnPoolMatch::SQL_CP_STRICT_MATCH;

impl From<ConnPoolMatch> for SQLPOINTER {
    fn from(source: ConnPoolMatch) -> SQLPOINTER {
        source as u32 as SQLPOINTER
    }
}

/// Enable/disable null-terminated strings
///
/// Possible values for `SQL_ATTR_OUTPUT_NTS` attribute set with `SQLSetEnvAttr` to
/// enable/disable null-terminated strings
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OutputNTS {
    SQL_FALSE = 0,
    SQL_TRUE = 1,
}
pub use OutputNTS::*;

impl From<OutputNTS> for SQLPOINTER {
    fn from(source: OutputNTS) -> SQLPOINTER {
        source as i32 as SQLPOINTER
    }
}
