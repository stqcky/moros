use crate::platform::get_proc_address;
use anyhow::{anyhow, bail, Context};
use encryption::x;
use iced_x86::{DecoderOptions, Mnemonic, OpKind, Register};
use lazy_static::lazy_static;
use windows::Win32::Foundation::HMODULE;

use self::item::InterfaceRegistryItem;

mod item;

lazy_static! {
    static ref CREATE_INTERFACE_PROC_NAME: String = x!("CreateInterface\0");
}

pub struct InterfaceRegistry<'a> {
    curr: Option<&'a InterfaceRegistryItem>,
}

impl InterfaceRegistry<'_> {
    pub fn new(module: HMODULE) -> anyhow::Result<Self> {
        let interface = Self::find_interface_registry(module)?;

        Ok(Self {
            curr: Some(interface),
        })
    }

    pub fn create<T>(&mut self, name: &str) -> anyhow::Result<&'static T> {
        let interface = self
            .find(name)
            .ok_or(anyhow!("{}", x!("interface not found")))?;

        Ok(interface
            .instantiate()
            .context(x!("could not instantiate interface"))?)
    }

    pub fn find(&mut self, name: &str) -> Option<&InterfaceRegistryItem> {
        Iterator::find(self, |interface| interface.name().contains(name))
    }

    fn find_interface_registry(module: HMODULE) -> anyhow::Result<&'static InterfaceRegistryItem> {
        // mov r9, [interface_list]
        const INSTRUCTION_SIZE: usize = 8;

        let create_interface = get_proc_address(module, &CREATE_INTERFACE_PROC_NAME)
            .map(|proc| unsafe { std::slice::from_raw_parts(proc, INSTRUCTION_SIZE) })
            .ok_or(anyhow!("{}", x!("CreateInterface procedure not found")))?;

        let mut decoder = iced_x86::Decoder::with_ip(
            64,
            create_interface,
            create_interface.as_ptr() as u64,
            DecoderOptions::NONE,
        );

        let instruction = decoder.decode();

        if instruction.mnemonic() != Mnemonic::Mov
            || instruction.op_register(0) != Register::R9
            || instruction.op_kind(1) != OpKind::Memory
        {
            bail!("{}", x!("unexpected instruction in CreateInterface"))
        }

        let interface_list_ptr = instruction
            .virtual_address(1, 0, |_, _, _| Some(0))
            .ok_or(anyhow!("{}", x!("could not get virtual address")))?
            as *const *const InterfaceRegistryItem;

        let interface = unsafe {
            interface_list_ptr
                .read()
                .as_ref()
                .ok_or(anyhow!("{}", x!("interface is null")))?
        };

        Ok(interface)
    }
}

impl<'a> Iterator for InterfaceRegistry<'a> {
    type Item = &'a InterfaceRegistryItem;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr?;

        self.curr = current.next();

        Some(current)
    }
}
