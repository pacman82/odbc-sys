use SQLPOINTER;

pub use self::SQL_ATTR_CONNECTION_POOLING::*;
pub use self::SQL_ATTR_CP_MATCH::*;
pub use self::SQL_ATTR_ODBC_VERSION::*;
pub use EnvironmentAttribute::*;

pub extern crate odbc_sys_derive;
pub use odbc_sys_derive::{EnumDefault, IntoSQLPOINTER};

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
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoSQLPOINTER)]
pub enum SQL_ATTR_ODBC_VERSION {
    // Not supported by this crate
    // SQL_OV_ODBC2 = 2,
    SQL_OV_ODBC3 = 3,
    #[cfg(feature = "odbc_version_3_80")]
    SQL_OV_ODBC3_80 = 380,
    #[cfg(feature = "odbc_version_4")]
    SQL_OV_ODBC4 = 400,
}

/// Connection pool configuration
///
/// Possible values for `SQL_ATTR_CONNECTION_POOLING` attribute set with `SQLSetEnvAttr` to define
/// which pooling scheme will be used
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoSQLPOINTER, EnumDefault)]
pub enum SQL_ATTR_CONNECTION_POOLING {
    #[default(SQL_CP_DEFAULT)]
    SQL_CP_OFF = 0,
    SQL_CP_ONE_PER_DRIVER = 1,
    SQL_CP_ONE_PER_HENV = 2,
    SQL_CP_DRIVER_AWARE = 3,
}

/// Matching of pooled connections
///
/// Possible values for `SQL_ATTR_CP_MATCH` attribute set with `SQLSetEnvAttr` to define
/// which connection attributes must match for a connection returned from the pool
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntoSQLPOINTER, EnumDefault)]
pub enum SQL_ATTR_CP_MATCH {
    #[default(SQL_CP_MATCH_DEFAULT)]
    SQL_CP_STRICT_MATCH = 0,
    SQL_CP_RELAXED_MATCH = 1,
}
