use std::{ffi::c_char, marker::PhantomData};

use encryption::{encryption_procmacro::encrypt, x};
use memory::signature;
use memory_macros::C;

use crate::{
    ehandle::Handle,
    interfaces::engine::resource_service::RESOURCE_SERVICE,
    schema::client::{BaseEntity, PlayerController},
};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENTITY_SYSTEM: &'static GameEntitySystem<'static> = RESOURCE_SERVICE
        .entity_system()
        .expect(&x!("could not get entity system"));
}

pub struct GameEntitySystem<'a> {
    phantom: PhantomData<&'a usize>,
}

#[repr(C)]
#[derive(C)]
struct EntityEntry {
    entity: *const BaseEntity,
    __unk1: *const usize,
    handle: u32,
    __unk2: u32,
    __unk3: usize,
    name: *const c_char,
    __pad: [u8; 76],
}

impl GameEntitySystem<'_> {
    pub fn get_entity_by_index(&self, index: u32) -> Option<&BaseEntity> {
        if index > 0x7FFE || (index >> 9) > 0x3F {
            return None;
        }

        let this = unsafe { std::mem::transmute::<_, *const *const EntityEntry>(self) };

        let list =
            unsafe { std::slice::from_raw_parts(*this.add((index as usize >> 9) + 2), 1024) };

        let entry = list.get(index as usize & 0x1FF)?;

        if entry.handle & 0x7FFF != index {
            return None;
        }

        unsafe { entry.entity.as_ref() }
    }

    pub fn get_entity_by_handle<T>(&self, handle: Handle<T>) -> Option<&T> {
        let index = handle.handle & 0x7FFF;

        unsafe { std::mem::transmute::<_, Option<&T>>(self.get_entity_by_index(index)) }
    }

    #[encrypt]
    pub fn get_highest_entity_index(&self) -> u32 {
        lazy_static! {
            static ref GET_HIGHEST_ENTITY_INDEX: extern "fastcall" fn(this: *const GameEntitySystem, ret: *mut u32) -> u32 =
                signature::scan("client.dll", "E8 ? ? ? ? 8B 08 FF C1 85 C9")
                    .expect(&"could not find GetHighestEntityIndex signature")
                    .call()
                    .expect(&"GetHighestEntityIndex signature is invalid");
        };

        let mut ret = 0;
        GET_HIGHEST_ENTITY_INDEX(self, &mut ret);
        ret
    }

    pub fn entities(&self) -> GameEntitySystemIter {
        GameEntitySystemIter {
            entity_system: self,
            index: 1,
        }
    }

    pub fn players(&self) -> GameEntitySystemPlayerControllerIter {
        GameEntitySystemPlayerControllerIter {
            entity_system: self,
            index: 1,
        }
    }
}

pub struct GameEntitySystemPlayerControllerIter<'a> {
    entity_system: &'a GameEntitySystem<'a>,
    index: u32,
}

impl<'a> Iterator for GameEntitySystemPlayerControllerIter<'a> {
    type Item = &'a PlayerController;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: replace with value from global vars.
        while self.index <= 64 {
            if let Some(entity) = self.entity_system.get_entity_by_index(self.index) {
                self.index += 1;

                return Some(unsafe { std::mem::transmute::<_, &PlayerController>(entity) });
            }

            self.index += 1;
        }

        None
    }
}

pub struct GameEntitySystemIter<'a> {
    entity_system: &'a GameEntitySystem<'a>,
    index: u32,
}

impl<'a> Iterator for GameEntitySystemIter<'a> {
    type Item = &'a BaseEntity;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index <= self.entity_system.get_highest_entity_index() {
            if let Some(entity) = self.entity_system.get_entity_by_index(self.index) {
                self.index += 1;

                return Some(entity);
            }

            self.index += 1;
        }

        None
    }
}
