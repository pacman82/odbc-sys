//! Contains test for the ffi layer
extern crate odbc_sys;
use odbc_sys::HandleType::*;
use odbc_sys::*;
use std::ptr;

const SUCCESS: SqlReturn = SqlReturn::SUCCESS;

fn succeeds(ret: SqlReturn) {
    assert_eq!(SUCCESS, ret);
}

#[test]
fn allocate_environment() {
    let mut env = Handle::null();
    unsafe {
        succeeds(SQLAllocHandle(Env, Handle::null(), &mut env as *mut Handle));
        succeeds(SQLFreeHandle(Env, env));
    }
}

#[test]
fn allocate_connection() {
    let mut env = Handle::null();
    let mut conn = Handle::null();

    unsafe {
        succeeds(SQLAllocHandle(Env, Handle::null(), &mut env as *mut Handle));

        succeeds(SQLSetEnvAttr(
            env.as_henv(),
            EnvironmentAttribute::OdbcVersion,
            AttrOdbcVersion::Odbc3.into(),
            0,
        ));

        succeeds(SQLAllocHandle(Dbc, env, &mut conn as *mut Handle));

        succeeds(SQLFreeHandle(Dbc, conn));
        succeeds(SQLFreeHandle(Env, env));
    }
}

#[test]
fn allocate_connection_error() {
    let mut env = Handle::null();
    let mut conn = Handle::null();

    unsafe {
        succeeds(SQLAllocHandle(Env, Handle::null(), &mut env as *mut Handle));

        assert_eq!(
            SqlReturn::ERROR,
            SQLAllocHandle(Dbc, env, &mut conn as *mut Handle)
        );

        succeeds(SQLFreeHandle(Env, env));
    }
}

#[test]
#[ignore]
fn select_42() {
    let mut env = Handle::null();
    let mut conn = Handle::null();
    let mut stmt = Handle::null();

    unsafe {
        succeeds(SQLAllocHandle(Env, Handle::null(), &mut env as *mut Handle));
        succeeds(SQLSetEnvAttr(
            env.as_henv(),
            EnvironmentAttribute::OdbcVersion,
            AttrOdbcVersion::Odbc3.into(),
            0,
        ));
        succeeds(SQLAllocHandle(Dbc, env, &mut conn as *mut Handle));

        let conn_str = b"DRIVER=SQLite3;Database=:memory:";
        succeeds(SQLDriverConnect(
            conn.as_hdbc(),
            ptr::null_mut(),
            conn_str.as_ptr(),
            conn_str.len() as i16,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            DriverConnectOption::NoPrompt,
        ));

        succeeds(SQLAllocHandle(Stmt, conn, &mut stmt as *mut Handle));

        let sql = b"SELECT 42";
        succeeds(SQLExecDirect(
            stmt.as_hstmt(),
            sql.as_ptr(),
            sql.len() as i32,
        ));

        let mut result: i32 = 0;
        let mut indicator: isize = 0;
        succeeds(SQLBindCol(
            stmt.as_hstmt(),
            1,
            CDataType::SLong,
            &mut result as *mut i32 as *mut _,
            0,
            &mut indicator as *mut isize,
        ));
        succeeds(SQLFetch(stmt.as_hstmt()));
        assert_eq!(42, result);

        succeeds(SQLFreeHandle(Stmt, stmt));
        succeeds(SQLDisconnect(conn.as_hdbc()));
        succeeds(SQLFreeHandle(Dbc, conn));
        succeeds(SQLFreeHandle(Env, env));
    }
}

#[test]
#[ignore]
fn invalid_sql_error() {
    let mut env = Handle::null();
    let mut conn = Handle::null();
    let mut stmt = Handle::null();

    unsafe {
        succeeds(SQLAllocHandle(Env, Handle::null(), &mut env as *mut Handle));
        succeeds(SQLSetEnvAttr(
            env.as_henv(),
            EnvironmentAttribute::OdbcVersion,
            AttrOdbcVersion::Odbc3.into(),
            0,
        ));
        succeeds(SQLAllocHandle(Dbc, env, &mut conn as *mut Handle));

        let conn_str = b"DRIVER=SQLite3;Database=:memory:";
        succeeds(SQLDriverConnect(
            conn.as_hdbc(),
            ptr::null_mut(),
            conn_str.as_ptr(),
            conn_str.len() as i16,
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            DriverConnectOption::NoPrompt,
        ));

        succeeds(SQLAllocHandle(Stmt, conn, &mut stmt as *mut Handle));

        let sql = b"xurgblob";
        assert_eq!(
            SqlReturn::ERROR,
            SQLExecDirect(stmt.as_hstmt(), sql.as_ptr(), sql.len() as i32)
        );

        let mut sqlstate = [0u8; 6];
        let mut native_error = 0i32;
        let mut message_text = [0u8; 512];
        let mut text_length = 0i16;
        succeeds(SQLGetDiagRec(
            Stmt,
            stmt,
            1,
            sqlstate.as_mut_ptr(),
            &mut native_error as *mut i32,
            message_text.as_mut_ptr(),
            message_text.len() as i16,
            &mut text_length as *mut i16,
        ));
        assert!(text_length > 0);
        let message_bytes = &message_text[..text_length as usize];
        assert!(message_bytes.ends_with(b"syntax error (1)"));

        succeeds(SQLFreeHandle(Stmt, stmt));
        succeeds(SQLDisconnect(conn.as_hdbc()));
        succeeds(SQLFreeHandle(Dbc, conn));
        succeeds(SQLFreeHandle(Env, env));
    }
}
