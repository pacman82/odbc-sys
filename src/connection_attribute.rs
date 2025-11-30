/// Connection attributes for `SQLSetConnectAttr`
///
/// See: <https://docs.microsoft.com/en-us/sql/odbc/reference/syntax/sqlsetconnectattr-function>
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ConnectionAttribute(pub i32);

impl ConnectionAttribute {
    pub const ASYNC_ENABLE: Self = Self(4);

    /// Equivalent to `SQL_ATTR_ACCESS_MODE` in the C-API
    pub const ACCESS_MODE: Self = Self(101);
    /// Equivalent to `SQL_ATTR_AUTOCOMMIT` in the C-API
    pub const AUTOCOMMIT: Self = Self(102);
    /// Equivalent to `SQL_ATTR_LOGIN_TIMEOUT` in the C-API
    pub const LOGIN_TIMEOUT: Self = Self(103);
    /// Equivalent to `SQL_ATTR_TRACE` in the C-API
    pub const TRACE: Self = Self(104);
    /// Equivalent to `SQL_ATTR_TRACEFILE` in the C-API
    pub const TRACEFILE: Self = Self(105);
    /// Equivalent to `SQL_ATTR_TRANSLATE_LIB` in the C-API
    pub const TRANSLATE_LIB: Self = Self(106);
    /// Equivalent to `SQL_ATTR_TRANSLATE_OPTION` in the C-API
    pub const TRANSLATE_OPTION: Self = Self(107);
    /// Equivalent to `SQL_ATTR_TXN_ISOLATION` in the C-API
    pub const TXN_ISOLATION: Self = Self(108);
    /// Equivalent to `SQL_ATTR_CURRENT_CATALOG` in the C-API
    pub const CURRENT_CATALOG: Self = Self(109);
    /// Equivalent to `SQL_ATTR_ODBC_CURSORS` in the C-API
    pub const ODBC_CURSORS: Self = Self(110);
    /// Equivalent to `SQL_ATTR_QUIET_MODE` in the C-API
    pub const QUIET_MODE: Self = Self(111);
    /// Equivalent to `SQL_ATTR_PACKET_SIZE` in the C-API
    pub const PACKET_SIZE: Self = Self(112);
    /// Equivalent to `SQL_ATTR_CONNECTION_TIMEOUT` in the C-API
    pub const CONNECTION_TIMEOUT: Self = Self(113);
    /// Equivalent to `SQL_ATTR_DISCONNECT_BEHAVIOUR` in the C-API
    pub const DISCONNECT_BEHAVIOUR: Self = Self(114);

    // Attribute 115 is `SQL_ATTR_ANSI_APP`. This attribute is not set by an odbc applictation, but
    // by the driver manager itself. Therfore it is not included here.

    /// Equivalent to `SQL_ATTR_DBC_INFO_TOKEN`
    ///
    /// Reset the pooled connection in case it is not a perfect match
    pub const DBC_INFO_TOKEN: Self = Self(118);

    /// Equivalent to `SQL_ATTR_RESET_CONNECTION` in the C-API
    pub const RESET_CONNECTION: Self = Self(116);
    /// Equivalent to `SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE` in the C-API
    pub const ASYNC_DBC_FUNCTIONS_ENABLE: Self = Self(117);
    /// Equivalent to `SQL_ATTR_ASYNC_DBC_EVENT` in the C-API
    pub const ASYNC_DBC_EVENT: Self = Self(119);
    /// Equivalent to `SQL_ATTR_ENLIST_IN_DTC` in the C-API
    pub const ENLIST_IN_DTC: Self = Self(1207);
    /// Equivalent to `SQL_ATTR_ENLIST_IN_XA` in the C-API
    pub const ENLIST_IN_XA: Self = Self(1208);
    pub const CONNECTION_DEAD: Self = Self(1209);
    pub const AUTO_IPD: Self = Self(10001);
    pub const METADATA_ID: Self = Self(10014);
}
