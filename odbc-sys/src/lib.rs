#[cfg(feature = "unixodbc-sys")]
pub use unixodbc_sys::*;

#[cfg(feature = "iodbc-sys")]
pub use iodbc_sys::*;

#[cfg(all(not(feature = "static"), not(feature = "iodbc")))]
#[cfg_attr(windows, link(name = "odbc32"))]
#[cfg_attr(not(windows), link(name = "odbc"))]
extern "system" {}

#[cfg(all(not(feature = "static"), feature = "iodbc"))]
#[cfg_attr(windows, link(name = "odbc32"))]
#[cfg_attr(not(windows), link(name = "iodbc"))]
extern "system" {}

#[cfg(not(feature = "static"))]
pub use odbc_sys_core::*;
