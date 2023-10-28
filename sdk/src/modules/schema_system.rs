use crate::{
    platform::get_module_handle,
    sdk::{
        interface_registry::InterfaceRegistry,
        interfaces::schema_system::schema_system::SchemaSystem,
    },
};
use anyhow::Context;
use encryption::x;
use lazy_static::lazy_static;

lazy_static! {
    static ref SCHEMA_SYSTEM: String = x!("schemasystem.dll\0");
}

pub struct SchemaSystemModule<'a> {
    pub schema_system: &'a SchemaSystem<'a>,
}

impl SchemaSystemModule<'_> {
    pub fn new() -> anyhow::Result<Self> {
        let module = get_module_handle(&SCHEMA_SYSTEM)
            .context(x!("could not get schema system module handle"))?;

        let mut interfaces = InterfaceRegistry::new(module)
            .context(x!("could not initialize schema system interface registry"))?;

        Ok(Self {
            schema_system: interfaces
                .create::<SchemaSystem>("SchemaSystem")
                .context(x!("could not create SchemaSystem interface"))?,
        })
    }
}
