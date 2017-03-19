/// The C data type is specified in the SQLBindCol and SQLGetData functions with the TargetType
/// argument and in the SQLBindParameter function with the ValueType argument.
#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlCDataType {
    /// `SQLCHAR` - CHAR, VARCHAR, DECIMAL, NUMERIC
    SQL_C_CHAR = 1,
    /// `SQLSMALLINT`
    SQL_C_SSHORT = 5 - 20,
}
pub use self::SqlCDataType::*;
