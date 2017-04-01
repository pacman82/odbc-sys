/// Used by `SQLBindParameter`.
#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InputOutput {
    SQL_PARAM_TYPE_UNKNOWN = 0,
    SQL_PARAM_INPUT = 1,
    SQL_PARAM_INPUT_OUTPUT = 2,
    SQL_RESULT_COL = 3,
    SQL_PARAM_OUTPUT = 4,
    SQL_RETURN_VALUE = 5,
    #[cfg(feature = "odbc_version_3_80")]
    SQL_PARAM_INPUT_OUTPUT_STREAM = 8,
    #[cfg(feature = "odbc_version_3_80")]
    SQL_PARAM_OUTPUT_STREAM = 16,
}
pub use InputOutput::*;

