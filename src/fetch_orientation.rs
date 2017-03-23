/// Codes used for FetchOrientation in `SQLFetchScroll`, `SQLDataSources` and in `SQLDrivers`
#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FetchOrientation {
    SQL_FETCH_NEXT = 1,
    SQL_FETCH_FIRST = 2,
    // Other codes used for FetchOrientation in SQLFetchScroll()
    SQL_FETCH_LAST = 3,
    SQL_FETCH_PRIOR = 4,
    SQL_FETCH_ABSOLUTE = 5,
    SQL_FETCH_RELATIVE = 6,
    // additional SQLDataSources fetch directions
    SQL_FETCH_FIRST_USER = 31,
    SQL_FETCH_FIRST_SYSTEM = 32,
}
pub use self::FetchOrientation::*;
