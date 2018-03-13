//! ODBC types those representation is compatible with the ODBC C API.
//!
//! This layer has not been created using automatic code generation. It is incomplete, i.e. it does
//! not contain every symbol or constant defined in the ODBC C headers. Symbols which are
//! deprecated since ODBC 3 have been left out intentionally. While some extra type safety has been
//! added by grouping some of C's `#define` constants into `enum`-types it mostly offers the same
//! power (all) and safety guarantess(none) as the wrapped C-API.
//! ODBC 4.0 is still under development by Microsoft, so these symbols are deactivated by default
//! in the cargo.toml

pub use self::sqlreturn::*;
pub use self::info_type::*;
pub use self::fetch_orientation::*;
pub use self::attributes::*;
pub use self::c_data_type::*;
pub use self::input_output::*;
pub use self::nullable::*;
pub use self::sql_bulk_operations::*;
use std::os::raw::c_void;

mod sqlreturn;
mod info_type;
mod fetch_orientation;
mod attributes;
mod c_data_type;
mod input_output;
mod nullable;
mod sql_bulk_operations;

//These types can never be instantiated in Rust code.
pub enum Obj {}

pub enum Env {}

pub enum Dbc {}

pub enum Stmt {}

pub enum Desc {}

pub type SQLHANDLE = *mut Obj;
pub type SQLHENV = *mut Env;
pub type SQLHDESC = *mut Desc;

/// The connection handle references storage of all information about the connection to the data
/// source, including status, transaction state, and error information.
pub type SQLHDBC = *mut Dbc;
pub type SQLHSTMT = *mut Stmt;

pub type SQLSMALLINT = i16;
pub type SQLUSMALLINT = u16;
pub type SQLINTEGER = i32;
pub type SQLUINTEGER = u32;
pub type SQLPOINTER = *mut c_void;
pub type SQLCHAR = u8;
pub type SQLWCHAR = u16;

#[cfg(target_pointer_width = "64")]
pub type SQLLEN = i64;
#[cfg(target_pointer_width = "32")]
pub type SQLLEN = SQLINTEGER;

#[cfg(target_pointer_width = "64")]
pub type SQLULEN = u64;
#[cfg(target_pointer_width = "32")]
pub type SQLULEN = SQLUINTEGER;

pub type SQLHWND = SQLPOINTER;

pub type RETCODE = i16;

// flags for null-terminated string
pub const SQL_NTS: SQLSMALLINT = -3;
pub const SQL_NTSL: SQLINTEGER = -3;

/// Maximum message length
pub const SQL_MAX_MESSAGE_LENGTH: SQLSMALLINT = 512;
pub const SQL_SQLSTATE_SIZE: usize = 5;
pub const SQL_SQLSTATE_SIZEW: usize = 10;

// Special SQLGetData indicator values
pub const SQL_NULL_DATA: SQLLEN = -1;
pub const SQL_NO_TOTAL: SQLLEN = -4;

/// SQL Free Statement options
#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FreeStmtOption {
    /// Closes the cursor associated with StatementHandle (if one was defined) and discards all
    /// pending results. The application can reopen this cursor later by executing a SELECT
    /// statement again with the same or different parameter values. If no cursor is open, this
    /// option has no effect for the application. `SQLCloseCursor` can also be called to close a
    /// cursor.
    SQL_CLOSE = 0,
    // SQL_DROP = 1, is deprecated in favour of SQLFreeHandle
    /// Sets the `SQL_DESC_COUNT` field of the ARD to 0, releasing all column buffers bound by
    /// `SQLBindCol` for the given StatementHandle. This does not unbind the bookmark column; to do
    /// that, the `SQL_DESC_DATA_PTR` field of the ARD for the bookmark column is set to NULL.
    /// Notice that if this operation is performed on an explicitly allocated descriptor that is
    /// shared by more than one statement, the operation will affect the bindings of all statements
    /// that share the descriptor.
    SQL_UNBIND = 2,
    /// Sets the `SQL_DESC_COUNT` field of the APD to 0, releasing all parameter buffers set by
    /// `SQLBindParameter` for the given StatementHandle. If this operation is performed on an
    /// explicitly allocated descriptor that is shared by more than one statement, this operation
    /// will affect the bindings of all the statements that share the descriptor.
    SQL_RESET_PARAMS = 3,
}

