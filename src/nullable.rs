/// Used by `SQLDescribeCol`.
#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Nullable(pub i16);

impl Nullable {
    pub const UNKNOWN: Nullable = Nullable(2);
    pub const NULLABLE: Nullable = Nullable(1);
    pub const NO_NULLS: Nullable = Nullable(0);
}
