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
use sdk::{interfaces::client::entity_system::GameEntitySystem, xw};
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

use crate::{
    interfaces::ENTITY_SYSTEM,
    memory::{self, signature},
    ui,
};

static MODULE: OnceLock<HINSTANCE> = OnceLock::new();
static UNLOAD: AtomicBool = AtomicBool::new(false);

pub fn attach(module: HINSTANCE) {
    let _ = MODULE.set(module);

    if unsafe { AllocConsole() }.is_err() {
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

    log::info!("entity system: -> {:p}", *ENTITY_SYSTEM);

    let sig = signature::scan!("client.dll\0", "E8 ? ? ? ? 8B 08 FF C1 85 C9", 0)?;

    type A = fn(this: *const u8, a: usize) -> usize;

    let a: A = sig.call()?;

    log::info!("{:p}", a);

    // type GetHighestEntIndex = extern "fastcall" fn(this: *const GameEntitySystem, ret: *mut usize) -> usize;
    
    // for player in ENTITY_SYSTEM.players() {
    //     log::info!("name: {}", player.player_name());
    //     log::info!("ping {}", player.ping());
    //     log::info!(
    //         "leader count: {}",
    //         player
    //             .inventory_services()
    //             .unwrap()
    //             .persona_data_public_commends_leader()
    //     );
    //
    //     let pawn = ENTITY_SYSTEM.get_entity_by_handle(player.player_pawn());
    //
    //     if let Some(pawn) = pawn {
    //         log::info!("pawn -> {:p}", pawn);
    //         log::info!("team: {}", pawn.team_num());
    //     }
    // }

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

    let _ = destroy();
}
