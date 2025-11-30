//! ODBC types those representation is compatible with the ODBC C API.
//!
//! This layer has not been created using automatic code generation. It is incomplete, i.e. it does
//! not contain every symbol or constant defined in the ODBC C headers. Symbols which are
//! deprecated since ODBC 3 have been left out intentionally. While some extra type safety has been
//! added by grouping some of C's `#define` constants into `enum`-types it mostly offers the same
//! power (all) and safety guarantess(none) as the wrapped C-API.
//! ODBC 4.0 is still under development by Microsoft, so these symbols are deactivated by default
//! in the cargo.toml

mod attributes;
mod bulk_operation;
mod c_data_type;
mod connection_attribute;
mod desc;
mod fetch_orientation;
mod functions;
mod handles;
mod indicator;
mod info_type;
mod interval;
mod nullability;
mod param_type;
mod set_pos;
mod sql_data_type;
mod sqlreturn;

pub use self::{
    attributes::{
        AttrConnectionPooling, AttrCpMatch, AttrOdbcVersion, EnvironmentAttribute,
        StatementAttribute,
    },
    bulk_operation::BulkOperation,
    c_data_type::*,
    connection_attribute::ConnectionAttribute,
    desc::Desc,
    fetch_orientation::FetchOrientation,
    functions::*,
    handles::{HDbc, HDesc, HEnv, HStmt, Handle},
    indicator::{len_data_at_exec, DATA_AT_EXEC, NO_TOTAL, NULL_DATA},
    info_type::InfoType,
    interval::Interval,
    nullability::Nullability,
    param_type::ParamType,
    set_pos::{Lock, Operation},
    sql_data_type::SqlDataType,
    sqlreturn::SqlReturn,
};
use std::os::raw::{c_int, c_void};

pub type USmallInt = u16;
pub type Pointer = *mut c_void;
pub type Char = u8;
pub type SChar = i8;
pub type WChar = u16;

pub type Len = isize;
pub type ULen = usize;
pub type HWnd = Pointer;
pub type RetCode = i16;

/// Row index parameter for [`crate::SQLSetPos`]
#[cfg(target_pointer_width = "64")]
pub type SetPosIRow = u64;
/// Row index parameter for [`crate::SQLSetPos`]
#[cfg(not(target_pointer_width = "64"))]
pub type SetPosIRow = i16;

// flags for null-terminated string
pub const NTS: isize = -3;
pub const NTSL: isize = -3;

/// Maximum message length
pub const MAX_MESSAGE_LENGTH: i16 = 512;
pub const SQLSTATE_SIZE: usize = 5;
pub const SQLSTATE_SIZEW: usize = 10;

/// SQL Free Statement options
#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FreeStmtOption {
    /// Closes the cursor associated with StatementHandle (if one was defined) and discards all
    /// pending results. The application can reopen this cursor later by executing a SELECT
    /// statement again with the same or different parameter values. If no cursor is open, this
    /// option has no effect for the application. `SQLCloseCursor` can also be called to close a
    /// cursor.
    Close = 0,
    // SQL_DROP = 1, is deprecated in favour of SQLFreeHandle
    /// Sets the `SQL_DESC_COUNT` field of the ARD to 0, releasing all column buffers bound by
    /// `SQLBindCol` for the given StatementHandle. This does not unbind the bookmark column; to do
    /// that, the `SQL_DESC_DATA_PTR` field of the ARD for the bookmark column is set to NULL.
    /// Notice that if this operation is performed on an explicitly allocated descriptor that is
    /// shared by more than one statement, the operation will affect the bindings of all statements
    /// that share the descriptor.
    Unbind = 2,
    /// Sets the `SQL_DESC_COUNT` field of the APD to 0, releasing all parameter buffers set by
    /// `SQLBindParameter` for the given StatementHandle. If this operation is performed on an
    /// explicitly allocated descriptor that is shared by more than one statement, this operation
    /// will affect the bindings of all the statements that share the descriptor.
    ResetParams = 3,
}

