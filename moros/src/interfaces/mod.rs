use encryption::x;
use iced_x86::{DecoderOptions, Mnemonic, OpKind, Register};
use lazy_static::lazy_static;
use std::{borrow::Cow, ffi::c_char};
use thiserror::Error;
use windows::{
    core::PCSTR,
    Win32::{Foundation::HMODULE, System::LibraryLoader::GetProcAddress},
};

use crate::{mem, w};

pub mod client;

lazy_static! {
    static ref CREATE_INTERFACE: String = x!("CreateInterface");
}

#[repr(C)]
pub struct Interface {
    get: *const u8,
    name: *const c_char,
    next: *const Interface,
}

#[derive(Error, Debug)]
pub enum InterfaceCreateError {
    #[error("{}", x!("interface create fn ptr is null"))]
    CreateFnNullPointer,
    #[error("{}", x!("encountered unexpected instruction"))]
    UnexpectedInstruction,
    #[error("{}", x!("unexpected interface null pointer"))]
    InterfaceNullPointer,
}

impl Interface {
    pub fn create<T>(&self) -> Result<&T, InterfaceCreateError> {
        // 48 8D 05 F9 0D 05 00 | lea rax, [rip+0x50DF9]
        // C3                   | ret

        const PROC_SIZE: usize = 8;

        let rip = self.get;

        if rip.is_null() {
            return Err(InterfaceCreateError::CreateFnNullPointer);
        }

        let create_fn_ptr = unsafe { std::slice::from_raw_parts(rip, PROC_SIZE) };

        log::info!("rip {:?}", self.get);
        log::info!("ptr {create_fn_ptr:?}");

        let mut decoder =
            iced_x86::Decoder::with_ip(64, create_fn_ptr, rip as u64, DecoderOptions::NONE);

        let load = decoder.decode();

        log::info!("{load}");

        if load.mnemonic() != Mnemonic::Lea
            || load.op_register(0) != Register::RAX
            || load.op_kind(1) != OpKind::Memory
        {
            return Err(InterfaceCreateError::UnexpectedInstruction);
        }

        let ret = decoder.decode();

        if ret.mnemonic() != Mnemonic::Ret {
            return Err(InterfaceCreateError::UnexpectedInstruction);
        }

        let interface_ptr =
            load.virtual_address(1, 0, |_, _, _| Some(0))
                .ok_or(InterfaceCreateError::UnexpectedInstruction)? as *const *const T;

        Ok(unsafe {
            (*interface_ptr)
                .as_ref()
                .ok_or(InterfaceCreateError::InterfaceNullPointer)?
        })
    }

    pub fn name(&self) -> Cow<str> {
        unsafe { std::ffi::CStr::from_ptr(self.name).to_string_lossy() }
    }

    pub fn next(&self) -> Option<&Interface> {
        unsafe { self.next.as_ref() }
    }
}

pub struct Interfaces<'a> {
    first: &'a Interface,
    curr: Option<&'a Interface>,
}

#[derive(Error, Debug)]
pub enum InterfacesInitError {
    #[error("{}", x!("could not initialize interface list: received null pointer"))]
    NullPointer,
    #[error("{}", x!("could not get CreateInterface proc address"))]
    CouldNotFindCreateInterface,
    #[error("{}", x!("invalid instruction in CreateInterface"))]
    InvalidInstruction,
}

impl Interfaces<'_> {
    pub fn new(module: HMODULE) -> Result<Self, InterfacesInitError> {
        let interface = Self::find_interface_list(module)?;

        Ok(Self::from(interface))
    }

    pub fn find(&mut self, name: &str) -> Option<&Interface> {
        Iterator::find(self, |interface| interface.name().contains(name))
    }

    fn find_interface_list(module: HMODULE) -> Result<&'static Interface, InterfacesInitError> {
        // mov r9, [interface_list]
        const INSTRUCTION_SIZE: usize = 8;

        let create_interface = unsafe {
            GetProcAddress(module, w!(CREATE_INTERFACE))
                .map(|ptr| mem::slice_from_fn_ptr(ptr, INSTRUCTION_SIZE))
        }
        .ok_or(InterfacesInitError::CouldNotFindCreateInterface)?;

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
            return Err(InterfacesInitError::InvalidInstruction);
        }

        let interface_list_ptr = instruction
            .virtual_address(1, 0, |_, _, _| Some(0))
            .ok_or(InterfacesInitError::InvalidInstruction)?
            as *const *const Interface;

        let interface = unsafe {
            (*interface_list_ptr)
                .as_ref()
                .ok_or(InterfacesInitError::NullPointer)?
        };

        Ok(interface)
    }
}

impl<'a> From<&'a Interface> for Interfaces<'a> {
    fn from(interface: &'a Interface) -> Self {
        Self {
            first: interface,
            curr: Some(interface),
        }
    }
}

impl<'a> Iterator for Interfaces<'a> {
    type Item = &'a Interface;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr?;

        self.curr = current.next();

        Some(current)
    }
}
