#[cfg_attr(windows, link(name = "odbc32"))]
#[cfg_attr(all(not(windows), not(feature = "static")), link(name = "odbc"))]
#[cfg_attr(
    all(not(windows), feature = "static"),
    link(name = "odbc", kind = "static")
)]
extern "system" {}

pub use odbc_sys_core::*;
