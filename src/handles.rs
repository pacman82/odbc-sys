use std::{ffi::c_void, ptr::null_mut};

// ODBC handles are pointers to opaque C types. We do not know their memory layout or other details
// of their implementation. This RFC suggest how to represent them in Rust:
// <https://rust-lang.github.io/rfcs/1861-extern-types.html>
//
// Until this is stable we could choose to represent them like suggested in the nomicon:
// <https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs>
//
// However, we know in the context of ODBC that we are always interested in the *mut versions of
// this pointer. For now a strict alias around `*mut c_void` seems to be the most pragmatic
// solution.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Handle(pub *mut c_void);

impl Handle {
    pub fn null() -> Self {
        Self(null_mut())
    }

    pub fn as_henv(self) -> HEnv {
        HEnv(self.0)
    }

    pub fn as_hdbc(self) -> HDbc {
        HDbc(self.0)
    }

    pub fn as_hstmt(self) -> HStmt {
        HStmt(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HEnv(pub *mut c_void);

impl HEnv {
    pub fn null() -> Self {
        Self(null_mut())
    }

    pub fn as_handle(self) -> Handle {
        Handle(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HDesc(pub *mut c_void);

impl HDesc {
    pub fn null() -> Self {
        Self(null_mut())
    }

    pub fn as_handle(self) -> Handle {
        Handle(self.0)
    }
}

/// The connection handle references storage of all information about the connection to the data
/// source, including status, transaction state, and error information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HDbc(pub *mut c_void);

impl HDbc {
    pub fn as_handle(self) -> Handle {
        Handle(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HStmt(pub *mut c_void);

impl HStmt {
    pub fn as_handle(self) -> Handle {
        Handle(self.0)
    }
}

// Handles are `Send` according to ODBC spec, since they must be able to be used from different
// threads. They even must protect their inner state. This would also make them `Sync`, yet they do
// have interior mutability for error handling.
// See: <https://docs.microsoft.com/en-us/sql/odbc/reference/develop-app/multithreading>

unsafe impl Send for HEnv {}

unsafe impl Send for HDbc {}

// Send may not be implemented for HStmt, since statements may bind other parameters and there is no
// guarantee that the parameter pointers bound to the statement are `Send`. Safe abstractions over
// statement handles may of course only allow `Send` parameters to bind and then implement `Send`.