/// Represented in C headers as SQLSMALLINT
#[repr(i16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HandleType {
    Env = 1,
    Dbc = 2,
    Stmt = 3,
    Desc = 4,
    // Only used between Drivers and Driver Manager to enable connection pooling.
    // https://learn.microsoft.com/en-us/sql/odbc/reference/develop-driver/developing-connection-pool-awareness-in-an-odbc-driver?view=sql-server-ver16
    // Defined in sqlspi.h
    DbcInfoToken = 6,
}

/// Options for `SQLDriverConnect`
#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DriverConnectOption {
    NoPrompt = 0,
    Complete = 1,
    Prompt = 2,
    CompleteRequired = 3,
}

// Attribute for string lengths

/// SQL_IS_POINTER
pub const IS_POINTER: i32 = -4;
/// SQL_IS_UINTEGER
pub const IS_UINTEGER: i32 = -5;
/// SQL_IS_INTEGER
pub const IS_INTEGER: i32 = -6;
/// SQL_IS_USMALLINT
pub const IS_USMALLINT: i32 = -7;
/// SQL_IS_SMALLINT
pub const IS_SMALLINT: i32 = -8;

/// SQL_YEAR_MONTH_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct YearMonth {
    pub year: u32,
    pub month: u32,
}

/// SQL_DAY_SECOND_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct DaySecond {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub fraction: u32,
}

/// SQL_INTERVAL_UNION
#[repr(C)]
#[derive(Copy, Clone)]
pub union IntervalUnion {
    pub year_month: YearMonth,
    pub day_second: DaySecond,
}

/// SQL_INTERVAL_STRUCT
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IntervalStruct {
    pub interval_type: c_int,
    pub interval_sign: i16,
    pub interval_value: IntervalUnion,
}

/// SQL_DATE_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Date {
    pub year: i16,
    pub month: USmallInt,
    pub day: USmallInt,
}

/// SQL_TIME_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Time {
    pub hour: USmallInt,
    pub minute: USmallInt,
    pub second: USmallInt,
}

/// SQL_TIMESTAMP_STRUCT
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Timestamp {
    pub year: i16,
    pub month: USmallInt,
    pub day: USmallInt,
    pub hour: USmallInt,
    pub minute: USmallInt,
    pub second: USmallInt,
    pub fraction: u32,
}

/// SQLGUID
#[repr(C)]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Guid {
    pub d1: u32,
    pub d2: u16,
    pub d3: u16,
    pub d4: [u8; 8],
}

