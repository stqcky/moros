use std::ffi::c_char;

use cstruct::{vmt, C};

use crate::interfaces::client::entity_system::GameEntitySystem;

#[vmt]
#[derive(C)]
pub struct GameResourceService {
    __pad0x0000: [u8; 0x20],
    interface_name: *const c_char,
    __pad0x0030: [u8; 0x28],

    #[ptr]
    entity_system: *const GameEntitySystem,
}

unsafe impl Sync for GameResourceService {}
