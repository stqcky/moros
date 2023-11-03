use std::{
    panic::PanicInfo,
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
    time::Duration,
};

use dotenvy_macro::dotenv;
use encryption::x;
use sdk::xw;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HINSTANCE,
        System::{
            Console::{AllocConsole, FreeConsole},
            LibraryLoader::FreeLibraryAndExitThread,
        },
        UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
    },
};

use crate::ui;

static MODULE: OnceLock<HINSTANCE> = OnceLock::new();
static UNLOAD: AtomicBool = AtomicBool::new(false);

pub fn attach(module: HINSTANCE) {
    let _ = MODULE.set(module);

    if let Err(_) = unsafe { AllocConsole() } {
        log::error!("{}", x!("could not allocate console. aborting."));
        return;
    }

    pretty_env_logger::env_logger::Builder::from_env(
        pretty_env_logger::env_logger::Env::default().default_filter_or(dotenv!("RUST_LOG")),
    )
    .init();

    std::panic::set_hook(Box::new(panic_handler));

    if let Err(e) = init() {
        log::error!("{e}");
    }

    if let Err(e) = destroy() {
        log::error!("{e}");
    }
}

fn init() -> anyhow::Result<()> {
    ui::setup()?;

    while !UNLOAD.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

pub fn unload() {
    UNLOAD.store(true, Ordering::SeqCst);
}

pub fn destroy() -> anyhow::Result<()> {
    ui::destroy()?;

    let module = MODULE.get().expect(&x!("module handle is null"));

    unsafe {
        if let Err(e) = FreeConsole() {
            log::error!("{} {e}", x!("could not free console. reason:"));
        }

        FreeLibraryAndExitThread(*module, 0);
    }
}

fn panic_handler(info: &PanicInfo) {
    unsafe {
        MessageBoxA(
            None,
            PCSTR::from_raw((info.to_string() + "\0").as_ptr()),
            xw!("moros error\0"),
            MB_OK,
        )
    };
}
