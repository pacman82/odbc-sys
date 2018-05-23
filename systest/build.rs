extern crate ctest;

fn build_ctest() {
    let mut cfg = ctest::TestGenerator::new();

    // Include the header files where the C APIs are defined
    cfg.header("odbcinst.h")
        .header("sql.h")
        .header("sqlext.h")
        .header("sqltypes.h")
        .header("sqlucode.h");

    //TODO: enum values are not checked now - only constants are

    //TODO: those are not really checked - be careful
    cfg.type_name(|ty, is_struct| if is_struct {
        match ty.to_string().as_ref() {
            "HandleType" => "SQLSMALLINT".to_owned(),
            "SqlStatementAttribute" => "SQLINTEGER".to_owned(),
            "EnvironmentAttribute" => "SQLINTEGER".to_owned(),
            "FetchOrientation" => "SQLUSMALLINT".to_owned(),
            "FreeStmtOption" => "SQLUSMALLINT".to_owned(),
            "SqlDataType" => "SQLSMALLINT".to_owned(),
            "SqlCDataType" => "SQLSMALLINT".to_owned(),
            "InfoType" => "SQLUSMALLINT".to_owned(),
            "SqlConnectionAttribute" => "SQLINTEGER".to_owned(),
            "SqlCompletionType" => "SQLSMALLINT".to_owned(),
            "InputOutput" => "SQLSMALLINT".to_owned(),
            "SqlDriverConnectOption" => "SQLUSMALLINT".to_owned(),
            "Nullable" => "SQLSMALLINT".to_owned(),
            _ => ty.to_string()
        }
    } else {
        ty.to_string()
    });

    let skip_signedness = vec![
        "SQLHENV",
        "SQLHDBC",
        "SQLHSTMT",
        "SQLPOINTER",
        "SQLHWND",
        "SQLHANDLE",
    ];

    cfg.skip_signededness(move |t| skip_signedness.contains(&t));

    //TODO: those are incompatible with headers due to difference in constness of raw pointers for input params
    let skip_fn = vec![
        "SQLPrepare",
        "SQLExecDirect",
        "SQLConnect",
        "SQLExecDirect",
        "SQLTables",
        "SQLDriverConnect"
    ];

    cfg.skip_fn(move |t| skip_fn.contains(&t));

    // Include the directory where the header files are defined
    // cfg.include("/usr/local/include/");

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../src/lib.rs", "all.rs");
}

fn main() {
    if cfg!(not(target_os = "windows")) {
        build_ctest();
    }

}