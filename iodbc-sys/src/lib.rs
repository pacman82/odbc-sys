#[cfg_attr(windows, link(name = "odbc32"))]
#[cfg_attr(all(not(windows), not(feature = "static")), link(name = "iodbc"))]
#[cfg_attr(
    all(not(windows), feature = "static"),
    link(name = "iodbc", kind = "static")
)]
extern "system" {}

pub use odbc_sys_core::*;
