use std::{
    panic::PanicInfo,
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
    time::Duration,
};

use anyhow::Context;
use dotenvy_macro::dotenv;
use encryption::x;
use memory::signature;
use platform::{
    module::PlatformModule,
    windows::{alloc_console, free_console, message_box, WindowsModule},
};
use sdk::interfaces::client::{entity_system::ENTITY_SYSTEM, global_vars::GLOBAL_VARS};
use windows::Win32::UI::WindowsAndMessaging::MB_OK;

use encryption_procmacro::encrypt;

use crate::render;

static MODULE: OnceLock<WindowsModule> = OnceLock::new();
static UNLOAD: AtomicBool = AtomicBool::new(false);

#[encrypt]
pub fn attach(module: WindowsModule) {
    let _ = MODULE.set(module);

    if alloc_console().is_err() {
        log::error!("could not allocate console. aborting.");
        return;
    }

    pretty_env_logger::env_logger::Builder::from_env(
        pretty_env_logger::env_logger::Env::default().default_filter_or("TRACE"),
    )
    .init();

    std::panic::set_hook(Box::new(panic_handler));

    if let Err(e) = init() {
        log::error!("{e}");
    }

    if let Err(e) = destroy() {
        log::error!("{}", e);
    }
}

fn init() -> anyhow::Result<()> {
    render::setup()?;

    sdk::dump::interfaces()?;

    while !UNLOAD.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

pub fn unload() {
    UNLOAD.store(true, Ordering::SeqCst);
}

#[encrypt]
pub fn destroy() -> anyhow::Result<()> {
    render::destroy()?;

    let module = MODULE.get().expect(&"module handle is null");

    if free_console().is_err() {
        log::error!("could not free console");
    }

    module.free_library_and_exit_thread(0);

    Ok(())
}

// #[encrypt]
fn panic_handler(info: &PanicInfo) {
    log::error!("panic: {}", info.to_string());
    message_box(&"moros error", &info.to_string(), MB_OK);

    let _ = destroy();
}
