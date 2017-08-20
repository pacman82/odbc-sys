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
use std::os::raw::c_void;

mod sqlreturn;
mod info_type;
mod fetch_orientation;
mod attributes;
mod c_data_type;
mod input_output;
mod nullable;

//These types can never be instantiated in Rust code.
pub enum Obj {}
pub enum Env {}
pub enum Dbc {}
pub enum Stmt {}

pub type SQLHANDLE = *mut Obj;
pub type SQLHENV = *mut Env;

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

#[cfg(target_pointer_width = "64")]
pub type SQLLEN = i64;
#[cfg(target_pointer_width = "32")]
pub type SQLLEN = SQLINTEGER;

#[cfg(target_pointer_width = "64")]
pub type SQLULEN = u64;
#[cfg(target_pointer_width = "32")]
pub type SQLULEN = SQLUINTEGER;

pub type SQLHWND = SQLPOINTER;

// flags for null-terminated string
pub const SQL_NTS: SQLSMALLINT = -3;
pub const SQL_NTSL: SQLINTEGER = -3;

/// Maximum message length
pub const SQL_MAX_MESSAGE_LENGTH: SQLSMALLINT = 512;
pub const SQL_SQLSTATE_SIZE: usize = 5;

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

    //SQL extended datatypes:
    SQL_EXT_LONGVARCHAR = -1,
    SQL_EXT_BINARY = -2,
    SQL_EXT_VARBINARY = -3,
    SQL_EXT_LONGVARBINARY = -4,
    SQL_EXT_BIGINT = -5,
    SQL_EXT_TINYINT = -6,
    SQL_EXT_BIT = -7,
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

#[cfg_attr(windows, link(name = "odbc32"))]
#[cfg_attr(not(windows), link(name = "odbc"))]
extern "C" {
    /// Allocates an environment, connection, statement, or descriptor handle.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`
    pub fn SQLAllocHandle(
        handle_type: HandleType,
        input_handle: SQLHANDLE,
        output_Handle: *mut SQLHANDLE,
    ) -> SQLRETURN;

    /// Frees resources associated with a specific environment, connection, statement, or
    /// descriptor handle.
    ///
    /// If `SQL_ERRQR` is returned the handle is still valid.
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_ERROR`, or `SQL_INVALID_HANDLE`
    pub fn SQLFreeHandle(handle_type: HandleType, handle: SQLHANDLE) -> SQLRETURN;

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
    pub fn SQLGetDiagRec(
        handle_type: HandleType,
        handle: SQLHANDLE,
        RecNumber: SQLSMALLINT,
        state: *mut SQLCHAR,
        native_error_ptr: *mut SQLINTEGER,
        message_text: *mut SQLCHAR,
        buffer_length: SQLSMALLINT,
        text_length_ptr: *mut SQLSMALLINT,
    ) -> SQLRETURN;

    /// Executes a preparable statement, using the current values of the parameter marker variables
    /// if any parameters exist in the statement. This is the fastest way to submit an SQL
    /// statement for one-time execution
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_NEED_DATA`, `SQL_STILL_EXECUTING`, `SQL_ERROR`
    /// , `SQL_NO_DATA`, `SQL_INVALID_HANDLE`, or `SQL_PARAM_DATA_AVAILABLE`.
    pub fn SQLExecDirect(
        statement_handle: SQLHSTMT,
        statement_text: *const SQLCHAR,
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
    pub fn SQLGetInfo(
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
    pub fn SQLConnect(
        connection_handle: SQLHDBC,
        server_name: *const SQLCHAR,
        name_length_1: SQLSMALLINT,
        user_name: *const SQLCHAR,
        name_length_2: SQLSMALLINT,
        authentication: *const SQLCHAR,
        name_length_3: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns the list of table, catalog, or schema names, and table types, stored in a specific
    /// data source. The driver returns the information as a result set
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or
    /// `SQL_STILL_EXECUTING`
    pub fn SQLTables(
        statement_handle: SQLHSTMT,
        catalog_name: *const SQLCHAR,
        name_length_1: SQLSMALLINT,
        schema_name: *const SQLCHAR,
        name_length_2: SQLSMALLINT,
        table_name: *const SQLCHAR,
        name_length_3: SQLSMALLINT,
        TableType: *const SQLCHAR,
        name_length_4: SQLSMALLINT,
    ) -> SQLRETURN;

    /// Returns information about a data source. This function is implemented only by the Driver
    /// Manager.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_NO_DATA`
    pub fn SQLDataSources(
        environment_handle: SQLHENV,
        direction: FetchOrientation,
        server_name: *mut SQLCHAR,
        buffer_length_1: SQLSMALLINT,
        name_length_1: *mut SQLSMALLINT,
        description: *mut SQLCHAR,
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
    pub fn SQLDriverConnect(
        connection_handle: SQLHDBC,
        window_handle: SQLHWND,
        in_connection_string: *const SQLCHAR,
        string_length_1: SQLSMALLINT,
        out_connection_string: *mut SQLCHAR,
        buffer_length: SQLSMALLINT,
        string_length_2: *mut SQLSMALLINT,
        DriverCompletion: SqlDriverConnectOption,
    ) -> SQLRETURN;

    /// Lists driver descriptions and driver attribute keywords. This function is implemented only
    /// by the Driver Manager.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or `SQL_NO_DATA`
    pub fn SQLDrivers(
        henv: SQLHENV,
        direction: FetchOrientation,
        driver_desc: *mut SQLCHAR,
        driver_desc_max: SQLSMALLINT,
        out_driver_desc: *mut SQLSMALLINT,
        driver_attributes: *mut SQLCHAR,
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

    /// Compiles the statement and generates an access plan.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_ERROR`, `SQL_INVALID_HANDLE`, or
    /// `SQL_STILL_EXECUTING`
    pub fn SQLPrepare(
        hstmt: SQLHSTMT,
        statement_text: *const SQLCHAR,
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

    /// Returns the result descriptor for one column in the result set — column name, type, column
    /// size, decimal digits, and nullability.
    ///
    /// This information also is available in the fields of the IRD.
    ///
    /// # Returns
    /// `SQL_SUCCESS`, `SQL_SUCCESS_WITH_INFO`, `SQL_STILL_EXECUTING`, `SQL_ERROR`, or
    /// `SQL_INVALID_HANDLE`.
    pub fn SQLDescribeCol(
        hstmt: SQLHSTMT,
        col_number: SQLUSMALLINT,
        col_name: *mut SQLCHAR,
        buffer_length: SQLSMALLINT,
        name_length: *mut SQLSMALLINT,
        data_type: *mut SqlDataType,
        col_size: *mut SQLULEN,
        decimal_digits: *mut SQLSMALLINT,
        nullable: *mut Nullable,
    ) -> SQLRETURN;
}
