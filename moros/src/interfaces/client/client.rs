use thiserror::Error;
use windows::{
    core::{s, PCSTR},
    Win32::System::LibraryLoader::GetModuleHandleA,
};

use crate::interfaces::{Interface, Interfaces, InterfacesInitError};

const CLIENT_MODULE: PCSTR = s!("client.dll");
const INTERFACES_OFFSET: isize = 0x1976138;

pub struct Client<'a> {
    handle: isize,
    pub interfaces: Interfaces<'a>,
}

#[derive(Error, Debug)]
pub enum ClientInitError {
    #[error("could not get client.dll module handle")]
    CouldNotGetModuleHandle(#[from] windows::core::Error),

    #[error("could not initialize client interfaces")]
    CouldNotInitInterfaces(#[from] InterfacesInitError),
}

impl<'a> Client<'_> {
    pub fn new() -> Result<Self, ClientInitError> {
        let handle = unsafe { GetModuleHandleA(CLIENT_MODULE)?.0 };
        let interfaces = Interfaces::new((handle + INTERFACES_OFFSET) as *const *const Interface)?;

        Ok(Self { handle, interfaces })
    }

    fn init_interfaces(&self) {}
}