pub use FreeStmtOption::*;

/// SQL Data Types
#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlDataType {
    SQL_UNKNOWN_TYPE = 0,
    // also called SQL_VARIANT_TYPE since odbc 4.0
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
    #[cfg(feature = "odbc_version_4")] SQL_UDT = 17,
    #[cfg(feature = "odbc_version_4")] SQL_ROW = 19,
    #[cfg(feature = "odbc_version_4")] SQL_ARRAY = 50,
    #[cfg(feature = "odbc_version_4")] SQL_MULTISET = 55,

    // one-parameter shortcuts for date/time data types
    SQL_DATE = 91,
    SQL_TIME = 92,
    SQL_TIMESTAMP = 93,
    #[cfg(feature = "odbc_version_4")] SQL_TIME_WITH_TIMEZONE = 94,
    #[cfg(feature = "odbc_version_4")] SQL_TIMESTAMP_WITH_TIMEZONE = 95,

    //SQL extended datatypes:
    SQL_EXT_LONGVARCHAR = -1,
    SQL_EXT_BINARY = -2,
    SQL_EXT_VARBINARY = -3,
    SQL_EXT_LONGVARBINARY = -4,
    SQL_EXT_BIGINT = -5,
    SQL_EXT_TINYINT = -6,
    SQL_EXT_BIT = -7,
    SQL_EXT_WCHAR = -8,
    SQL_EXT_WVARCHAR = -9,
    SQL_EXT_WLONGVARCHAR = -10,
    SQL_EXT_GUID = -11,
}

pub use self::SqlDataType::*;

/// Represented in C headers as SQLSMALLINT
#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HandleType {
    SQL_HANDLE_ENV = 1,
    SQL_HANDLE_DBC = 2,
    SQL_HANDLE_STMT = 3,
    SQL_HANDLE_DESC = 4,
}

pub use self::HandleType::*;

/// Options for `SQLDriverConnect`
#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlDriverConnectOption {
    SQL_DRIVER_NOPROMPT = 0,
    SQL_DRIVER_COMPLETE = 1,
    SQL_DRIVER_PROMPT = 2,
    SQL_DRIVER_COMPLETE_REQUIRED = 3,
}

pub use self::SqlDriverConnectOption::*;

/// Statement attributes for `SQLSetStmtAttr`
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlStatementAttribute {
    SQL_ATTR_PARAM_BIND_TYPE = 18,
    SQL_ATTR_PARAMSET_SIZE = 22,
    SQL_ATTR_ROW_BIND_TYPE = 5,
    SQL_ATTR_ROW_ARRAY_SIZE = 27,
    SQL_ATTR_ROWS_FETCHED_PTR = 26,
}

pub use self::SqlStatementAttribute::*;

/// Connection attributes for `SQLSetConnectAttr`
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlConnectionAttribute {
    SQL_ATTR_ACCESS_MODE = 101,
    SQL_ATTR_AUTOCOMMIT = 102,
    SQL_ATTR_LOGIN_TIMEOUT = 103,
    SQL_ATTR_TRACE = 104,
    SQL_ATTR_TRACEFILE = 105,
    SQL_ATTR_TRANSLATE_LIB = 106,
    SQL_ATTR_TRANSLATE_OPTION = 107,
    SQL_ATTR_TXN_ISOLATION = 108,
    SQL_ATTR_CURRENT_CATALOG = 109,
    SQL_ATTR_ODBC_CURSORS = 110,
    SQL_ATTR_QUIET_MODE = 111,
    SQL_ATTR_PACKET_SIZE = 112,
    SQL_ATTR_CONNECTION_TIMEOUT = 113,
    SQL_ATTR_DISCONNECT_BEHAVIOR = 114,
    SQL_ATTR_ENLIST_IN_DTC = 1207,
    SQL_ATTR_ENLIST_IN_XA = 1208,
    SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE = 117,
}

