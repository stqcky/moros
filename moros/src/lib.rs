use std::time::Duration;

use windows::Win32::{Foundation::*, System::SystemServices::*};

mod dll;
mod interfaces;
mod mem;
mod modules;
mod win;
mod features;
mod cheat;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            if dll::MODULE.set(module).is_err() {
                return false;
            }

            std::thread::spawn(move || {
                if let Err(e) = dll::attach(module) {
                    log::error!("{e}");
                }

                std::thread::sleep(Duration::from_secs(2));

                unsafe { dll::detach() };
            });
        }
        _ => (),
    }

    true
}
