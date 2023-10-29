use crate::schema::SCHEMA_SYSTEM;
use schema::Schema;

// scope networksystem.dll
// 2023-10-29 23:17:01.011033500 UTC

#[derive(Schema)]
#[scope("networksystem.dll")]
pub struct ChangeAccessorFieldPathIndex {
    #[field("ChangeAccessorFieldPathIndex_t", "m_Value")]
    value: i16,
}
