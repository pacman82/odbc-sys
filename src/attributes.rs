use std::os::raw::{c_ulong, c_void};

/// Governs behaviour of EnvironmentAttribute
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnvironmentAttribute {
    SQL_ATTR_ODBC_VERSION = 200,
    SQL_ATTR_CONNECTION_POOLING = 201,
    SQL_ATTR_CP_MATCH = 202,
    // For private driver manager
    SQL_ATTR_APPLICATION_KEY = 203,
    SQL_ATTR_OUTPUT_NTS = 10001,
}
pub use EnvironmentAttribute::*;

/// ODBC verions
///
/// Used in conjunction with `SQL_ATTR_ODBC_VERSION` and `SQLSetEnvAttr` to declare the ODBC
/// version used by the application.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OdbcVersion {
    SQL_OV_ODBC2 = 2,
    SQL_OV_ODBC3 = 3,
    #[cfg(feature = "odbc_version_3_80")] SQL_OV_ODBC3_80 = 380,
    #[cfg(feature = "odbc_version_4")] SQL_OV_ODBC4 = 400,
}
pub use OdbcVersion::*;

impl From<OdbcVersion> for *mut c_void {
    fn from(source: OdbcVersion) -> *mut c_void {
        source as c_ulong as *mut c_void
    }
}
