use anyhow::{bail, Context};
use encryption_procmacro::encrypt;
use iced_x86::{DecoderOptions, Mnemonic, OpKind};
use platform::module::PlatformModule;
use platform::windows::get_module_handle;
use regex::bytes::Regex;

#[encrypt]
fn create_regex(pattern: &str) -> anyhow::Result<Regex> {
    let mut pattern = pattern
        .to_string()
        .split_whitespace()
        .map(|x| {
            if x == "?" {
                ".".to_string()
            } else {
                format!("\\x{x}")
            }
        })
        .collect::<Vec<_>>()
        .join("");

    pattern.insert_str(0, &"(?s-u)");

    Ok(Regex::new(&pattern).context("could not create signature regex pattern")?)
}

#[encrypt]
pub fn scan<T>(module: T, pattern: T) -> anyhow::Result<Signature>
where
    T: AsRef<str>,
{
    let module = get_module_handle(module.as_ref()).context("could not get module handle")?;
    let module_info = module
        .get_module_info()
        .context("could not get module info")?;

    let data = unsafe {
        std::slice::from_raw_parts(module_info.base as *const u8, module_info.size as usize)
    };

    let address = create_regex(pattern.as_ref())?
        .find(data)
        .context("could not find pattern")?
        .start() as usize;

    Ok(Signature {
        address,
        rip: address,
        module_base: module_info.base,
    })
}

#[derive(Debug)]
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

    pub fn absolute(&self) -> usize {
        self.address + self.module_base
    }

    #[encrypt]
    pub fn call<T>(&self) -> anyhow::Result<T> {
        let absolute = (self.module_base + self.address) as *const u8;
        let instruction = unsafe { std::slice::from_raw_parts(absolute, 5) };

        let mut decoder =
            iced_x86::Decoder::with_ip(64, instruction, self.rip as u64, DecoderOptions::NONE);

        let instruction = decoder.decode();

        if instruction.mnemonic() != Mnemonic::Call
            || instruction.op0_kind() != OpKind::NearBranch64
        {
            bail!("expected call instruction, got: {}", instruction);
        }

        let address = instruction.near_branch64() + self.module_base as u64;

        Ok(unsafe { std::mem::transmute_copy(&address) })
    }
}

// #[macro_export]
// macro_rules! scan {
//     ($module: literal, $pattern: literal, $offset: expr) => {
//         $crate::memory::signature::scan_internal(
//             &encryption::x!($module),
//             &encryption::x!($pattern),
//         )
//         .map(|x| x.offset($offset))
//     };
//     ($module: literal, $pattern: literal) => {
//         $crate::signature_internal::scan_internal(
//             &encryption::x!($module),
//             &encryption::x!($pattern),
//         )
//     };
// }
