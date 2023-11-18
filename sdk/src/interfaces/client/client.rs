use glam::{Vec2, Vec3, Vec4, Vec4Swizzles};
use lazy_static::lazy_static;
use memory::signature;

use crate::{
    interfaces::{create_interface, engine::engine_client::ENGINE},
    schema::client::PlayerController,
};

use super::render_game_system::RENDER_GAME_SYSTEM;

lazy_static! {
    pub static ref CLIENT: &'static Source2Client =
        create_interface!("client.dll", "Source2Client0");
}

pub struct Source2Client {}

impl Source2Client {
    pub fn world_to_screen(&self, position: &Vec3) -> Option<Vec2> {
        let matrix = RENDER_GAME_SYSTEM.get_view_matrix();

        let clip_space = matrix * Vec4::new(position.x, position.y, position.z, 1.0);

        if clip_space.w < 0.01 {
            return None;
        }

        let ndc_space = clip_space.xy() / clip_space.w;

        let (screen_width, screen_height) = ENGINE.get_screen_dimensions();

        let view_offset = Vec2::new(screen_width / 2.0, screen_height / 2.0);

        let window_space = Vec2::new(
            view_offset.x + ndc_space.x / 2.0 * screen_width,
            view_offset.y - ndc_space.y / 2.0 * screen_height,
        );

        Some(window_space.round())
    }

    pub fn get_local_player_controller(&self) -> Option<&PlayerController> {
        let func: extern "fastcall" fn(index: i32) -> *const PlayerController =
            signature::scan("client.dll", "E8 ? ? ? ? 48 85 C0 74 12 8B CB")
                .expect(&"local player controller signature not found")
                .call()
                .expect(&"local player controller signature is invalid");

        unsafe { func(0).as_ref() }
    }
}
