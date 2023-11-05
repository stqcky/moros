use std::ffi::c_char;

use cstruct::C;

use crate::{
    ehandle::Handle,
    schema::client::{BaseEntity, PlayerController},
};
use std::borrow::Cow;

pub struct GameEntitySystem {}

#[repr(C)]
#[derive(C)]
struct EntityEntry {
    entity: *const BaseEntity,
    __unk1: *const usize,
    handle: u32,
    __unk2: u32,
    __unk3: usize,
    #[str]
    name: *const c_char,
    __pad: [u8; 76],
}

impl GameEntitySystem {
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

    pub fn get_highest_entity_index(&self) -> u32 {
        // static GET_HIGHEST_ENTITY_INDEX = 
    }

    pub fn players(&self) -> GameEntitySystemPlayerControllerIter {
        GameEntitySystemPlayerControllerIter {
            entity_system: self,
            index: 1,
        }
    }
}

pub struct GameEntitySystemPlayerControllerIter<'a> {
    entity_system: &'a GameEntitySystem,
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
