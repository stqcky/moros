use crate::interface_registry::InterfaceRegistry;
use anyhow::Context;
use encryption::encryption_procmacro::encrypt;
use platform::windows::get_module_handle;

pub mod client;
pub mod engine;
pub mod schema_system;

#[encrypt]
pub fn create_interface_internal<'a, T>(
    module: &'a str,
    interface: &'a str,
) -> anyhow::Result<&'static T> {
    let module_handle =
        get_module_handle(module).context(format!("could not get module handle: {module}"))?;

    let mut registry = InterfaceRegistry::new(module_handle)
        .context(format!("could not initialize interface registry: {module}",))?;

    Ok(registry
        .create::<T>(interface)
        .context(format!("could not create interface: {interface}"))?)
}

macro_rules! create_interface {
    ($mod:literal, $interface:literal) => {
        $crate::interfaces::create_interface_internal(
            &encryption::x!($mod),
            &encryption::x!($interface),
        )
        .expect(&format!(
            "{}: {}",
            encryption::x!("could not create interface"),
            encryption::x!($interface)
        ))
    };
}
pub(crate) use create_interface;
