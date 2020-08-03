use crate::Pointer;

/// Governs behaviour of EnvironmentAttribute
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnvironmentAttribute {
    OdbcVersion = 200,
    ConnectionPooling = 201,
    CpMatch = 202,
    // This attribute was commented out because there is no mention of it in the ODBC
    // specification nor does this attribute exist in unixODBC or iODBC implementations.
    // This attribute exists in Microsoft implementation only and it's usage is unclear.
    // For private driver manager
    //SQL_ATTR_APPLICATION_KEY = 203,
    OutputNts = 10001,
}

/// ODBC verions
///
/// Possible values for `OdbcVersion` attribute set with `SQLSetEnvAttr` to
/// declare ODBC version
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AttrOdbcVersion {
    // Not supported by this crate
    // SQL_OV_ODBC2 = 2,
    Odbc3 = 3,
    #[cfg(feature = "odbc_version_3_80")]
    Odbc3_80 = 380,
    #[cfg(feature = "odbc_version_4")]
    Odbc4 = 400,
}

impl From<AttrOdbcVersion> for Pointer {
    fn from(source: AttrOdbcVersion) -> Pointer {
        source as i32 as Pointer
    }
}
/// Connection pool configuration
///
/// Possible values for `ConnectionPooling` attribute set with `SQLSetEnvAttr` to define
/// which pooling scheme will be used
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AttrConnectionPooling {
    Off = 0,
    OnePerDriver = 1,
    OnePerHenv = 2,
    DriverAware = 3,
}

/// Connection pool default configuration
impl Default for AttrConnectionPooling {
    fn default() -> Self {
        AttrConnectionPooling::Off
    }
}

impl From<AttrConnectionPooling> for Pointer {
    fn from(source: AttrConnectionPooling) -> Pointer {
        source as u32 as Pointer
    }
}

/// Matching of pooled connections
///
/// Possible values for `CpMatch` attribute set with `SQLSetEnvAttr` to define
/// which connection attributes must match for a connection returned from the pool
#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AttrCpMatch {
    Strict = 0,
    Relaxed = 1,
}

/// Default matching for connections returned from the pool
impl Default for AttrCpMatch {
    fn default() -> Self {
        AttrCpMatch::Strict
    }
}

impl From<AttrCpMatch> for Pointer {
    fn from(source: AttrCpMatch) -> Pointer {
        source as u32 as Pointer
    }
}
