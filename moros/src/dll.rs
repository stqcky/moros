use std::{cell::OnceCell, time::Duration};

use crate::modules::client::ClientModule;
use dotenvy_macro::dotenv;
use encryption::x;
use thiserror::Error;
use windows::{
    core::s,
    Win32::{
        Foundation::HINSTANCE,
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA},
        },
        UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    },
};

pub static mut MODULE: OnceCell<HINSTANCE> = OnceCell::new();

#[derive(Error, Debug)]
pub enum CheatInitError {
    #[error("{}", x!("could not allocate console"))]
    CouldNotAllocConsole,
    #[error("{}", x!("error while initializing modules"))]
    CouldNotInitModules,
}

pub fn attach(module: HINSTANCE) -> Result<(), CheatInitError> {
    unsafe { AllocConsole().map_err(|_| CheatInitError::CouldNotAllocConsole)? };

    pretty_env_logger::env_logger::Builder::from_env(
        pretty_env_logger::env_logger::Env::default().default_filter_or(dotenv!("RUST_LOG")),
    )
    .init();

    log::trace!("{}", x!("initializing client.dll module"));

    let mut client_module = match ClientModule::new() {
        Ok(client) => client,
        Err(e) => {
            log::error!("{}{}", x!("error initializing client.dll module: "), e);
            return Err(CheatInitError::CouldNotInitModules);
        }
    };

    let source2client = client_module.interfaces.find(&x!("Source2Client"));

    if let Some(client) = source2client {
        let a = client.create::<u64>().unwrap();

        log::info!("{}", a);
    }

    Ok(())
}

pub unsafe fn detach() {
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
