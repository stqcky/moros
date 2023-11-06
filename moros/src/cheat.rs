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
use sdk::interfaces::client::entity_system::ENTITY_SYSTEM;
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

// class CGlobalVars
// {
// public:
//     float m_realtime; //0x0000
//     __int32 m_framecount; //0x0004
//     float N0000007D; //0x0008
//     float N00000087; //0x000C
//     __int32 m_maxclients; //0x0010
//     float m_intervalpertick; //0x0014
//     __int32 N0000007F; //0x0018
//     __int32 N0000008B; //0x001C
//     void* m_unkfunc; //0x0020
//     float N00000081; //0x0028
//     float m_curtime; //0x002C
//     float m_curtime2; //0x0030
//     char pad_0x0034[0xC]; //0x0034
//     __int32 m_tickcount; //0x0040
//     float m_intervalpertick2; //0x0044
//     void* m_current_netchan; //0x0048
//     char pad_0x0050[0x130]; //0x0050
//     char* m_current_map; //0x0180
//     char* m_current_mapname; //0x0188
//
// }; //Size=0x0190

fn init() -> anyhow::Result<()> {
    render::setup()?;

    for player in ENTITY_SYSTEM.players() {
        log::info!("name: {}", player.player_name());
        // log::info!("ping {}", player.ping());
        // log::info!(
        //     "leader count: {}",
        //     player
        //         .inventory_services()
        //         .unwrap()
        //         .persona_data_public_commends_leader()
        // );
        //
        // if let Some(pawn) = player.pawn() {
        //     log::info!("pawn -> {:p}", pawn);
        //     log::info!("team: {}", pawn.team_num());
        // }
    }

    let sig = signature::scan("client.dll", "40 53 48 81 EC ? ? ? ? 49 8B C1")?;

    log::info!("{:p}", sig.address as *const usize);

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
