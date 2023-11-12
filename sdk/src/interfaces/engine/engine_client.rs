use lazy_static::lazy_static;
use memory_macros::vfunc;

use crate::interfaces::create_interface;

lazy_static! {
    pub static ref ENGINE: &'static EngineClient =
        create_interface!("engine2.dll", "Source2EngineToClient0");
}

pub struct EngineClient {}

impl EngineClient {
    #[vfunc(50)]
    fn get_screen_dimensions_vfunc(&self, width: &mut i32, height: &mut i32) {}

    pub fn get_screen_dimensions(&self) -> (f32, f32) {
        let mut width = 0;
        let mut height = 0;

        self.get_screen_dimensions_vfunc(&mut width, &mut height);

        (width as f32, height as f32)
    }
}
