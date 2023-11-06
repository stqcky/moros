use anyhow::{anyhow, bail, Context};
use encryption::{encryption_procmacro::encrypt, x};
use iced_x86::{DecoderOptions, Mnemonic, OpKind, Register};
use lazy_static::lazy_static;
use platform::module::PlatformModule;

use self::item::InterfaceRegistryItem;

mod item;

lazy_static! {
    static ref CREATE_INTERFACE_PROC_NAME: String = x!("CreateInterface");
}

pub struct InterfaceRegistry<'a> {
    curr: Option<&'a InterfaceRegistryItem>,
}

impl InterfaceRegistry<'_> {
    pub fn new(module: impl PlatformModule) -> anyhow::Result<Self> {
        let interface = Self::find_interface_registry(module)?;

        Ok(Self {
            curr: Some(interface),
        })
    }

    #[encrypt]
    pub fn create<'a, T>(&mut self, name: &str) -> anyhow::Result<&'a T> {
        let interface = self.find(name).ok_or(anyhow!("interface not found"))?;

        Ok(interface
            .instantiate()
            .context("could not instantiate interface")?)
    }

    pub fn find(&mut self, name: &str) -> Option<&InterfaceRegistryItem> {
        Iterator::find(self, |interface| interface.name().contains(name))
    }

    #[encrypt]
    fn find_interface_registry(
        module: impl PlatformModule,
    ) -> anyhow::Result<&'static InterfaceRegistryItem> {
        // mov r9, [interface_list]
        const INSTRUCTION_SIZE: usize = 8;

        let create_interface = unsafe {
            std::slice::from_raw_parts(
                module
                    .get_proc_address(&*CREATE_INTERFACE_PROC_NAME)
                    .context("could not find CreateInterface procedure")?,
                INSTRUCTION_SIZE,
            )
        };

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
            bail!("unexpected instruction in CreateInterface")
        }

        let interface_list_ptr = instruction
            .virtual_address(1, 0, |_, _, _| Some(0))
            .ok_or(anyhow!("could not get virtual address"))?
            as *const *const InterfaceRegistryItem;

        let interface = unsafe {
            interface_list_ptr
                .read()
                .as_ref()
                .ok_or(anyhow!("interface is null"))?
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
