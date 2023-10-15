use std::{cell::OnceCell, time::Duration};

use windows::{
    core::s,
    Win32::{
        Foundation::*,
        System::Console::*,
        System::LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA},
        System::SystemServices::*,
        UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    },
};

use crate::interfaces::{client::client::Client, Interface};

mod interfaces;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

static mut MODULE: OnceCell<HINSTANCE> = OnceCell::new();

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn DllMain(module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            if MODULE.set(module).is_err() {
                return false;
            }

            std::thread::spawn(|| attach());
        }
        _ => (),
    }

    true
}

unsafe fn attach() {
    AllocConsole().expect("could not allocate console");

    let client = Client::new().unwrap();

    for interface in client.interfaces {
        println!("located client interface: {}", interface.name());
    }

    std::thread::sleep(Duration::from_secs(2));
    detach();
}

unsafe fn detach() {
    if let Some(module) = MODULE.get() {
        MessageBoxA(None, s!("detaching"), s!("detaching"), MB_OK);
        FreeConsole().expect("could not free console");
        FreeLibraryAndExitThread(*module, 0);
    } else {
        MessageBoxA(
            None,
            s!("Could not get module."),
            s!("error detaching"),
            MB_OK,
        );
    }
}
