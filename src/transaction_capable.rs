/// Level of transaction support offered by a data source. `SQLGetInfo` reports one of these values
/// for the information type [`crate::InfoType::TransactionCapable`] (`SQL_TXN_CAPABLE`).
///
/// This is a strict alias over `u16`, so values not listed here, which a driver may report, can be
/// represented as well.
///
/// See: <https://learn.microsoft.com/sql/odbc/reference/syntax/sqlgetinfo-function>
#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TransactionCapable(pub u16);

impl TransactionCapable {
    /// Equivalent to `SQL_TC_NONE` in the C-API. Transactions are not supported.
    pub const NONE: Self = Self(0);
    /// Equivalent to `SQL_TC_DML` in the C-API. Transactions can only contain data manipulation
    /// statements (DML). Data definition statements (DDL) encountered in a transaction cause an
    /// error.
    pub const DML: Self = Self(1);
    /// Equivalent to `SQL_TC_ALL` in the C-API. Transactions can contain DML and DDL statements.
    pub const ALL: Self = Self(2);
    /// Equivalent to `SQL_TC_DDL_COMMIT` in the C-API. Transactions can only contain DML
    /// statements. DDL statements encountered in a transaction cause the transaction to be
    /// committed.
    pub const DDL_COMMIT: Self = Self(3);
    /// Equivalent to `SQL_TC_DDL_IGNORE` in the C-API. Transactions can only contain DML
    /// statements. DDL statements encountered in a transaction are ignored.
    pub const DDL_IGNORE: Self = Self(4);
}
