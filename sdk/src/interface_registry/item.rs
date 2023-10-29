use std::{borrow::Cow, ffi::c_char};

use encryption::x;
use iced_x86::{DecoderOptions, Mnemonic, OpKind, Register};
use thiserror::Error;

#[repr(C)]
pub struct InterfaceRegistryItem {
    create: *const u8,
    name: *const c_char,
    next: *const InterfaceRegistryItem,
}

#[derive(Error, Debug)]
pub enum InstantiateInterfaceError {
    #[error("{}", x!("interface create fn ptr is null"))]
    CreateFnNullPointer,

    #[error("{}", x!("encountered unexpected instruction"))]
    UnexpectedInstruction,

    #[error("{}", x!("instantiated interface is null"))]
    InterfaceNullPointer,
}

impl InterfaceRegistryItem {
    pub fn instantiate<'a, T>(&self) -> Result<&'a T, InstantiateInterfaceError> {
        // 48 8D 05 F9 0D 05 00 | lea rax, [offset]
        // C3                   | ret

        const PROC_SIZE: usize = 8;

        let rip = self.create;

        if rip.is_null() {
            return Err(InstantiateInterfaceError::CreateFnNullPointer);
        }

        let create_fn_ptr = unsafe { std::slice::from_raw_parts(rip, PROC_SIZE) };

        let mut decoder =
            iced_x86::Decoder::with_ip(64, create_fn_ptr, rip as u64, DecoderOptions::NONE);

        let load = decoder.decode();

        if load.mnemonic() != Mnemonic::Lea
            || load.op_register(0) != Register::RAX
            || load.op_kind(1) != OpKind::Memory
        {
            return Err(InstantiateInterfaceError::UnexpectedInstruction);
        }

        let ret = decoder.decode();

        if ret.mnemonic() != Mnemonic::Ret {
            return Err(InstantiateInterfaceError::UnexpectedInstruction);
        }

        let interface_ptr =
            load.virtual_address(1, 0, |_, _, _| Some(0))
                .ok_or(InstantiateInterfaceError::UnexpectedInstruction)? as *const T;

        Ok(unsafe {
            interface_ptr
                .as_ref()
                .ok_or(InstantiateInterfaceError::InterfaceNullPointer)?
        })
    }

    pub fn name(&self) -> Cow<str> {
        unsafe { std::ffi::CStr::from_ptr(self.name).to_string_lossy() }
    }

    pub fn next(&self) -> Option<&InterfaceRegistryItem> {
        unsafe { self.next.as_ref() }
    }
}
