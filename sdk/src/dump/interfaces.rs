use std::ffi::CStr;

use platform::windows::WindowsModule;
use windows::Win32::{
    Foundation::HMODULE,
    System::{
        LibraryLoader::GetModuleFileNameA, ProcessStatus::EnumProcessModules,
        Threading::GetCurrentProcess,
    },
};

use crate::interface_registry::InterfaceRegistry;

pub fn dump() -> anyhow::Result<()> {
    let mut modules = [HMODULE::default(); 256];
    let mut cb_needed = 0;

    unsafe {
        EnumProcessModules(
            GetCurrentProcess(),
            modules.as_mut_ptr(),
            std::mem::size_of::<[HMODULE; 256]>() as u32,
            &mut cb_needed,
        )?
    };

    for module in modules.iter().filter(|module| module.0 != 0) {
        let Ok(registry) = InterfaceRegistry::new(WindowsModule(*module)) else {
            continue;
        };

        let mut module_name = [0u8; 256];
        unsafe { GetModuleFileNameA(*module, &mut module_name) };
        let module_name =
            unsafe { CStr::from_ptr(module_name.as_ptr() as *const i8).to_string_lossy() };

        let base_module_name = module_name.split("\\").last().unwrap_or("unknown module");

        for interface in registry {
            log::info!(
                "{base_module_name} -> {} @ {:p}",
                interface.name(),
                interface
                    .instantiate::<*const usize>()
                    .unwrap_or(&std::ptr::null())
            );
        }

        println!("");
    }

    Ok(())
}
