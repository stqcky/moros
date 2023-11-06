use egui::{Context, Rect, Window};
use encryption_procmacro::encrypt;
use glam::{Mat4, Vec2, Vec3, Vec4, Vec4Swizzles};
use lazy_static::lazy_static;
use memory::signature;
use sdk::{interfaces::client::entity_system::ENTITY_SYSTEM, math::vector::Vector};

use crate::cheat;

#[derive(Default)]
pub struct State {}

fn world_to_screen(position: &Vec3, screen: Rect) -> Option<Vec2> {
    let matrix_address = 0x7ffbae93a0d0 as *const Mat4;
    let matrix = unsafe { *matrix_address };

    let matrix = matrix.transpose();

    // 48 89 74 24 10 57 48 83 EC 20 48 8B 05 ?? ?? ?? ?? 48 8B FA
    // RenderGameSystem in World2Screen
    // mov rax, cs:RenderGameSystem

    let clip_space = matrix * Vec4::new(position.x, position.y, position.z, 1.0);

    if clip_space.w < 0.01 {
        return None;
    }

    let ndc_space = clip_space.xy() / clip_space.w;

    let view_offset = Vec2::new(screen.width() / 2.0, screen.height() / 2.0);

    let window_space = Vec2::new(
        view_offset.x + ndc_space.x / 2.0 * screen.width(),
        view_offset.y - ndc_space.y / 2.0 * screen.height(),
    );

    Some(window_space)
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

            let Some(screen) = world_to_screen(&pawn.old_origin(), ctx.screen_rect()) else {
                continue;
            };

            ui.label(format!("{} {}", screen.x, screen.y,));

            let painter = ctx.debug_painter();

            let Some(collision) = pawn.p_collision() else {
                continue;
            };

            let origin = pawn.old_origin();

            let min = origin + collision.mins();
            let max = origin + collision.maxs();

            let Some(min) = world_to_screen(&min, ctx.screen_rect()) else {
                continue;
            };

            let Some(max) = world_to_screen(&max, ctx.screen_rect()) else {
                continue;
            };

            ui.label(format!("{} {} - {} {}", min.x, min.y, max.x, max.y));

            painter.rect_stroke(
                egui::Rect {
                    min: egui::pos2(min.x, max.y),
                    max: egui::pos2(max.x, min.y),
                },
                0.0,
                egui::Stroke::new(3.0, egui::Color32::RED),
            );

            // if let Some(screen) = screen {
            //     ui.label(format!("{} {}", screen.x, screen.y));
            // }

            // ui.separator();
        }
    });
}
