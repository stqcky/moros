use std::ffi::{c_char, c_void};

use anyhow::{bail, Context};
use encryption::encryption_procmacro::encrypt;
use iced_x86::{DecoderOptions, Mnemonic, OpKind, Register};
use lazy_static::lazy_static;
use memory::signature;
use memory_macros::C;

#[encrypt]
fn find_global_vars() -> anyhow::Result<&'static GlobalVars> {
    // 48 8B 05 ? ? ? ? mov rax, cs:globals
    let func = signature::scan("client.dll", "E8 ? ? ? ? 0F B6 74 24 60")
        .context("could not find global vars signature")?
        .call::<*const u8>()
        .context("global vars signature is invalid")?;

    let mut decoder = iced_x86::Decoder::with_ip(
        64,
        unsafe { std::slice::from_raw_parts(func, 7) },
        func as u64,
        DecoderOptions::NONE,
    );

    let instruction = decoder.decode();

    if instruction.mnemonic() != Mnemonic::Mov
        || instruction.op0_register() != Register::RAX
        || instruction.op1_kind() != OpKind::Memory
    {
        bail!(
            "encountered invalid instruction, expected `mov rax, cs:globals`, got `{}`",
            instruction
        );
    }

    let globals = instruction
        .virtual_address(1, 0, |_, _, _| Some(0))
        .context("could not get global vars virtual address")?
        as *const *const GlobalVars;

    unsafe { globals.read().as_ref().context("global vars pointer is null") }
}

#[encrypt]
lazy_static! {
    pub static ref GLOBAL_VARS: &'static GlobalVars =
        find_global_vars().expect(&"could not find global vars");
}

#[repr(C)]
#[derive(C)]
pub struct GlobalVars {
    pub real_time: f32,
    pub frame_count: i32,
    pub absolute_frame_time: f32,
    pub absolute_frame_start_times: f32,
    pub max_clients: i32,
    pub interval_per_tick: f32,
    __pad1: [u8; 0x12],
    pub cur_time: f32,
    pub frame_time: f32,
    __pad2: [u8; 12],
    pub tick_count: i32,
    __pad3: [u8; 4],
    pub net_channel: *const c_void,
    __pad4: [u8; 0x130],
    pub map_full_name: *const c_char,
    pub map_name: *const c_char,
}

unsafe impl Sync for GlobalVars {}
