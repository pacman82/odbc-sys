//These types can never be instantiated in Rust code. The actual objects are created by the ODBC
// driver manager and driver. We just want Rust to realize that these are distinct types. We never
// will work with the instances directly. ODBC will only care about the pointers to these.
pub enum Obj {}

pub enum Env {}

pub enum Dbc {}

pub enum Stmt {}

pub enum Description {}

pub type Handle = *mut Obj;
pub type HEnv = *mut Env;
pub type HDesc = *mut Description;

/// The connection handle references storage of all information about the connection to the data
/// source, including status, transaction state, and error information.
pub type HDbc = *mut Dbc;
pub type HStmt = *mut Stmt;
