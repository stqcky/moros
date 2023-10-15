use std::{borrow::Cow, ffi::c_char};
use thiserror::Error;

pub mod client;

#[repr(C)]
pub struct Interface {
    get: fn() -> *mut (),
    name: *const c_char,
    next: *mut Interface,
}

impl Interface {
    pub fn create<T>(&self) -> *mut T {
        (self.get)().cast()
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
    #[error("could not initialize interface list: received null pointer")]
    NullPointer,
}

impl Interfaces<'_> {
    fn new(interfaces_ptr: *const *const Interface) -> Result<Self, InterfacesInitError> {
        let first: &Interface = unsafe {
            (*interfaces_ptr)
                .as_ref()
                .ok_or(InterfacesInitError::NullPointer)?
        };

        Ok(Self::from(first))
    }

    fn find(&mut self, name: &str) -> Option<&Interface> {
        Iterator::find(self, |interface| interface.name() == name)
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
