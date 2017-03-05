use std::os::raw::c_ulong;

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
pub use self::EnvironmentAttribute::*;

// ODBC verions
pub const SQL_OV_ODBC2: c_ulong = 2;
pub const SQL_OV_ODBC3: c_ulong = 3;
#[cfg(feature = "odbc_version_3_80")]
pub const SQL_OV_ODBC3_80: c_ulong = 380;
#[cfg(feature = "odbc_version_4")]
pub const SQL_OV_ODBC4: c_ulong = 400;

