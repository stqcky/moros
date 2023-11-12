use platform::windows::WindowsModule;
use windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};

mod cheat;
mod features;
mod hook;
mod render;
pub mod ui;
mod vmt;
mod settings;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        std::thread::spawn(move || {
            cheat::attach(WindowsModule(module.into()));
        });
    }

    true
}
