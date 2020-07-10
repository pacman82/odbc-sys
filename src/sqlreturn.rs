use crate::SQLSMALLINT;
use std::convert::TryFrom;

pub use self::SqlReturn::*;

/// Indicates the overall success or failure of a function
///
/// NOT INTENDED TO BE USED DIRECTLY:
/// In their application users are greatly encouraged to use `SqlReturn` enum which provides better
/// safety guarantees by implementing `TryFrom<SQLRETURN>` and only convert to/from `SQLRETURN` when
/// interfacing API which consumes `SQLRETURN` arguments
pub type SQLRETURN = SQLSMALLINT;

/// Indicates the overall success or failure of a function
///
/// Each function in ODBC returns a code, known as its return code, which indicates the overall
/// success or failure of the function. Program logic is generally based on return codes.
/// See [ODBC reference][1]
/// [1]: https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/return-codes-odbc
#[must_use]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlReturn {
    /// Function failed due to an invalid environment, connection, statement, or descriptor handle
    ///
    /// This indicates a programming error. No additional information is available from
    /// `SQLGetDiagRec` or `SQLGetDiagField`. This code is returned only when the handle is a null
    /// pointer or is the wrong type, such as when a statement handle is passed for an argument a
    /// connection handle.
    SQL_INVALID_HANDLE = -2,

    /// Function failed
    ///
    /// The application calls `SQLGetDiagRec` or `SQLGetDiagField` to retrieve additional
    /// information. The contents of any output arguments to the function are undefined.
    SQL_ERROR = -1,

    /// Function completed successfully
    ///
    /// The application calls `SQLGetDiagField` to retrieve additional information from the header
    /// record.
    SQL_SUCCESS = 0,

    /// Function completed successfully, possibly with a nonfatal error (warning)
    ///
    /// The application calls `SQLGetDiagRec` or `SQLGetDiagField` to retrieve additional
    /// information.
    SQL_SUCCESS_WITH_INFO = 1,

    /// A function that was started asynchronously is still executing
    ///
    /// The application `SQLGetDiagRec` or `SQLGetDiagField` to retrieve additional information if
    /// any.
    SQL_STILL_EXECUTING = 2,

    /// More data is needed
    ///
    /// ,such as when a parameter data is sent at execution time or additional connection
    /// information is required. The application calls `SQLGetDiagRec` or `SQLGetDiagField` to
    /// retrieve additional information, if any.
    SQL_NEED_DATA = 99,

    /// No more data was available
    ///
    /// The application calls `SQLGetDiagRec` or `SQLGetDiagField` to retrieve additional
    /// information. One or more driver-defined status records in class 02xxx may be returned.
    SQL_NO_DATA = 100,

    #[cfg(feature = "odbc_version_3_80")]
    SQL_PARAM_DATA_AVAILABLE = 101,
}

impl TryFrom<SQLRETURN> for SqlReturn {
    type Error = SQLRETURN;

    fn try_from(source: SQLRETURN) -> Result<Self, Self::Error> {
        match source {
            x if x == SQL_INVALID_HANDLE as SQLRETURN => Ok(SQL_INVALID_HANDLE),
            x if x == SQL_ERROR as SQLRETURN => Ok(SQL_ERROR),
            x if x == SQL_SUCCESS as SQLRETURN => Ok(SQL_SUCCESS),
            x if x == SQL_SUCCESS_WITH_INFO as SQLRETURN => Ok(SQL_SUCCESS_WITH_INFO),
            x if x == SQL_STILL_EXECUTING as SQLRETURN => Ok(SQL_STILL_EXECUTING),
            x if x == SQL_NEED_DATA as SQLRETURN => Ok(SQL_NEED_DATA),
            x if x == SQL_NO_DATA as SQLRETURN => Ok(SQL_NO_DATA),

            #[cfg(feature = "odbc_version_3_80")]
            x if x == SQL_PARAM_DATA_AVAILABLE as SQLRETURN => Ok(SQL_PARAM_DATA_AVAILABLE),

            // ODBC driver returned value unspecified by the ODBC standard
            unknown => Err(unknown),
        }
    }
}
