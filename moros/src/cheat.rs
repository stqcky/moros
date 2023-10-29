use std::{sync::OnceLock, time::Duration};

use anyhow::Context;
use dotenvy_macro::dotenv;
use encryption::x;
use lazy_static::lazy_static;
use sdk::{create_interface, interfaces::client::client::Source2Client, xw};
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{
        Console::{AllocConsole, FreeConsole},
        LibraryLoader::FreeLibraryAndExitThread,
    },
    UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
};

use crate::ui;

static MODULE: OnceLock<HINSTANCE> = OnceLock::new();

pub fn attach(module: HINSTANCE) {
    if let Err(_) = MODULE.set(module) {
        log::error!("{}", x!("cheat module instance is already set. aborting."));
        return;
    }

    if let Err(_) = unsafe { AllocConsole() } {
        log::error!("{}", x!("could not allocate console. aborting."));
        return;
    }

    pretty_env_logger::env_logger::Builder::from_env(
        pretty_env_logger::env_logger::Env::default().default_filter_or(dotenv!("RUST_LOG")),
    )
    .init();

    if let Err(e) = init() {
        log::error!("{e:?}");
        detach();
    }
}

fn init() -> anyhow::Result<()> {
    ui::vulkan::setup()?;

    std::thread::sleep(Duration::from_secs(2));

    ui::vulkan::unhook().context("could not uhnook vulkan")?;

    detach();

    Ok(())
}

pub fn detach() {
    if let Some(module) = MODULE.get() {
        unsafe {
            MessageBoxA(None, xw!("detaching\0"), xw!("moros\0"), MB_OK);

            if let Err(e) = FreeConsole() {
                log::error!("{} {e}", x!("could not detach console. reason:"));
            }

            FreeLibraryAndExitThread(*module, 0);
        }
    } else {
        unsafe {
            MessageBoxA(
                None,
                xw!("could not properly detach cheat.\0"),
                xw!("moros\0"),
                MB_OK,
            );
        }
    }
}
