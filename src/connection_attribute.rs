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

    /// Equivalent to `SQL_COPT_SS_ACCESS_TOKEN` in `msodbcsql.h`.
    ///
    /// Pre-connect attribute to authenticate with a Microsoft Entra ID access token. Specific to
    /// the Microsoft ODBC Driver for SQL Server. Setting this for other drivers could cause
    /// undefined behavior. The value is a pointer to an [`SsAccessToken`]. It must stay allocated
    /// for as long as the connection handle is allocated.
    ///
    /// See:
    /// <https://learn.microsoft.com/sql/connect/odbc/using-azure-active-directory#authenticating-with-an-access-token>
    pub const SS_ACCESS_TOKEN: Self = Self(1256);
}

/// Equivalent to `ACCESSTOKEN` in `msodbcsql.h`. Value of [`ConnectionAttribute::SS_ACCESS_TOKEN`].
///
/// Specific to the Microsoft ODBC Driver for SQL Server. A dynamically sized type: pointers to it
/// are fat and carry the length of `data`, which must equal `data_size`. Cast to a thin pointer
/// before passing it to the driver.
///
/// See:
/// <https://learn.microsoft.com/sql/connect/odbc/using-azure-active-directory#authenticating-with-an-access-token>
#[repr(C)]
pub struct SsAccessToken {
    /// Length of `data` in bytes. Does not include a terminating zero.
    pub data_size: u32,
    /// Flexible array member (`BYTE data[]` in C). Directly follows `data_size`.
    ///
    /// According to Microsoft, a token obtained from an OAuth 2.0 JSON response must be expanded,
    /// so each byte is followed by a zero byte. For the ASCII characters of an access token, this
    /// equals UTF-16 with little endian byte order. This holds whether the narrow or wide ODBC
    /// functions are used, since the driver manager passes the pointer through unchanged.
    pub data: [u8],
}
