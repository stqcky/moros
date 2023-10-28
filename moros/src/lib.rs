use std::time::Duration;

use windows::Win32::{Foundation::*, System::SystemServices::*};

mod cheat;
mod features;
mod platform;
mod sdk;
mod dump;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            std::thread::spawn(move || {
                cheat::attach(module);

                std::thread::sleep(Duration::from_secs(2));

                cheat::detach();
            });
        }
        _ => (),
    }

    true
}
