use odbc_sys::*;

/// Connection string to our Microsoft SQL Database. Boot it up with docker-compose up
#[allow(dead_code)] // Not used for now. Eventually we want to test against a real database.
const MSSQL: &str = "Driver={ODBC Driver 18 for SQL Server};\
    Server=localhost;\
    UID=SA;\
    PWD=My@Test@Password1;\
    TrustServerCertificate=yes;";

#[test]
fn allocate_environment() {
    let mut env = Handle::null();
    unsafe {
        assert_eq!(
            SqlReturn::SUCCESS,
            SQLAllocHandle(HandleType::Env, Handle::null(), &mut env as *mut Handle)
        );
        assert_eq!(SqlReturn::SUCCESS, SQLFreeHandle(HandleType::Env, env));
    }
}

#[test]
fn allocate_connection() {
    let mut env = Handle::null();
    let mut conn = Handle::null();

    unsafe {
        assert_eq!(
            SqlReturn::SUCCESS,
            SQLAllocHandle(HandleType::Env, Handle::null(), &mut env as *mut Handle)
        );

        assert_eq!(
            SqlReturn::SUCCESS,
            SQLSetEnvAttr(
                env.as_henv(),
                EnvironmentAttribute::OdbcVersion,
                AttrOdbcVersion::Odbc3.into(),
                0
            )
        );

        assert_eq!(
            SqlReturn::SUCCESS,
            SQLAllocHandle(HandleType::Dbc, env, &mut conn as *mut Handle)
        );

        assert_eq!(SqlReturn::SUCCESS, SQLFreeHandle(HandleType::Dbc, conn));
        assert_eq!(SqlReturn::SUCCESS, SQLFreeHandle(HandleType::Env, env));
    }
}

#[test]
fn allocate_connection_error() {
    let mut env = Handle::null();
    let mut conn = Handle::null();

    unsafe {
        assert_eq!(
            SqlReturn::SUCCESS,
            SQLAllocHandle(HandleType::Env, Handle::null(), &mut env as *mut Handle)
        );

        // Allocating connection without setting ODBC Version first should result in an error
        assert_eq!(
            SqlReturn::ERROR,
            SQLAllocHandle(HandleType::Dbc, env, &mut conn as *mut Handle)
        );

        assert_eq!(SqlReturn::SUCCESS, SQLFreeHandle(HandleType::Env, env));
    }
}
