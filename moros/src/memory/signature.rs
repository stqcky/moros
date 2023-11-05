use anyhow::{bail, Context};
use encryption::x;
use iced_x86::{DecoderOptions, Mnemonic};
use regex::bytes::Regex;
use windows::{
    core::PCSTR,
    Win32::System::{
        LibraryLoader::GetModuleHandleA,
        ProcessStatus::{GetModuleInformation, MODULEINFO},
        Threading::GetCurrentProcess,
    },
};

fn get_module_info(module: PCSTR) -> anyhow::Result<MODULEINFO> {
    let module = unsafe { GetModuleHandleA(module).context(x!("could not get module handle"))? };

    let mut module_info = MODULEINFO::default();

    unsafe {
        GetModuleInformation(
            GetCurrentProcess(),
            module,
            &mut module_info,
            std::mem::size_of::<MODULEINFO>() as u32,
        )
        .context(x!("could not get module information"))?
    };

    Ok(module_info)
}

fn create_regex(pattern: &str) -> anyhow::Result<Regex> {
    let mut pattern = pattern
        .to_string()
        .split_whitespace()
        .map(|x| match x {
            "?" => ".".to_string(),
            _ => format!("\\x{x}"),
        })
        .collect::<Vec<_>>()
        .join("");

    pattern.insert_str(0, "(?s-u)");

    Ok(Regex::new(&pattern).context(x!("could not create signature regex pattern"))?)
}

pub fn internal_scan(module: PCSTR, pattern: &str) -> anyhow::Result<Signature> {
    let module_info = get_module_info(module)?;

    let data = unsafe {
        std::slice::from_raw_parts(
            module_info.lpBaseOfDll as *const u8,
            module_info.SizeOfImage as usize,
        )
    };

    let address = create_regex(pattern)?
        .find(data)
        .context(x!("could not find pattern"))?
        .start() as usize;

    Ok(Signature {
        address,
        rip: address,
        module_base: module_info.lpBaseOfDll as usize,
    })
}

pub struct Signature {
    pub address: usize,
    pub rip: usize,
    module_base: usize,
}

impl Signature {
    pub fn offset(mut self, offset: usize) -> Self {
        self.address += offset;
        self
    }

    pub fn call<T>(&self) -> anyhow::Result<T> {
        let absolute = (self.module_base + self.address) as *const u8;
        let instruction = unsafe { std::slice::from_raw_parts(absolute, 5) };

        let mut decoder =
            iced_x86::Decoder::with_ip(64, instruction, self.rip as u64, DecoderOptions::NONE);

        let instruction = decoder.decode();

        if instruction.mnemonic() != Mnemonic::Call {
            bail!(
                "{}: {:?}",
                x!("expected call instruction, got"),
                instruction
            );
        }

        let address = instruction.near_branch64();

        Ok(unsafe { std::mem::transmute_copy(&address) })
    }
}

#[macro_export]
macro_rules! scan {
    ($module: literal, $pattern: literal, $offset: expr) => {
        $crate::memory::signature::internal_scan(xw!($module), &encryption::x!($pattern))
            .map(|x| x.offset($offset))
    };
}
pub(crate) use scan;
