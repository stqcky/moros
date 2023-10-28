use encryption::x;
use lazy_static::lazy_static;

use crate::{platform::get_module_handle, sdk::interface_registry::InterfaceRegistry};

lazy_static! {
    static ref CLIENT_MODULE: String = x!("client.dll\0");
}

pub struct ClientModule {}

impl ClientModule {
    pub fn new() -> anyhow::Result<Self> {
        let module = get_module_handle(&CLIENT_MODULE)?;
        let interfaces = InterfaceRegistry::new(module)?;

        Ok(Self {})
    }
}
