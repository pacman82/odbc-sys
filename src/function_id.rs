/// ODBC function identifiers used with `SQLGetFunctions`.
///
/// These are the `SQL_API_*` constants defined in `sql.h` and `sqlext.h`.
/// Applications and Driver Managers use them to query whether a driver supports
/// a specific ODBC function.
///
/// See: <https://learn.microsoft.com/en-us/sql/odbc/reference/syntax/sqlgetfunctions-function>
#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FunctionId {
    // -----------------------------------------------------------------------
    // Core API (sql.h)
    // -----------------------------------------------------------------------
    /// `SQL_API_SQLALLOCCONNECT` (ODBC 2.x, deprecated in 3.x — use `AllocHandle`).
    AllocConnect = 1,
    /// `SQL_API_SQLALLOCENV` (ODBC 2.x, deprecated in 3.x — use `AllocHandle`).
    AllocEnv = 2,
    /// `SQL_API_SQLALLOCSTMT` (ODBC 2.x, deprecated in 3.x — use `AllocHandle`).
    AllocStmt = 3,
    /// `SQL_API_SQLBINDCOL`.
    BindCol = 4,
    /// `SQL_API_SQLCANCEL`.
    Cancel = 5,
    /// `SQL_API_SQLCOLATTRIBUTE`.
    ColAttribute = 6,
    /// `SQL_API_SQLCONNECT`.
    Connect = 7,
    /// `SQL_API_SQLDESCRIBECOL`.
    DescribeCol = 8,
    /// `SQL_API_SQLDISCONNECT`.
    Disconnect = 9,
    /// `SQL_API_SQLERROR` (ODBC 2.x, deprecated in 3.x — use `GetDiagRec`).
    Error = 10,
    /// `SQL_API_SQLEXECDIRECT`.
    ExecDirect = 11,
    /// `SQL_API_SQLEXECUTE`.
    Execute = 12,
    /// `SQL_API_SQLFETCH`.
    Fetch = 13,
    /// `SQL_API_SQLFREECONNECT` (ODBC 2.x, deprecated in 3.x — use `FreeHandle`).
    FreeConnect = 14,
    /// `SQL_API_SQLFREEENV` (ODBC 2.x, deprecated in 3.x — use `FreeHandle`).
    FreeEnv = 15,
    /// `SQL_API_SQLFREESTMT`.
    FreeStmt = 16,
    /// `SQL_API_SQLGETCURSORNAME`.
    GetCursorName = 17,
    /// `SQL_API_SQLNUMRESULTCOLS`.
    NumResultCols = 18,
    /// `SQL_API_SQLPREPARE`.
    Prepare = 19,
    /// `SQL_API_SQLROWCOUNT`.
    RowCount = 20,
    /// `SQL_API_SQLSETCURSORNAME`.
    SetCursorName = 21,
    /// `SQL_API_SQLSETPARAM` (ODBC 2.x, deprecated — use `BindParameter`).
    SetParam = 22,
    /// `SQL_API_SQLTRANSACT` (ODBC 2.x, deprecated in 3.x — use `EndTran`).
    Transact = 23,

    // -----------------------------------------------------------------------
    // Extended API (sqlext.h)
    // -----------------------------------------------------------------------
    /// `SQL_API_SQLBULKOPERATIONS`.
    BulkOperations = 24,
    /// `SQL_API_SQLCOLUMNS`.
    Columns = 40,
    /// `SQL_API_SQLDRIVERCONNECT`.
    DriverConnect = 41,
    /// `SQL_API_SQLGETCONNECTOPTION` (ODBC 2.x, deprecated — use `GetConnectAttr`).
    GetConnectOption = 42,
    /// `SQL_API_SQLGETDATA`.
    GetData = 43,
    /// `SQL_API_SQLGETFUNCTIONS`.
    GetFunctions = 44,
    /// `SQL_API_SQLGETINFO`.
    GetInfo = 45,
    /// `SQL_API_SQLGETSTMTOPTION` (ODBC 2.x, deprecated — use `GetStmtAttr`).
    GetStmtOption = 46,
    /// `SQL_API_SQLGETTYPEINFO`.
    GetTypeInfo = 47,
    /// `SQL_API_SQLPARAMDATA`.
    ParamData = 48,
    /// `SQL_API_SQLPUTDATA`.
    PutData = 49,
    /// `SQL_API_SQLSETCONNECTOPTION` (ODBC 2.x, deprecated — use `SetConnectAttr`).
    SetConnectOption = 50,
    /// `SQL_API_SQLSETSTMTOPTION` (ODBC 2.x, deprecated — use `SetStmtAttr`).
    SetStmtOption = 51,
    /// `SQL_API_SQLSPECIALCOLUMNS`.
    SpecialColumns = 52,
    /// `SQL_API_SQLSTATISTICS`.
    Statistics = 53,
    /// `SQL_API_SQLTABLES`.
    Tables = 54,
    /// `SQL_API_SQLBROWSECONNECT`.
    BrowseConnect = 55,
    /// `SQL_API_SQLCOLUMNPRIVILEGES`.
    ColumnPrivileges = 56,
    /// `SQL_API_SQLDATASOURCES`.
    DataSources = 57,
    /// `SQL_API_SQLDESCRIBEPARAM`.
    DescribeParam = 58,
    /// `SQL_API_SQLEXTENDEDFETCH` (ODBC 2.x, deprecated — use `FetchScroll`).
    ExtendedFetch = 59,
    /// `SQL_API_SQLFOREIGNKEYS`.
    ForeignKeys = 60,
    /// `SQL_API_SQLMORERESULTS`.
    MoreResults = 61,
    /// `SQL_API_SQLNATIVESQL`.
    NativeSql = 62,
    /// `SQL_API_SQLNUMPARAMS`.
    NumParams = 63,
    /// `SQL_API_SQLPARAMOPTIONS` (ODBC 2.x, deprecated).
    ParamOptions = 64,
    /// `SQL_API_SQLPRIMARYKEYS`.
    PrimaryKeys = 65,
    /// `SQL_API_SQLPROCEDURECOLUMNS`.
    ProcedureColumns = 66,
    /// `SQL_API_SQLPROCEDURES`.
    Procedures = 67,
    /// `SQL_API_SQLSETPOS`.
    SetPos = 68,
    /// `SQL_API_SQLSETSCROLLOPTIONS` (ODBC 2.x, deprecated).
    SetScrollOptions = 69,
    /// `SQL_API_SQLTABLEPRIVILEGES`.
    TablePrivileges = 70,
    /// `SQL_API_SQLDRIVERS`.
    Drivers = 71,
    /// `SQL_API_SQLBINDPARAMETER`.
    BindParameter = 72,
    /// `SQL_API_SQLALLOCHANDLESTD`.
    AllocHandleStd = 73,

    // -----------------------------------------------------------------------
    // ODBC 3.x API (sql.h, IDs >= 1000)
    // -----------------------------------------------------------------------
    /// `SQL_API_SQLALLOCHANDLE`.
    AllocHandle = 1001,
    /// `SQL_API_SQLBINDPARAM`.
    BindParam = 1002,
    /// `SQL_API_SQLCLOSECURSOR`.
    CloseCursor = 1003,
    /// `SQL_API_SQLCOPYDESC`.
    CopyDesc = 1004,
    /// `SQL_API_SQLENDTRAN`.
    EndTran = 1005,
    /// `SQL_API_SQLFREEHANDLE`.
    FreeHandle = 1006,
    /// `SQL_API_SQLGETCONNECTATTR`.
    GetConnectAttr = 1007,
    /// `SQL_API_SQLGETDESCFIELD`.
    GetDescField = 1008,
    /// `SQL_API_SQLGETDESCREC`.
    GetDescRec = 1009,
    /// `SQL_API_SQLGETDIAGFIELD`.
    GetDiagField = 1010,
    /// `SQL_API_SQLGETDIAGREC`.
    GetDiagRec = 1011,
    /// `SQL_API_SQLGETENVATTR`.
    GetEnvAttr = 1012,
    /// `SQL_API_SQLGETSTMTATTR`.
    GetStmtAttr = 1014,
    /// `SQL_API_SQLSETCONNECTATTR`.
    SetConnectAttr = 1016,
    /// `SQL_API_SQLSETDESCFIELD`.
    SetDescField = 1017,
    /// `SQL_API_SQLSETDESCREC`.
    SetDescRec = 1018,
    /// `SQL_API_SQLSETENVATTR`.
    SetEnvAttr = 1019,
    /// `SQL_API_SQLSETSTMTATTR`.
    SetStmtAttr = 1020,
    /// `SQL_API_SQLFETCHSCROLL`.
    FetchScroll = 1021,
    /// `SQL_API_SQLCANCELHANDLE` (ODBC 3.8).
    CancelHandle = 1022,
}

/// Special value for `SQLGetFunctions` requesting the ODBC 3.x bitmap of all
/// supported functions.
pub const SQL_API_ODBC3_ALL_FUNCTIONS: u16 = 999;

/// Size of the bitmap array (in `u16` elements) returned by `SQLGetFunctions`
/// when called with [`SQL_API_ODBC3_ALL_FUNCTIONS`].
pub const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: usize = 250;

/// Special value for `SQLGetFunctions` requesting the ODBC 2.x array of all
/// supported functions.
pub const SQL_API_ALL_FUNCTIONS: u16 = 0;
