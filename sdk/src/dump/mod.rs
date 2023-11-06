use crate::interfaces::schema_system::schema_system::SCHEMA_SYSTEM;

mod schema;
mod sdk_builder;

pub fn schema(output: &str) {
    schema::dump(&SCHEMA_SYSTEM, output);
}
