use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Stroke};
use glam::{Vec2, Vec3};
use sdk::{
    interfaces::client::{client::CLIENT, entity_system::ENTITY_SYSTEM},
    schema::{client::PlayerController, global::LifeState},
};

use crate::settings::EspSettings;

struct EspBuilder<'a> {
    player: &'a PlayerController,

    mins: Pos2,
    maxs: Pos2,

    top_offset: f32,
    bottom_offset: f32,

    left_offset: egui::Vec2,
    right_offset: egui::Vec2,
}

impl<'a> EspBuilder<'a> {
    const BOX_SIZE: f32 = 3.0;
    const BAR_SIZE: f32 = 3.0;
    const SPACING: f32 = 3.0;

    pub fn new(player: &'a PlayerController) -> Option<Self> {
        let bounds = Self::get_player_bounds(player)?;

        Some(Self {
            player,

            mins: bounds.0,
            maxs: bounds.1,

            top_offset: Self::SPACING,
            bottom_offset: Self::SPACING,

            left_offset: egui::Vec2::new(Self::SPACING, 0.0),
            right_offset: egui::Vec2::new(Self::SPACING, 0.0),
        })
    }

    fn get_player_bounds(player: &PlayerController) -> Option<(Pos2, Pos2)> {
        let pawn = player.pawn()?;
        let collision = pawn.p_collision()?;

        let scene_node = pawn.body_component()?.scene_node()?;

        let origin = scene_node.abs_origin();

        let mins = origin + collision.mins();
        let maxs = origin + collision.maxs();

        // TODO: use this
        // let transform = scene_node.node_to_world().affine();

        Some(
            [
                Vec3::new(mins.x, mins.y, mins.z),
                Vec3::new(maxs.x, mins.y, mins.z),
                Vec3::new(maxs.x, maxs.y, mins.z),
                Vec3::new(mins.x, maxs.y, mins.z),
                Vec3::new(mins.x, mins.y, maxs.z),
                Vec3::new(maxs.x, mins.y, maxs.z),
                Vec3::new(maxs.x, maxs.y, maxs.z),
                Vec3::new(mins.x, maxs.y, maxs.z),
            ]
            .iter()
            .map(|point| CLIENT.world_to_screen(&point))
            .collect::<Option<Vec<Vec2>>>()?
            .iter()
            .fold(
                (Pos2::new(f32::MAX, f32::MAX), Pos2::ZERO),
                |mut bound, point| {
                    bound.0.x = bound.0.x.min(point.x);
                    bound.0.y = bound.0.y.min(point.y);

                    bound.1.x = bound.1.x.max(point.x);
                    bound.1.y = bound.1.y.max(point.y);

                    bound
                },
            ),
        )
    }

    pub fn draw_box(&mut self, painter: &Painter, color: Color32) {
        painter.rect_stroke(
            Rect {
                min: self.mins + egui::Vec2::new(1.0, 1.0),
                max: self.maxs - egui::Vec2::new(1.0, 1.0),
            },
            0.0,
            Stroke::new(Self::BOX_SIZE, Color32::BLACK),
        );

        painter.rect_stroke(
            Rect {
                min: self.mins + egui::Vec2::new(1.0, 1.0),
                max: self.maxs - egui::Vec2::new(1.0, 1.0),
            },
            0.0,
            Stroke::new(1.0, color),
        );
    }

    fn get_font(&self) -> FontId {
        FontId {
            size: 1.0,
            ..Default::default()
        }
    }

    pub fn text_top(&mut self, painter: &Painter, text: impl ToString, color: Color32) {
        let center_x = (self.mins.x + self.maxs.x) / 2.0;

        let rect = painter.text(
            Pos2::new(center_x, self.mins.y - self.top_offset),
            Align2::CENTER_BOTTOM,
            text,
            self.get_font(),
            color,
        );

        self.top_offset += rect.height() + Self::SPACING;
    }

    pub fn text_bottom(&mut self, painter: &Painter, text: impl ToString, color: Color32) {
        let center_x = (self.mins.x + self.maxs.x) / 2.0;

        let rect = painter.text(
            Pos2::new(center_x, self.maxs.y + self.bottom_offset),
            Align2::CENTER_TOP,
            text,
            self.get_font(),
            color,
        );

        self.bottom_offset += rect.height() + Self::SPACING;
    }

    pub fn text_left(&mut self, painter: &Painter, text: impl ToString, color: Color32) {
        let rect = painter.text(
            Pos2::new(self.mins.x, self.mins.y) - self.left_offset,
            Align2::RIGHT_TOP,
            text,
            self.get_font(),
            color,
        );

        self.left_offset.y += rect.height() + Self::SPACING;
    }

    pub fn text_right(&mut self, painter: &Painter, text: impl ToString, color: Color32) {
        let rect = painter.text(
            Pos2::new(self.maxs.x, self.mins.y) + self.right_offset,
            Align2::LEFT_TOP,
            text,
            self.get_font(),
            color,
        );

        self.right_offset.y += rect.height() + Self::SPACING;
    }

    pub fn bar_left(&mut self, painter: &Painter, percentage: f32, color: Color32) {
        let left = egui::Pos2::new(self.mins.x - self.left_offset.x, self.mins.y);

        painter.rect_filled(
            Rect {
                min: left - egui::Vec2::new(Self::BAR_SIZE, 1.0),
                max: egui::Pos2::new(left.x, self.maxs.y),
            },
            0.0,
            Color32::BLACK,
        );

        let height = (self.maxs.y - self.mins.y) * percentage.clamp(0.0, 1.0);

        painter.vline(
            left.x - 1.0,
            self.maxs.y - height..=self.maxs.y - 1.0,
            Stroke::new(1.0, color),
        );

        self.left_offset.x += Self::BAR_SIZE + Self::SPACING;
    }
}

pub fn render(painter: Painter, settings: &EspSettings) {
    if !settings.enabled {
        return;
    }

    let builders = ENTITY_SYSTEM.players().filter_map(|player| {
        let pawn = player.pawn()?;

        if player.is_local_player_controller() || pawn.life_state() != LifeState::Alive {
            None
        } else {
            EspBuilder::new(player)
        }
    });

    for mut builder in builders {
        if settings.draw_boxes {
            builder.draw_box(&painter, settings.box_color);
        }

        if settings.draw_nametags {
            builder.text_top(&painter, builder.player.player_name(), Color32::WHITE);
        }

        if settings.draw_money {
            let Some(money_services) = builder.player.in_game_money_services() else {
                continue;
            };

            builder.text_right(
                &painter,
                format!("${}", money_services.account()),
                Color32::GREEN,
            );
        }

        if settings.draw_health {
            let Some(pawn) = builder.player.pawn() else {
                continue;
            };

            builder.bar_left(
                &painter,
                pawn.health() as f32 / pawn.max_health() as f32,
                Color32::GREEN,
            );

            builder.text_left(&painter, pawn.health(), Color32::WHITE);
        }
    }
}
