use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
    time::Duration,
};

use dotenvy_macro::dotenv;
use encryption::x;
use windows::Win32::{
    Foundation::HINSTANCE,
    System::{
        Console::{AllocConsole, FreeConsole},
        LibraryLoader::FreeLibraryAndExitThread,
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

    if let Err(e) = init() {
        log::error!("{e}");
    }

    if let Err(e) = detach() {
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

pub fn detach() -> anyhow::Result<()> {
    ui::destroy()?;

    let module = MODULE.get().expect(&x!("module handle is null"));

    unsafe {
        if let Err(e) = FreeConsole() {
            log::error!("{} {e}", x!("could not free console. reason:"));
        }

        FreeLibraryAndExitThread(*module, 0);
    }
}
