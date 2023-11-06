use std::ffi::c_char;

use lazy_static::lazy_static;
use memory_macros::{vmt, C};

use crate::interfaces::{client::entity_system::GameEntitySystem, create_interface};

lazy_static! {
    pub static ref RESOURCE_SERVICE: &'static GameResourceService<'static> =
        create_interface!("engine2.dll", "GameResourceServiceClientV0");
}

#[vmt]
#[derive(C)]
pub struct GameResourceService<'a> {
    __pad0x0000: [u8; 0x20],
    interface_name: *const c_char,
    __pad0x0030: [u8; 0x28],
    entity_system: *const GameEntitySystem<'a>,
}

unsafe impl Sync for GameResourceService<'_> {}
