/// Used by `SQLDescribeCol`.
#[repr(i16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Nullable {
    SQL_NULLABLE_UNKNOWN = 2,
    SQL_NULLABLE = 1,
    SQL_NO_NULLS = 0,
}
pub use Nullable::*;