/// `DiagIdentifier` for `SQLGetDiagField`
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlHeaderDiagnosticIdentifier {
    SQL_DIAG_RETURNCODE = 1,
    SQL_DIAG_NUMBER = 2,
    SQL_DIAG_ROW_COUNT = 3,
    SQL_DIAG_SQLSTATE = 4,
    SQL_DIAG_NATIVE = 5,
    SQL_DIAG_MESSAGE_TEXT = 6,
    SQL_DIAG_DYNAMIC_FUNCTION = 7,
    SQL_DIAG_CLASS_ORIGIN = 8,
    SQL_DIAG_SUBCLASS_ORIGIN = 9,
    SQL_DIAG_CONNECTION_NAME = 10,
    SQL_DIAG_SERVER_NAME = 11,
    SQL_DIAG_DYNAMIC_FUNCTION_CODE = 12,
    SQL_DIAG_CURSOR_ROW_COUNT = -1249,
    SQL_DIAG_ROW_NUMBER = -1248,
    SQL_DIAG_COLUMN_NUMBER = -1247,
}

#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlAsyncConnectionBehavior {
    SQL_ASYNC_DBC_ENABLE_ON = 1,
    SQL_ASYNC_DBC_ENABLE_OFF = 0,
}
pub use self::SqlAsyncConnectionBehavior::*;

impl Default for SqlAsyncConnectionBehavior {
    fn default() -> SqlAsyncConnectionBehavior {
        SqlAsyncConnectionBehavior::SQL_ASYNC_DBC_ENABLE_OFF
    }
}

#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlDynamicDiagnosticIdentifier {
    SQL_DIAG_ALTER_DOMAIN = 3,
    SQL_DIAG_ALTER_TABLE = 4,
    SQL_DIAG_CALL = 7,
    SQL_DIAG_CREATE_ASSERTION = 6,
    SQL_DIAG_CREATE_CHARACTER_SET = 8,
    SQL_DIAG_CREATE_COLLATION = 10,
    SQL_DIAG_CREATE_DOMAIN = 23,
    SQL_DIAG_CREATE_INDEX = -1,
    SQL_DIAG_CREATE_SCHEMA = 64,
    SQL_DIAG_CREATE_TABLE = 77,
    SQL_DIAG_CREATE_TRANSLATION = 79,
    SQL_DIAG_CREATE_VIEW = 84,
    SQL_DIAG_DELETE_WHERE = 19,
    SQL_DIAG_DROP_ASSERTION = 24,
    SQL_DIAG_DROP_CHARACTER_SET = 25,
    SQL_DIAG_DROP_COLLATION = 26,
    SQL_DIAG_DROP_DOMAIN = 27,
    SQL_DIAG_DROP_INDEX = -2,
    SQL_DIAG_DROP_SCHEMA = 31,
    SQL_DIAG_DROP_TABLE = 32,
    SQL_DIAG_DROP_TRANSLATION = 33,
    SQL_DIAG_DROP_VIEW = 36,
    SQL_DIAG_DYNAMIC_DELETE_CURSOR = 38,
    SQL_DIAG_DYNAMIC_UPDATE_CURSOR = 81,
    SQL_DIAG_GRANT = 48,
    SQL_DIAG_INSERT = 50,
    SQL_DIAG_REVOKE = 59,
    SQL_DIAG_SELECT_CURSOR = 85,
    SQL_DIAG_UNKNOWN_STATEMENT = 0,
    SQL_DIAG_UPDATE_WHERE = 82,
}

