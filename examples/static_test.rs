// This is a minimal test example program, based off of one of the tests
// It's primary purpose is to provide something that should have odbc_sys linked
// in so we can check the static linking works properly.

extern crate odbc_sys;

use std::ptr::null_mut;

use odbc_sys::{Handle, HandleType, SQLAllocHandle, SQLFreeHandle, SqlReturn};

fn main() {
    let mut env: Handle = null_mut();
    unsafe {
        assert_eq!(
            SqlReturn::SUCCESS,
            SQLAllocHandle(HandleType::Env, null_mut(), &mut env as *mut Handle)
        );
        assert_eq!(SqlReturn::SUCCESS, SQLFreeHandle(HandleType::Env, env));
    }
}
