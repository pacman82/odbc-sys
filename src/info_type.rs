/// Information requested by SQLGetInfo
#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InfoType {
    MaxDriverConnections = 0,
    MaxConcurrentActivities = 1,
    DataSourceName = 2,
    /// `SQL_DRIVER_NAME`. The name of the driver implementation file.
    DriverName = 6,
    /// `SQL_DRIVER_VER`. The version of the driver, in the form "MM.mm.rrrr".
    DriverVer = 7,
    // FetchDirection = 8, Deprecated in ODBC 3
    ServerName = 13,
    SearchPatternEscape = 14,
    DbmsName = 17,
    DbmsVer = 18,
    AccessibleTables = 19,
    AccessibleProcedures = 20,
    /// `SQL_CONCAT_NULL_BEHAVIOR`. How the data source handles concatenation of
    /// NULL-valued character columns with non-NULL valued character columns.
    ConcatNullBehavior = 22,
    CursorCommitBehaviour = 23,
    DataSourceReadOnly = 25,
    DefaultTxnIsolation = 26,
    /// `SQL_EXPRESSIONS_IN_ORDERBY`. Whether the data source supports expressions
    /// in the ORDER BY list ("Y" or "N").
    ExpressionsInOrderBy = 27,
    IdentifierCase = 28,
    IdentifierQuoteChar = 29,
    MaxColumnNameLen = 30,
    MaxCursorNameLen = 31,
    MaxSchemaNameLen = 32,
    MaxCatalogNameLen = 34,
    MaxTableNameLen = 35,
    /// `SQL_MULT_RESULT_SETS`. Whether the data source supports multiple result
    /// sets ("Y" or "N").
    MultResultSets = 36,
    /// `SQL_OUTER_JOINS` (ODBC 2.x). Replaced by `OuterJoinCapabilities` in
    /// ODBC 3.x, but some Driver Managers still query this value.
    OuterJoins = 38,
    /// `SQL_SCHEMA_TERM` (also known as `SQL_OWNER_TERM` in ODBC 2.x). The data
    /// source vendor's name for a schema (e.g. "schema", "owner").
    SchemaTerm = 39,
    /// `SQL_CATALOG_NAME_SEPARATOR` (also known as `SQL_QUALIFIER_NAME_SEPARATOR`
    /// in ODBC 2.x). The character used as a separator between a catalog name and
    /// the qualified name element that follows or precedes it.
    CatalogNameSeparator = 41,
    /// `SQL_CATALOG_TERM` (also known as `SQL_QUALIFIER_TERM` in ODBC 2.x). The
    /// data source vendor's name for a catalog (e.g. "catalog", "database").
    CatalogTerm = 42,
    // ScrollConcurrency = 43, deprecated in ODBC 3
    /// `SQL_SCROLL_OPTIONS` C-API places this in the extended header. Lists the supported cursor
    /// types (forward-only, static, keyset-driven, dynamic, or mixed). All data sources must
    /// support forward-only cursors. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    ScrollOptions = 44,
    TransactionCapable = 46,
    UserName = 47,
    /// `SQL_CONVERT_FUNCTIONS`. The scalar conversion functions supported by the
    /// driver and data source (bitmask: `SQL_FN_CVT_CAST`, `SQL_FN_CVT_CONVERT`).
    ConvertFunctions = 48,
    /// `SQL_NUMERIC_FUNCTIONS`. The scalar numeric functions supported by the
    /// driver and data source (bitmask).
    NumericFunctions = 49,
    /// `SQL_STRING_FUNCTIONS`. The scalar string functions supported by the driver
    /// and data source (bitmask).
    StringFunctions = 50,
    /// `SQL_SYSTEM_FUNCTIONS`. The scalar system functions supported by the driver
    /// and data source (bitmask).
    SystemFunctions = 51,
    /// `SQL_TIMEDATE_FUNCTIONS`. The scalar date and time functions supported by
    /// the driver and data source (bitmask).
    TimedateFunctions = 52,
    TransactionIsolationProtocol = 72,
    Integrity = 73,
    /// `SQL_CORRELATION_NAME`. Whether correlated table names are supported.
    CorrelationName = 74,
    /// `SQL_NON_NULLABLE_COLUMNS`. Whether the data source supports NOT NULL in
    /// columns.
    NonNullableColumns = 75,
    /// `SQL_DRIVER_ODBC_VER`. The version of ODBC that the driver supports, in
    /// the form "MM.mm" (e.g. "03.80").
    DriverOdbcVer = 77,
    GetDataExtensions = 81,
    NullCollation = 85,
    AlterTable = 86,
    /// `SQL_COLUMN_ALIAS`. Whether the data source supports column aliases
    /// ("Y" or "N").
    ColumnAlias = 87,
    /// `SQL_GROUP_BY`. The relationship between the columns in the GROUP BY
    /// clause and the nonaggregated columns in the select list.
    GroupBy = 88,
    OrderByColumnsInSelect = 90,
    /// `SQL_SCHEMA_USAGE` (also known as `SQL_OWNER_USAGE` in ODBC 2.x). A
    /// bitmask enumerating the statements in which schemas can be used.
    SchemaUsage = 91,
    /// `SQL_CATALOG_USAGE` (also known as `SQL_QUALIFIER_USAGE` in ODBC 2.x). A
    /// bitmask enumerating the statements in which catalogs can be used.
    CatalogUsage = 92,
    SpecialCharacters = 94,
    /// `SQL_SUBQUERIES`. A bitmask enumerating the predicates that support
    /// subqueries.
    Subqueries = 95,
    /// `SQL_UNION_STATEMENT` (also known as `SQL_UNION` in ODBC 2.x). A bitmask
    /// enumerating support for the UNION statement.
    UnionStatement = 96,
    MaxColumnsInGroupBy = 97,
    MaxColumnsInIndex = 98,
    MaxColumnsInOrderBy = 99,
    MaxColumnsInSelect = 100,
    MaxColumnsInTable = 101,
    MaxIndexSize = 102,
    /// `SQL_MAX_ROW_SIZE_INCLUDES_LONG`. Whether the maximum row size returned
    /// for `MaxRowSize` includes the length of all `SQL_LONGVARCHAR` and
    /// `SQL_LONGVARBINARY` columns.
    MaxRowSizeIncludesLong = 103,
    MaxRowSize = 104,
    MaxStatementLen = 105,
    MaxTablesInSelect = 106,
    MaxUserNameLen = 107,
    /// `SQL_TIMEDATE_ADD_INTERVALS`. A bitmask enumerating the timestamp
    /// intervals supported by the TIMESTAMPADD scalar function.
    TimedateAddIntervals = 109,
    /// `SQL_TIMEDATE_DIFF_INTERVALS`. A bitmask enumerating the timestamp
    /// intervals supported by the TIMESTAMPDIFF scalar function.
    TimedateDiffIntervals = 110,
    /// `SQL_NEED_LONG_DATA_LEN`. Whether the data source needs the length of a
    /// long data value before that value is sent to the data source ("Y" or "N").
    NeedLongDataLen = 111,
    /// `SQL_LIKE_ESCAPE_CLAUSE`. Whether the LIKE predicate supports an escape
    /// character ("Y" or "N").
    LikeEscapeClause = 113,
    /// `SQL_CATALOG_LOCATION` (also known as `SQL_QUALIFIER_LOCATION` in ODBC
    /// 2.x). The position of the catalog in a qualified table name.
    CatalogLocation = 114,
    OuterJoinCapabilities = 115,
    /// `SQL_ACTIVE_ENVIRONMENTS` C-API places this in the extended header. A `u16` value that
    /// specifies the maximum number of active environments that the driver can support. If there is
    /// no specified limit or the limit is unknown, this value is set to zero.
    ActiveEnvironments = 116,
    /// `SQL_SQL_CONFORMANCE`. The level of SQL-92 supported by the driver.
    SqlConformance = 118,
    /// `SQL_BATCH_ROW_COUNT`. A bitmask enumerating the behavior of the driver
    /// with respect to the availability of row counts.
    BatchRowCount = 120,
    /// `SQL_BATCH_SUPPORT`. A bitmask enumerating the driver's support for
    /// batches.
    BatchSupport = 121,
    /// `SQL_DYNAMIC_CURSOR_ATTRIBUTES1` C-API places this in the extended header. Lists the fetch
    /// types supported by scrollable cursors. The bits in the return value correspond to the fetch
    /// types in SQLFetchScroll. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    DynamicCursorAttributes1 = 144,
    /// `SQL_DYNAMIC_CURSOR_ATTRIBUTES2` C-API places this in the extended header. Lists whether
    /// cursors can detect their own updates, deletes, and inserts. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    DynamicCursorAttributes2 = 145,
    /// `SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1` C-API places this in the extended header. Lists the
    /// fetch types supported by scrollable cursors. The bits in the return value correspond to the
    /// fetch types in SQLFetchScroll. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    ForwardOnlyCursorAttributes1 = 146,
    /// `SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2` C-API places this in the extended header. Lists
    /// whether cursors can detect their own updates, deletes, and inserts. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    ForwardOnlyCursorAttributes2 = 147,
    /// `SQL_KEYSET_CURSOR_ATTRIBUTES1` C-API places this in the extended header. Lists the fetch
    /// types supported by scrollable cursors. The bits in the return value correspond to the fetch
    /// types in SQLFetchScroll. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    KeysetCursorAttributes1 = 150,
    /// `SQL_KEYSET_CURSOR_ATTRIBUTES2` C-API places this in the extended header. Lists whether
    /// cursors can detect their own updates, deletes, and inserts. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    KeysetCursorAttributes2 = 151,
    /// `SQL_ODBC_INTERFACE_CONFORMANCE`. The level of the ODBC 3.x interface that
    /// the driver complies with.
    OdbcInterfaceConformance = 152,
    /// `SQL_PARAM_ARRAY_ROW_COUNTS`. A bitmask enumerating the driver's
    /// properties regarding the availability of row counts in a parameterized
    /// execution.
    ParamArrayRowCounts = 153,
    /// `SQL_PARAM_ARRAY_SELECTS`. A bitmask enumerating the driver's properties
    /// regarding the availability of result sets in a parameterized execution.
    ParamArraySelects = 154,
    /// `SQL_SQL92_DATETIME_FUNCTIONS`. A bitmask enumerating the datetime scalar
    /// functions supported per SQL-92.
    Sql92DatetimeFunctions = 155,
    /// `SQL_SQL92_FOREIGN_KEY_DELETE_RULE`. Rules supported for a foreign key in
    /// a DELETE statement, as defined in SQL-92.
    Sql92ForeignKeyDeleteRule = 156,
    /// `SQL_SQL92_FOREIGN_KEY_UPDATE_RULE`. Rules supported for a foreign key in
    /// an UPDATE statement, as defined in SQL-92.
    Sql92ForeignKeyUpdateRule = 157,
    /// `SQL_SQL92_GRANT`. Clauses supported in the GRANT statement, as defined
    /// in SQL-92.
    Sql92Grant = 158,
    /// `SQL_SQL92_NUMERIC_VALUE_FUNCTIONS`. Numeric value scalar functions
    /// supported per SQL-92.
    Sql92NumericValueFunctions = 159,
    /// `SQL_SQL92_PREDICATES`. Predicates supported in a SELECT statement, as
    /// defined in SQL-92.
    Sql92Predicates = 160,
    /// `SQL_SQL92_RELATIONAL_JOIN_OPERATORS`. Relational join operators supported
    /// in a SELECT statement, as defined in SQL-92.
    Sql92RelationalJoinOperators = 161,
    /// `SQL_SQL92_REVOKE`. Clauses supported in the REVOKE statement, as defined
    /// in SQL-92.
    Sql92Revoke = 162,
    /// `SQL_SQL92_ROW_VALUE_CONSTRUCTOR`. Row value constructor expressions
    /// supported in a SELECT statement, as defined in SQL-92.
    Sql92RowValueConstructor = 163,
    /// `SQL_SQL92_STRING_FUNCTIONS`. String scalar functions supported per
    /// SQL-92.
    Sql92StringFunctions = 164,
    /// `SQL_SQL92_VALUE_EXPRESSIONS`. Value expressions supported, as defined in
    /// SQL-92.
    Sql92ValueExpressions = 165,
    /// `SQL_STATIC_CURSOR_ATTRIBUTES1` C-API places this in the extended header. Lists the fetch
    /// types supported by scrollable cursors. The bits in the return value correspond to the fetch
    /// types in SQLFetchScroll. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    StaticCursorAttributes1 = 167,
    /// `SQL_STATIC_CURSOR_ATTRIBUTES2` C-API places this in the extended header. Lists whether
    /// cursors can detect their own updates, deletes, and inserts. See:
    /// <https://learn.microsoft.com/sql/odbc/reference/develop-app/determining-cursor-capabilities>
    StaticCursorAttributes2 = 168,
    /// `SQL_AGGREGATE_FUNCTIONS`. A bitmask enumerating support for aggregation
    /// functions.
    AggregateFunctions = 169,
    XopenCliYear = 10000,
    CursorSensitivity = 10001,
    DescribeParameter = 10002,
    CatalogName = 10003,
    CollationSeq = 10004,
    MaxIdentifierLen = 10005,
    AsyncMode = 10021,
    MaxAsyncConcurrentStatements = 10022,
    AsyncDbcFunctions = 10023,
    DriverAwarePoolingSupported = 10024,
    AsyncNotification = 10025,
}
