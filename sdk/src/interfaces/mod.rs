use crate::{interface_registry::InterfaceRegistry, platform::get_module_handle};
use anyhow::Context;
use encryption::x;

pub mod client;
pub mod schema_system;

pub fn create_interface<'a, T>(module: &'a str, interface: &'a str) -> anyhow::Result<&'static T> {
    let module_handle = get_module_handle(module)
        .context(format!("{}: {module}", x!("could not get module handle")))?;

    let mut registry = InterfaceRegistry::new(module_handle).context(format!(
        "{}: {module}",
        x!("could not initialize interface registry")
    ))?;

    Ok(registry
        .create::<T>(interface)
        .context(format!("{}: {interface}", x!("could not create interface")))?)
}

#[macro_export]
macro_rules! create_interface {
    ($mod:literal, $interface:literal) => {
        $crate::interfaces::create_interface(
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
