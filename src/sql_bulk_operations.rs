/// Supported `SQLBulkOperations` operations
#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlBulkOperation {
    SQL_ADD = 4,
    SQL_UPDATE_BY_BOOKMARK = 5,
    SQL_DELETE_BY_BOOKMARK = 6,
    SQL_FETCH_BY_BOOKMARK = 7,
}

pub use self::SqlBulkOperation::*;
