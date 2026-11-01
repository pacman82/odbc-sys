use std::{
    alloc::{alloc, handle_alloc_error, Layout},
    ptr,
};

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

impl SsAccessToken {
    /// `data` must be the access token in the format expected by the driver: each byte of the
    /// ASCII token followed by a zero byte. Use [`Self::from_text`] to pass the token text instead.
    ///
    /// # Panics
    ///
    /// If `data` is larger than `u32::MAX` bytes.
    pub fn new(data: &[u8]) -> Box<Self> {
        let data_size =
            u32::try_from(data.len()).expect("Access token must not be larger than u32::MAX bytes");
        // Can not fail, since `data.len()` fits into an `u32`.
        let (layout, data_offset) = Layout::new::<u32>()
            .extend(Layout::array::<u8>(data.len()).unwrap())
            .unwrap();
        let layout = layout.pad_to_align();
        // Safety: `layout` has a non-zero size, since it contains at least the `u32` header. Header
        // and data are written within the allocation, at the offsets computed by `Layout::extend`.
        // The fat pointer carries `data.len()` as length of `data`, so `Layout::for_value` of the
        // resulting box equals `layout`, as required by `Box::from_raw`.
        unsafe {
            let raw = alloc(layout);
            if raw.is_null() {
                handle_alloc_error(layout);
            }
            raw.cast::<u32>().write(data_size);
            ptr::copy_nonoverlapping(data.as_ptr(), raw.add(data_offset), data.len());
            Box::from_raw(ptr::slice_from_raw_parts_mut(raw, data.len()) as *mut SsAccessToken)
        }
    }

    /// `token` is the access token as text, e.g. obtained from an OAuth 2.0 JSON response.
    ///
    /// # Panics
    ///
    /// If the encoded token is larger than `u32::MAX` bytes.
    pub fn from_text(token: &str) -> Box<Self> {
        let data: Vec<u8> = token.encode_utf16().flat_map(u16::to_le_bytes).collect();
        Self::new(&data)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::{align_of_val, size_of_val};

    use super::SsAccessToken;

    #[test]
    fn access_token_from_str_is_encoded_as_utf16_little_endian() {
        let token = SsAccessToken::from_text("eyJ");

        assert_eq!(6, token.data_size);
        assert_eq!(&[b'e', 0, b'y', 0, b'J', 0], &token.data);
    }

    #[test]
    fn access_token_new_copies_bytes_verbatim() {
        let token = SsAccessToken::new(&[1, 2, 3]);

        assert_eq!(3, token.data_size);
        assert_eq!(&[1, 2, 3], &token.data);
    }

    #[test]
    fn access_token_has_c_layout() {
        let token = SsAccessToken::new(&[1, 2, 3]);

        let start = &*token as *const SsAccessToken as *const u8 as usize;
        assert_eq!(4, token.data.as_ptr() as usize - start);
        // 4 bytes header, 3 bytes data, padded to alignment of 4
        assert_eq!(8, size_of_val(&*token));
        assert_eq!(4, align_of_val(&*token));
    }
}
