use encryption::x;
use lazy_static::lazy_static;

use crate::interfaces::{create_interface, schema_system::schema_system::SchemaSystem};

pub mod client;
pub mod engine2;
pub mod global;
pub mod networksystem;
pub mod panorama;
pub mod rendersystemvulkan;

lazy_static! {
    pub static ref SCHEMA_SYSTEM: &'static SchemaSystem<'static> =
        create_interface(&x!("schemasystem.dll\0"), &x!("SchemaSystem"))
            .expect("could not create SchemaSystem interface");
}
