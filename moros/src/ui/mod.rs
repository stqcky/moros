use egui::{Context, Window};
use encryption_procmacro::encrypt;
use lazy_static::lazy_static;
use memory::signature;
use sdk::{interfaces::client::entity_system::ENTITY_SYSTEM, math::vector::Vector};

use crate::cheat;

#[derive(Default)]
pub struct State {}

fn world_to_screen(position: &Vector) -> Vector {
    lazy_static! {
        static ref WORLD_TO_SCREEN: extern "system" fn(*const Vector, *mut Vector) -> bool =
            signature::scan("client.dll\0", "E8 ? ? ? ? F3 0F 10 45 18")
                .expect("could not find w2s")
                .call()
                .expect("invalid w2s signature");
    }

    let mut screen = Vector {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    WORLD_TO_SCREEN(position, &mut screen);

    screen
}

// #[encrypt]
pub fn render(ctx: &Context, state: &mut State) {
    Window::new("moros").show(ctx, |ui| {
        ui.label("hello world");

        if ui.button("unload").clicked() {
            cheat::unload();
        }

        for player in ENTITY_SYSTEM.players() {
            ui.label(player.player_name());

            let Some(pawn) = player.pawn() else {
                continue;
            };

            ui.label(format!(
                "{} {} {}",
                pawn.old_origin().x,
                pawn.old_origin().y,
                pawn.old_origin().z
            ));

            let screen = world_to_screen(&pawn.old_origin());

            ui.label(format!("{} {} {}", screen.x, screen.y, screen.z));

            ui.separator();
        }
    });
}