/// `DiagIdentifier` for `SQLGetDiagField`
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HeaderDiagnosticIdentifier {
    /// SQL_DIAG_RETURNCODE
    ReturnCode = 1,
    /// SQL_DIAG_NUMBER
    Number = 2,
    /// SQL_DIAG_ROW_COUNT
    RowCount = 3,
    /// SQL_DIAG_SQLSTATE
    SqlState = 4,
    /// SQL_DIAG_NATIVE
    Native = 5,
    /// SQL_DIAG_MESSAGE_TEXT
    MessageText = 6,
    /// SQL_DIAG_DYNAMIC_FUNCTION
    DynamicFunction = 7,
    /// SQL_DIAG_CLASS_ORIGIN
    ClassOrigin = 8,
    /// SQL_DIAG_SUBCLASS_ORIGIN
    SubclassOrigin = 9,
    /// SQL_DIAG_CONNECTION_NAME
    ConnectionName = 10,
    /// SQL_DIAG_SERVER_NAME
    ServerName = 11,
    /// SQL_DIAG_DYNAMIC_FUNCTION_CODE
    DynamicFunctionCode = 12,
    /// SQL_DIAG_CURSOR_ROW_COUNT
    CursorRowCount = -1249,
    /// SQL_DIAG_ROW_NUMBER
    RowNumber = -1248,
    /// SQL_DIAG_COLUMN_NUMBER
    ColumnNumber = -1247,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum AsyncConnectionBehavior {
    /// SQL_ASYNC_DBC_ENABLE_ON
    On = 1,
    /// SQL_ASYNC_DBC_ENABLE_OFF = 0
    #[default]
    Off = 0,
}

#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DynamicDiagnosticIdentifier {
    /// SQL_DIAG_ALTER_DOMAIN
    AlterDomain = 3,
    /// SQL_DIAG_ALTER_TABLE,
    AlterTable = 4,
    /// SQL_DIAG_CALL
    Call = 7,
    /// SQL_DIAG_CREATE_ASSERTION
    CreateAssertion = 6,
    /// SQL_DIAG_CREATE_CHARACTER_SET
    CreateCharacterSet = 8,
    /// SQL_DIAG_CREATE_COLLATION,
    CreateCollation = 10,
    /// SQL_DIAG_CREATE_DOMAIN
    CreateDomain = 23,
    /// SQL_DIAG_CREATE_INDEX
    CreateIndex = -1,
    /// SQL_DIAG_CREATE_SCHEMA
    CreateSchema = 64,
    /// SQL_DIAG_CREATE_TABLE
    CreateTable = 77,
    /// SQL_DIAG_CREATE_TRANSLATION
    CreateTranslation = 79,
    /// SQL_DIAG_CREATE_VIEW
    CreateView = 84,
    /// SQL_DIAG_DELETE_WHERE
    DeleteWhere = 19,
    /// SQL_DIAG_DROP_ASSERTION
    DropAssertion = 24,
    /// SQL_DIAG_DROP_CHARACTER_SET
    DropCharacterSet = 25,
    /// SQL_DIAG_DROP_COLLATION
    DropCollation = 26,
    /// SQL_DIAG_DROP_DOMAIN
    DropDomain = 27,
    /// SQL_DIAG_DROP_INDEX
    DropIndex = -2,
    /// SQL_DIAG_DROP_SCHEMA
    DropSchema = 31,
    /// SQL_DIAG_DROP_TABLE
    DropTable = 32,
    /// SQL_DIAG_DROP_TRANSLATION
    DropTranslation = 33,
    /// SQL_DIAG_DROP_VIEW
    DropView = 36,
    /// SQL_DIAG_DYNAMIC_DELETE_CURSOR
    DynamicDeleteCursor = 38,
    /// SQL_DIAG_DYNAMIC_UPDATE_CURSOR
    DynamicUpdateCursor = 81,
    /// SQL_DIAG_GRANT
    Grant = 48,
    /// SQL_DIAG_INSERT
    Insert = 50,
    /// SQL_DIAG_REVOKE
    Revoke = 59,
    // SQL_DIAG_SELECT_CURSOR
    SelectCursor = 85,
    /// SQL_DIAG_UNKNOWN_STATEMENT = 0,
    UnknownStatement = 0,
    /// SQL_DIAG_UPDATE_WHERE = 82,
    UpdateWhere = 82,
}

/// Completion types for `SQLEndTrans`
#[repr(i16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CompletionType {
    Commit = 0,
    Rollback = 1,
}

/// This means we use 128 bits (16 bytes) to for the numeric value. This excludes the sign, which is
/// stored separetly.
pub const MAX_NUMERIC_LEN: usize = 16;

/// Equivalent of `SQL_NUMERIC_STRUCT` in the ODBC C API.
///
/// # Examples
///
/// ## Store value in Numeric
///
/// ```
/// use odbc_sys::Numeric;
///
/// /// Store 123.45 in `Numeric`
/// let mut num = Numeric {
///     precision: 5, // 123.45 uses five digits
///     scale: 2,   // two digits after the decimal point
///     sign: 1,    // positive number
///     val: 12345u128.to_le_bytes(), // store 12345 as little-endian bytes
/// };
/// ```
///
/// See:
/// <https://learn.microsoft.com/en-us/sql/odbc/reference/appendixes/retrieve-numeric-data-sql-numeric-struct-kb222831>
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Numeric {
    /// Number of significant digits. `precision` and `scale` are ignored then using `Numeric` as
    /// input.
    pub precision: u8,
    /// Number of decimal digits to the right of the decimal point. `precision` and `scale` are
    /// ignored then using `Numeric` as input.
    pub scale: i8,
    /// 1 if positive, 0 if negative
    pub sign: u8,
    /// The value of the numeric as a little-endian array of bytes. The value is stored as an
    /// unsigned integer without any decimal point. For example, the number -123.45 with
    /// precision 5 and scale 2 is stored as 12345 (0x39 0x30 0x00 0x00 ...).
    pub val: [u8; MAX_NUMERIC_LEN],
}
