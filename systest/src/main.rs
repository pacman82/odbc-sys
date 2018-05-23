#![allow(bad_style, unused_macros)]

extern crate odbc_sys;
use odbc_sys::*;

#[cfg(not(target_os = "windows"))]
include!(concat!(env!("OUT_DIR"), "/all.rs"));

#[test]
#[cfg(not(target_os = "windows"))]
fn test_correctness() {
    main()
}