use encryption::x;
use lazy_static::lazy_static;
use thiserror::Error;
use windows::{
    core::{s, PCSTR},
    Win32::{Foundation::HMODULE, System::LibraryLoader::GetModuleHandleA},
};

use crate::{
    interfaces::{Interfaces, InterfacesInitError},
    w,
};

lazy_static! {
    static ref CLIENT_MODULE: String = x!("client.dll\0");
}

pub struct ClientModule<'a> {
    module: HMODULE,
    pub interfaces: Interfaces<'a>,
}

#[derive(Error, Debug)]
pub enum ClientModuleInitError {
    #[error("{}{0}", x!("could not get client.dll module handle: "))]
    CouldNotGetModuleHandle(#[from] windows::core::Error),

    #[error("{}{0}", x!("could not initialize client interfaces: "))]
    CouldNotInitInterfaces(#[from] InterfacesInitError),
}

impl<'a> ClientModule<'_> {
    pub fn new() -> Result<Self, ClientModuleInitError> {
        let module = unsafe { GetModuleHandleA(w!(CLIENT_MODULE))? };
        let interfaces = Interfaces::new(module)?;

        Ok(Self { module, interfaces })
    }

    fn init_interfaces(&self) {}
}
