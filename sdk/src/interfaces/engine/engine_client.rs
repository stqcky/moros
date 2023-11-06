use lazy_static::lazy_static;

use crate::interfaces::create_interface;

lazy_static! {
    pub static ref ENGINE: &'static EngineClient =
        create_interface!("engine2.dll", "Source2EngineToClient0");
}

pub struct EngineClient {}
