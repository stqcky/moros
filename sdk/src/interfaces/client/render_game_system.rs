use anyhow::{bail, Context};
use glam::Mat4;
use iced_x86::{DecoderOptions, Mnemonic, OpKind, Register};
use lazy_static::lazy_static;
use memory::signature;
use memory_macros::vfunc;

fn find_render_game_system() -> anyhow::Result<&'static RenderGameSystem> {
    // getting RenderGameSystem from WorldToScreen function.
    // mov [rsp+arg_8], rsi
    // push rdi
    // sub rsp, 0x20
    // mov rax, cs:RenderGameSystem

    let func = signature::scan(
        "client.dll",
        "48 89 74 24 10 57 48 83 EC 20 48 8B 05 ? ? ? ? 48 8B FA",
    )
    .context("could not find render game system signature")?
    .absolute() as *const u8;

    let mut decoder = iced_x86::Decoder::with_ip(
        64,
        unsafe { std::slice::from_raw_parts(func, 0x11) },
        func as u64,
        DecoderOptions::NONE,
    );

    let instruction = decoder
        .iter()
        .nth(3)
        .context("render game system signature is invalid")?;

    if instruction.mnemonic() != Mnemonic::Mov
        || instruction.op0_register() != Register::RAX
        || instruction.op1_kind() != OpKind::Memory
    {
        bail!(
            "encountered invalid instruction, expected `mov rax, cs:RenderGameSystem`, got `{}`",
            instruction
        )
    }

    let render_game_system = instruction
        .virtual_address(1, 0, |_, _, _| Some(0))
        .context("could not get render game system virtual address")?
        as *const *const RenderGameSystem;

    unsafe {
        render_game_system
            .read()
            .as_ref()
            .context("render game system pointer is null")
    }
}

lazy_static! {
    pub static ref RENDER_GAME_SYSTEM: &'static RenderGameSystem =
        find_render_game_system().expect("could not find render game system");
}

pub struct RenderGameSystem {}

impl RenderGameSystem {
    #[vfunc(53)]
    fn get_matrix(&self, index: u32) -> &Mat4 {}

    pub fn get_view_matrix(&self) -> Mat4 {
        let matrix = self.get_matrix(0).expect("could not get view matrix");

        matrix.transpose()
    }
}