pub use self::SqlConnectionAttribute::*;

/// Completion types for `SQLEndTrans`
#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlCompletionType {
    SQL_COMMIT = 0,
    SQL_ROLLBACK = 1,
}

pub use self::SqlCompletionType::*;

#[cfg_attr(windows, link(name = "odbc32"))]
#[cfg_attr(not(windows), link(name = "odbc"))]
extern "system" {
    /// Allocates an environment, connection, statement, or descriptor handle.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`
    pub fn SQLAllocHandle(
        handle_type: HandleType,
        input_handle: SQLHANDLE,
        output_handle: *mut SQLHANDLE,
    ) -> SQLRETURN;

    /// Frees resources associated with a specific environment, connection, statement, or
    /// descriptor handle.
    ///
    /// If `SQL_ERRQR` is returned the handle is still valid.
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`
    pub fn SQLFreeHandle(handle_type: HandleType, handle: SQLHANDLE) -> SQLRETURN;

    /// Gets attributes that govern aspects of environments
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`
    pub fn SQLGetEnvAttr(
        environment_handle: SQLHENV,
        attribute: EnvironmentAttribute,
        value_ptr: SQLPOINTER,
        buffer_length: SQLINTEGER,
        string_length: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Sets attributes that govern aspects of environments
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`
    pub fn SQLSetEnvAttr(
        environment_handle: SQLHENV,
        attribute: EnvironmentAttribute,
        value: SQLPOINTER,
        string_length: SQLINTEGER,
    ) -> SQLRETURN;

    /// Closes the connection associated with a specific connection handle.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`
    pub fn SQLDisconnect(connection_handle: SQLHDBC) -> SQLRETURN;

    /// Return the current values of multiple fields of a diagnostic record that contains eror,
    /// warning, and status information.
    ///
    /// # Returns
    ///
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`
    pub fn SQLGetDiagRecW(
        handle_type: HandleType,
        handle: SQLHANDLE,
        record_rumber: SQLSMALLINT,
        state: *mut SQLWCHAR,
        native_error_ptr: *mut SQLINTEGER,
        message_text: *mut SQLWCHAR,
        buffer_length: SQLSMALLINT,
        text_length_ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the current value of a field of a record of the diagnostic data structure (associated with a specified handle) that contains error, warning, and status information.
    ///
    /// Note:
    /// `diag_identifier` is either `SqlHeaderDiagnosticIdentifier` or `SqlDynamicDiagnosticIdentifier`
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_NO_DATA`.
    pub fn SQLGetDiagFieldW(
        handle_type: HandleType,
        handle: SQLHANDLE,
        record_rumber: SQLSMALLINT,
        diag_identifier: SQLSMALLINT,
        diag_info_ptr: SQLPOINTER,
        buffer_length: SQLSMALLINT,
        string_length_ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Executes a preparable statement, using the current values of the parameter marker variables
    /// if any parameters exist in the statement. This is the fastest way to submit an SQL
    /// statement for one-time execution
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_NEED_DATA`, `SQL_STILL_EXECUTING`, `SQL_ERROR`
    /// , `SQL_NO_DATA`, `SQL_INVALID_HANDLE`, or `SQL_PARAM_DATA_AVAILABLE`.
    pub fn SQLExecDirectW(
        statement_handle: SQLHSTMT,
        statement_text: *const SQLWCHAR,
        text_length: SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the number of columns in a result set
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE` or
    /// `SQL_STILL_EXECUTING`
    pub fn SQLNumResultCols(
        statement_handle: SQLHSTMT,
        column_count_ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    // Can be used since odbc version 3.8 to stream results
    pub fn SQLGetData(
        statement_handle: SQLHSTMT,
        col_or_param_num: SQLUSMALLINT,
        target_type: SqlCDataType,
        target_value_ptr: SQLPOINTER,
        buffer_length: SQLLEN,
        str_len_or_ind_ptr: *mut SQLLEN,
    ) -> SQLRETURN;

    /// SQLFetch fetches the next rowset of data from the result set and returns data for all bound
    /// columns.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, `SQL_NO_DATA` or
    /// `SQL_STILL_EXECUTING`
    pub fn SQLFetch(statement_handle: SQLHSTMT) -> SQLRETURN;

    /// Returns general information about the driver and data source associated with a connection
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`
    pub fn SQLGetInfoW(
        connection_handle: SQLHDBC,
        info_type: InfoType,
        info_value_ptr: SQLPOINTER,
        buffer_length: SQLSMALLINT,
        string_length_ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// SQLConnect establishes connections to a driver and a data source. The connection handle
    /// references storage of all information about the connection to the data source, including
    /// status, transaction state, and error information.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or
    /// `SQL_STILL_EXECUTING`
    pub fn SQLConnectW(
        connection_handle: SQLHDBC,
        server_name: *const SQLWCHAR,
        name_length_1: SQLSMALLINT,
        user_name: *const SQLWCHAR,
        name_length_2: SQLSMALLINT,
        authentication: *const SQLWCHAR,
        name_length_3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of table, catalog, or schema names, and table types, stored in a specific
    /// data source. The driver returns the information as a result set
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or
    /// `SQL_STILL_EXECUTING`
    pub fn SQLTablesW(
        statement_handle: SQLHSTMT,
        catalog_name: *const SQLWCHAR,
        name_length_1: SQLSMALLINT,
        schema_name: *const SQLWCHAR,
        name_length_2: SQLSMALLINT,
        table_name: *const SQLWCHAR,
        name_length_3: SQLSMALLINT,
        table_type: *const SQLWCHAR,
        name_length_4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns information about a data source. This function is implemented only by the Driver
    /// Manager.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_NO_DATA`
    pub fn SQLDataSourcesW(
        environment_handle: SQLHENV,
        direction: FetchOrientation,
        server_name: *mut SQLWCHAR,
        buffer_length_1: SQLSMALLINT,
        name_length_1: *mut SQLSMALLINT,
        description: *mut SQLWCHAR,
        buffer_length_2: SQLSMALLINT,
        name_length_2: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// An alternative to `SQLConnect`. It supports data sources that require more connection
    /// information than the three arguments in `SQLConnect`, dialog boxes to prompt the user for
    /// all connection information, and data sources that are not defined in the system information
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, `SQL_NO_DATA`,
    /// or `SQL_STILL_EXECUTING`
    pub fn SQLDriverConnectW(
        connection_handle: SQLHDBC,
        window_handle: SQLHWND,
        in_connection_string: *const SQLWCHAR,
        string_length_1: SQLSMALLINT,
        out_connection_string: *mut SQLWCHAR,
        buffer_length: SQLSMALLINT,
        string_length_2: *mut SQLSMALLINT,
        driver_completion: SqlDriverConnectOption,
    ) -> SQLRETURN;

    /// Lists driver descriptions and driver attribute keywords. This function is implemented only
    /// by the Driver Manager.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_NO_DATA`
    pub fn SQLDriversW(
        henv: SQLHENV,
        direction: FetchOrientation,
        driver_desc: *mut SQLWCHAR,
        driver_desc_max: SQLSMALLINT,
        out_driver_desc: *mut SQLSMALLINT,
        driver_attributes: *mut SQLWCHAR,
        drvr_attr_max: SQLSMALLINT,
        out_drvr_attr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Closes a cursor that has been opened on a statement and discards pending results.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR` or `SQL_INVALID_HANDLE`
    pub fn SQLCloseCursor(hstmt: SQLHSTMT) -> SQLRETURN;

    /// Binds a buffer to a parameter marker in an SQL statement
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR` or `SQL_INVALID_HANDLE`
    pub fn SQLBindParameter(
        hstmt: SQLHSTMT,
        parameter_number: SQLUSMALLINT,
        input_output_type: InputOutput,
        value_type: SqlCDataType,
        parmeter_type: SqlDataType,
        column_size: SQLULEN,
        decimal_digits: SQLSMALLINT,
        parameter_value_ptr: SQLPOINTER,
        buffer_length: SQLLEN,
        str_len_or_ind_ptr: *mut SQLLEN,
    ) -> SQLRETURN;

    /// Performs bulk insertions and bulk bookmark operations, including update, delete, and fetch by bookmark.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_NEED_DATA`, `SQL_STILL_EXECUTING`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`.
    pub fn SQLBulkOperations(statement_handle: SQLHSTMT, operation: SqlBulkOperation) -> SQLRETURN;

    /// Cancels the processing on a statement.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR` or `SQL_INVALID_HANDLE`
    pub fn SQLCancel(statement_handle: SQLHSTMT) -> SQLRETURN;

    /// Cancels the processing on a connection or statement.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR` or `SQL_INVALID_HANDLE`
    pub fn SQLCancelHandle(handle_type: HandleType, handle: SQLHANDLE) -> SQLRETURN;

    /// Compiles the statement and generates an access plan.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or
    /// `SQL_STILL_EXECUTING`
    pub fn SQLPrepareW(
        hstmt: SQLHSTMT,
        statement_text: *const SQLWCHAR,
        text_length: SQLINTEGER,
    ) -> SQLRETURN;

    /// Executes a prepared statement, using the current values of the parameter marker variables
    /// if any paramater markers exis in the statement.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_NEED_DATA`, `SQL_STILL_EXECUTING`, `SQL_ERROR`
    /// , `SQL_NO_DATA`, `SQL_INVALID_HANDLE`, or `SQL_PARAM_DATA_AVAILABLE`.
    pub fn SQLExecute(hstmt: SQLHSTMT) -> SQLRETURN;

    /// Stops processing associated with a specific statement, closes any open cursors associated
    /// with the statement, discards pending results, or, optionally, frees all resources
    /// associated with the statement handle.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`.
    pub fn SQLFreeStmt(hstmt: SQLHSTMT, option: FreeStmtOption) -> SQLRETURN;

    /// Binds application data bufferst to columns in the result set.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`.
    pub fn SQLBindCol(
        hstmt: SQLHSTMT,
        col_number: SQLUSMALLINT,
        target_type: SqlCDataType,
        target_value: SQLPOINTER,
        buffer_length: SQLLEN,
        length_or_indicatior: *mut SQLLEN,
    ) -> SQLRETURN;

    /// SQLBrowseConnect supports an iterative method of discovering and enumerating the attributes
    /// and attribute values required to connect to a data source.
    /// Each call to SQLBrowseConnect returns successive levels of attributes and attribute values.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_NEED_DATA`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_STILL_EXECUTING`.
    pub fn SQLBrowseConnectW(
        connection_handle: SQLHDBC,
        in_connection_string: *const SQLWCHAR,
        string_length: SQLSMALLINT,
        out_connection_string: *mut SQLWCHAR,
        buffer_length: SQLSMALLINT,
        out_buffer_length: *mut SQLSMALLINT,
    ) -> SQLRETURN;


    /// Returns descriptor information for a column in a result set.
    /// Descriptor information is returned as a character string,
    /// a descriptor-dependent value, or an integer value.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_STILL_EXECUTING`.
    pub fn SQLColAttributeW(
        statement_handle: SQLHSTMT,
        column_number: SQLUSMALLINT,
        field_identifier: SQLUSMALLINT,
        character_attribute_ptr: SQLPOINTER,
        buffer_length: SQLSMALLINT,
        string_length_ptr: *mut SQLSMALLINT,
        numeric_attribute_ptr: *mut SQLLEN,
    ) -> SQLRETURN;

    /// Copies descriptor information from one descriptor handle to another.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_ERROR`, `SQL_NO_DATA`, or `SQL_INVALID_HANDLE`.
    pub fn SQLCopyDesc(
        source_desc_handle: SQLHDESC,
        target_desc_handle: SQLHDESC,
    ) -> SQLRETURN;

    /// Returns the current setting of a connection attribute.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_ERROR`, `SQL_NO_DATA`, or `SQL_INVALID_HANDLE`.
    pub fn SQLGetConnectAttrW(
        connection_handle: SQLHDBC,
        attribute: SQLINTEGER,
        value_ptr: SQLPOINTER,
        buffer_length: SQLINTEGER,
        string_length_ptr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the cursor name associated with a specified statement.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`.
    pub fn SQLGetCursorNameW(
        statement_handle: SQLHSTMT,
        cursor_name: *mut SQLWCHAR,
        buffer_length: SQLSMALLINT,
        name_length_ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the current setting or value of a single field of a descriptor record.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_NO_DATA`, or `SQL_INVALID_HANDLE`.
    /// `SQL_NO_DATA` is returned if RecNumber is greater than the current number of descriptor records.
    /// `SQL_NO_DATA` is returned if DescriptorHandle is an IRD handle and the statement is in the prepared or executed state but there was no open cursor associated with it.
    pub fn SQLGetDescFieldW(
        descriptor_handle: SQLHDESC,
        record_number: SQLSMALLINT,
        field_identifier: SQLSMALLINT,
        value_ptr: SQLPOINTER,
        buffer_length: SQLINTEGER,
        string_length_ptr: *mut SQLINTEGER,
    ) -> SQLRETURN;

    /// Returns the current settings or values of multiple fields of a descriptor record.
    /// The fields returned describe the name, data type, and storage of column or parameter data.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_NO_DATA`, or `SQL_INVALID_HANDLE`.
    /// `SQL_NO_DATA` is returned if RecNumber is greater than the current number of descriptor records.
    /// `SQL_NO_DATA` is returned if DescriptorHandle is an IRD handle and the statement is in the prepared or executed state but there was no open cursor associated with it.
    pub fn SQLGetDescRecW(
        descriptor_handle: SQLHDESC,
        record_number: SQLSMALLINT,
        name: *mut SQLWCHAR,
        buffer_length: SQLSMALLINT,
        string_length_ptr: *mut SQLSMALLINT,
        type_ptr: *mut SQLSMALLINT,
        sub_type_ptr: *mut SQLSMALLINT,
        length_ptr: *mut SQLLEN,
        precision_ptr: *mut SQLSMALLINT,
        scale_ptr: *mut SQLSMALLINT,
        nullable_ptr: *mut Nullable,
    ) -> SQLRETURN;

    /// Returns a list of columns and associated privileges for the specified table.
    /// The driver returns the information as a result set on the specified StatementHandle.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_STILL_EXECUTING`.
    pub fn SQLColumnPrivilegesW(
        statement_handle: SQLHSTMT,
        catalog_name: *const SQLWCHAR,
        catalog_name_length: SQLSMALLINT,
        schema_name: *const SQLWCHAR,
        schema_name_length: SQLSMALLINT,
        table_name: *const SQLWCHAR,
        table_name_length: SQLSMALLINT,
        column_name: *const SQLWCHAR,
        column_name_length: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of column names in specified tables.
    /// The driver returns this information as a result set on the specified StatementHandle.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_STILL_EXECUTING`.
    pub fn SQLColumnsW(
        statement_handle: SQLHSTMT,
        catalog_name: *const SQLWCHAR,
        catalog_name_length: SQLSMALLINT,
        schema_name: *const SQLWCHAR,
        schema_name_length: SQLSMALLINT,
        table_name: *const SQLWCHAR,
        table_name_length: SQLSMALLINT,
        column_name: *const SQLWCHAR,
        column_name_length: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Can be used to determine when an asynchronous function is complete using either notification- or polling-based processing.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_ERROR`, `SQL_NO_DATA`, or `SQL_INVALID_HANDLE`.
    #[cfg(feature = "odbc_version_3_80")]
    pub fn SQLCompleteAsync(
        handle_type: HandleType,
        handle: SQLHANDLE,
        async_ret_code_ptr: *mut RETCODE,
    ) -> SQLRETURN;

    /// Fetches the specified rowset of data from the result set and returns data for all bound columns.
    /// Rowsets can be specified at an absolute or relative position or by bookmark.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_STILL_EXECUTING`.
    pub fn SQLFetchScroll(
        statement_handle: SQLHSTMT,
        fetch_orientation: FetchOrientation,
        fetch_offset: SQLLEN,
    ) -> SQLRETURN;

    /// Can return:
    /// - A list of foreign keys in the specified table (columns in the specified table that refer to primary keys in other tables).
    /// - A list of foreign keys in other tables that refer to the primary key in the specified table.
    ///
    /// The driver returns each list as a result set on the specified statement.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_STILL_EXECUTING`.
    pub fn SQLForeignKeysW(
        statement_handle: SQLHSTMT,
        pk_catalog_name: *const SQLWCHAR,
        pk_catalog_name_length: SQLSMALLINT,
        pk_schema_name: *const SQLWCHAR,
        pk_schema_name_length: SQLSMALLINT,
        pk_table_name: *const SQLWCHAR,
        pk_table_name_length: SQLSMALLINT,
        fk_catalog_name: *const SQLWCHAR,
        fk_catalog_name_length: SQLSMALLINT,
        fk_schema_name: *const SQLWCHAR,
        fk_schema_name_length: SQLSMALLINT,
        fk_table_name: *const SQLWCHAR,
        fk_table_name_length: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the result descriptor for one column in the result set — column name, type, column
    /// size, decimal digits, and nullability.
    ///
    /// This information also is available in the fields of the IRD.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_STILL_EXECUTING`, `SQL_ERROR`, or
    /// `SQL_INVALID_HANDLE`.
    pub fn SQLDescribeColW(
        hstmt: SQLHSTMT,
        col_number: SQLUSMALLINT,
        col_name: *mut SQLWCHAR,
        buffer_length: SQLSMALLINT,
        name_length: *mut SQLSMALLINT,
        data_type: *mut SqlDataType,
        col_size: *mut SQLULEN,
        decimal_digits: *mut SQLSMALLINT,
        nullable: *mut Nullable,
    ) -> SQLRETURN;

    /// Returns the description of a parameter marker associated with a prepared SQL statement.
    /// This information is also available in the fields of the IPD.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_STILL_EXECUTING`, `SQL_ERROR`, or
    /// `SQL_INVALID_HANDLE`.
    pub fn SQLDescribeParam(
        statement_handle: SQLHSTMT,
        parameter_number: SQLUSMALLINT,
        data_type_ptr: *mut SqlDataType,
        parameter_size_ptr: *mut SQLULEN,
        decimal_digits_ptr: *mut SQLSMALLINT,
        nullable_ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Sets attributes related to a statement.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`.
    pub fn SQLSetStmtAttrW(
        hstmt: SQLHSTMT,
        attr: SqlStatementAttribute,
        value: SQLPOINTER,
        str_length: SQLINTEGER,
    ) -> SQLRETURN;

    /// Sets attributes that govern aspects of connections.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_STILL_EXECUTING`.
    pub fn SQLSetConnectAttrW(
        hdbc: SQLHDBC,
        attr: SqlConnectionAttribute,
        value: SQLPOINTER,
        str_length: SQLINTEGER,
    ) -> SQLRETURN;

    /// Requests a commit or rollback operation for all active operations on all statements associated with a handle.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_STILL_EXECUTING`.
    pub fn SQLEndTran(
        handle_type: HandleType,
        handle: SQLHANDLE,
        completion_type: SqlCompletionType,
    ) -> SQLRETURN;
}
