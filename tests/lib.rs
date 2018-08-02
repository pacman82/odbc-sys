//! Contains test for the ffi layer
extern crate odbc_sys;
use odbc_sys::*;
use std::ptr::null_mut;

#[test]
fn allocate_environment() {
    let mut env: SQLHANDLE = null_mut();
    unsafe {
        assert_eq!(
            SQLRETURN::SQL_SUCCESS,
            SQLAllocHandle(SQL_HANDLE_ENV, null_mut(), &mut env as *mut SQLHANDLE)
        );
        assert_eq!(SQLRETURN::SQL_SUCCESS, SQLFreeHandle(SQL_HANDLE_ENV, env));
    }
}

#[test]
fn allocate_connection() {
    let mut env: SQLHANDLE = null_mut();
    let mut conn: SQLHANDLE = null_mut();

    unsafe {
        assert_eq!(
            SQL_SUCCESS,
            SQLAllocHandle(SQL_HANDLE_ENV, null_mut(), &mut env as *mut SQLHANDLE)
        );

        assert_eq!(
            SQL_SUCCESS,
            SQLSetEnvAttr(
                env as SQLHENV,
                SQL_ATTR_ODBC_VERSION,
                SQL_OV_ODBC3.into(),
                0
            )
        );

        assert_eq!(
            SQL_SUCCESS,
            SQLAllocHandle(SQL_HANDLE_DBC, env, &mut conn as *mut SQLHANDLE)
        );

        assert_eq!(SQL_SUCCESS, SQLFreeHandle(SQL_HANDLE_DBC, conn));
        assert_eq!(SQL_SUCCESS, SQLFreeHandle(SQL_HANDLE_ENV, env));
    }
}

#[test]
fn allocate_connection_error() {
    let mut env: SQLHANDLE = null_mut();
    let mut conn: SQLHANDLE = null_mut();

    unsafe {
        assert_eq!(
            SQL_SUCCESS,
            SQLAllocHandle(SQL_HANDLE_ENV, null_mut(), &mut env as *mut SQLHANDLE)
        );

        // Allocating connection without setting ODBC Version first should result in an error
        assert_eq!(
            SQL_ERROR,
            SQLAllocHandle(SQL_HANDLE_DBC, env, &mut conn as *mut SQLHANDLE)
        );

        assert_eq!(SQL_SUCCESS, SQLFreeHandle(SQL_HANDLE_ENV, env));
    }
}
