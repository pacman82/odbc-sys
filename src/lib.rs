//! ODBC types those representation is compatible with the ODBC C API.
//!
//! This layer has not been created using automatic code generation. It is incomplete, i.e. it does
//! not contain every symbol or constant defined in the ODBC C headers. Symbols which are
//! deprecated since ODBC 3 have been left out intentionally. While some extra type safety has been
//! added by grouping some of C's `#define` constants into `enum`-types it mostly offers the same
//! power (all) and safety guarantess(none) as the wrapped C-API.
//! ODBC 4.0 is still under development by Microsoft, so these symbols are deactivated by default
//! in the cargo.toml

mod sqlreturn;
pub use self::sqlreturn::*;
mod info_type;
pub use self::info_type::*;
use std::os::raw::{c_void, c_short, c_ushort, c_int, c_ulong};

//These types can never be instantiated in Rust code.
pub enum Obj {}
pub enum Env {}
pub enum Dbc {}
pub enum Stmt {}

pub type SQLHANDLE = *mut Obj;
pub type SQLHENV = *mut Env;
pub type SQLHDBC = *mut Dbc;
pub type SQLHSTMT = *mut Stmt;

pub type SQLSMALLINT = c_short;
pub type SQLUSMALLINT = c_ushort;
pub type SQLINTEGER = c_int;
pub type SQLPOINTER = *mut c_void;
pub type SQLCHAR = u8;

#[cfg(all(windows, target_pointer_width = "64"))]
pub type SQLLEN = i64;
#[cfg(not(windows))]
pub type SQLLEN = SQLINTEGER;

pub type SQLHWND = SQLPOINTER;

pub const SQL_ATTR_ODBC_VERSION: SQLINTEGER = 200;

// ODBC verions
pub const SQL_OV_ODBC2: c_ulong = 2;
pub const SQL_OV_ODBC3: c_ulong = 3;
#[cfg(feature = "odbc_version_3_80")]
pub const SQL_OV_ODBC3_80: c_ulong = 380;
#[cfg(feature = "odbc_version_4")]
pub const SQL_OV_ODBC4: c_ulong = 400;

/// Maximum message length
pub const SQL_MAX_MESSAGE_LENGTH: SQLSMALLINT = 512;
pub const SQL_SQLSTATE_SIZE: usize = 5;

// Special SQLGetData indicator values
pub const SQL_NULL_DATA: SQLLEN = -1;
pub const SQL_NO_TOTAL: SQLLEN = -4;

/// SQL Data Types
#[repr(i16)]
#[allow(non_camel_case_types)]
pub enum SqlDataType {
    SQL_UNKNOWN_TYPE = 0, // also called SQL_VARIANT_TYPE since odbc 4.0
    SQL_CHAR = 1,
    SQL_NUMERIC = 2,
    SQL_DECIMAL = 3,
    SQL_INTEGER = 4,
    SQL_SMALLINT = 5,
    SQL_FLOAT = 6,
    SQL_REAL = 7,
    SQL_DOUBLE = 8,
    SQL_DATETIME = 9,
    SQL_VARCHAR = 12,
    #[cfg(feature = "odbc_version_4")]
    SQL_UDT = 17,
    #[cfg(feature = "odbc_version_4")]
    SQL_ROW = 19,
    #[cfg(feature = "odbc_version_4")]
    SQL_ARRAY = 50,
    #[cfg(feature = "odbc_version_4")]
    SQL_MULTISET = 55,
}
pub use self::SqlDataType::*;

#[repr(i16)]
#[allow(non_camel_case_types)]
pub enum SqlCDataType {
    SQL_C_CHAR = 1,
}
pub use self::SqlCDataType::*;

#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SqlDriverConnectOption {
    SQL_DRIVER_NOPROMPT = 0,
    SQL_DRIVER_COMPLETE = 1,
    SQL_DRIVER_PROMPT = 2,
    SQL_DRIVER_COMPLETE_REQUIRED = 3,
}
pub use self::SqlDriverConnectOption::*;

/// Represented in C headers as SQLSMALLINT
#[repr(i16)]
#[allow(non_camel_case_types)]
pub enum HandleType {
    SQL_HANDLE_ENV = 1,
    SQL_HANDLE_DBC = 2,
    SQL_HANDLE_STMT = 3,
    SQL_HANDLE_DESC = 4,
}
pub use self::HandleType::*;

#[cfg_attr(windows, link(name="odbc32"))]
#[cfg_attr(not(windows), link(name="odbc"))]
extern "C" {
    pub fn SQLAllocHandle(handle_type: HandleType,
                          input_handle: SQLHANDLE,
                          output_Handle: *mut SQLHANDLE)
                          -> SQLRETURN;

    pub fn SQLFreeHandle(handle_type: HandleType, handle: SQLHANDLE) -> SQLRETURN;
    pub fn SQLSetEnvAttr(environment_handle: SQLHENV,
                         attribute: SQLINTEGER,
                         value: SQLPOINTER,
                         string_length: SQLINTEGER)
                         -> SQLRETURN;

    pub fn SQLDriverConnect(connection_handle: SQLHDBC,
                            window_handle: SQLHWND,
                            in_connection_string: *const SQLCHAR,
                            string_length_1: SQLSMALLINT,
                            out_connection_string: *mut SQLCHAR,
                            buffer_length: SQLSMALLINT,
                            string_length_2: *mut SQLSMALLINT,
                            DriverCompletion: SqlDriverConnectOption)
                            -> SQLRETURN;

    pub fn SQLDisconnect(connection_handle: SQLHDBC) -> SQLRETURN;

    pub fn SQLGetDiagRec(handle_type: HandleType,
                         handle: SQLHANDLE,
                         RecNumber: SQLSMALLINT,
                         state: *mut SQLCHAR,
                         native_error_ptr: *mut SQLINTEGER,
                         message_text: *mut SQLCHAR,
                         buffer_length: SQLSMALLINT,
                         text_length_ptr: *mut SQLSMALLINT)
                         -> SQLRETURN;

    pub fn SQLExecDirect(statement_handle: SQLHSTMT,
                         statement_text: *const SQLCHAR,
                         text_length: SQLINTEGER)
                         -> SQLRETURN;

    pub fn SQLNumResultCols(statement_handle: SQLHSTMT,
                            column_count_ptr: *mut SQLSMALLINT)
                            -> SQLRETURN;

    // Can be used since odbc version 3.8 to stream results
    pub fn SQLGetData(statement_handle: SQLHSTMT,
                      col_or_param_num: SQLUSMALLINT,
                      target_type: SqlCDataType,
                      target_value_ptr: SQLPOINTER,
                      buffer_length: SQLLEN,
                      str_len_or_ind_ptr: *mut SQLLEN)
                      -> SQLRETURN;

    pub fn SQLFetch(statement_handle: SQLHSTMT) -> SQLRETURN;

    /// Returns general information about the driver and data source associated with a connection
    pub fn SQLGetInfo(connection_handle: SQLHDBC,
                      info_type: InfoType,
                      info_value_ptr: SQLPOINTER,
                      buffer_length: SQLSMALLINT,
                      string_length_ptr: *mut SQLSMALLINT)
                      -> SQLRETURN;
}
