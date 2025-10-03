//! Contains test for the ffi layer
extern crate odbc_sys;
use odbc_sys::*;
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

#[test]
fn get_diagnostic_record() {
    let mut env = Handle::null();
    let mut conn = Handle::null();

    unsafe {
        assert_eq!(
            SqlReturn::SUCCESS,
            SQLAllocHandle(HandleType::Env, Handle::null(), &mut env as *mut Handle)
        );

        let result = SQLAllocHandle(HandleType::Dbc, env, &mut conn as *mut Handle);
        assert_eq!(SqlReturn::ERROR, result);

        let mut state = [0u8; 6];
        let mut native_error = 0i32;
        let mut message = [0u8; 256];
        let mut message_len = 0i16;

        let diag_result = SQLGetDiagRec(
            HandleType::Env,
            env,
            1,
            state.as_mut_ptr(),
            &mut native_error,
            message.as_mut_ptr(),
            message.len() as i16,
            &mut message_len,
        );

        assert!(diag_result == SqlReturn::SUCCESS || diag_result == SqlReturn::SUCCESS_WITH_INFO);
        assert!(message_len > 0);

        assert_eq!(SqlReturn::SUCCESS, SQLFreeHandle(HandleType::Env, env));
    }
}

#[test]
fn enumerate_drivers() {
    let mut env = Handle::null();

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

        let mut driver_desc = [0u8; 256];
        let mut desc_len = 0i16;
        let mut driver_attrs = [0u8; 256];
        let mut attrs_len = 0i16;

        let result = SQLDrivers(
            env.as_henv(),
            FetchOrientation::First,
            driver_desc.as_mut_ptr(),
            driver_desc.len() as i16,
            &mut desc_len,
            driver_attrs.as_mut_ptr(),
            driver_attrs.len() as i16,
            &mut attrs_len,
        );

        assert!(
            result == SqlReturn::SUCCESS
                || result == SqlReturn::SUCCESS_WITH_INFO
                || result == SqlReturn::NO_DATA
        );

        assert_eq!(SqlReturn::SUCCESS, SQLFreeHandle(HandleType::Env, env));
    }
}

#[test]
fn enumerate_data_sources() {
    let mut env = Handle::null();

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

        let mut dsn_name = [0u8; 256];
        let mut name_len = 0i16;
        let mut description = [0u8; 256];
        let mut desc_len = 0i16;

        let result = SQLDataSources(
            env.as_henv(),
            FetchOrientation::First,
            dsn_name.as_mut_ptr(),
            dsn_name.len() as i16,
            &mut name_len,
            description.as_mut_ptr(),
            description.len() as i16,
            &mut desc_len,
        );

        assert!(
            result == SqlReturn::SUCCESS
                || result == SqlReturn::SUCCESS_WITH_INFO
                || result == SqlReturn::NO_DATA
        );

        if result == SqlReturn::SUCCESS || result == SqlReturn::SUCCESS_WITH_INFO {
            assert!(name_len > 0);
        }

        assert_eq!(SqlReturn::SUCCESS, SQLFreeHandle(HandleType::Env, env));
    }
}
