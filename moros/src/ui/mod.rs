use egui::{panel::Side, Context, SidePanel, Ui, Window};

use crate::{
    cheat,
    settings::{MiscSettings, Settings, Tab, VisualsSettings},
};

#[cfg_attr(not(debug_assertions), encrypt)]
pub fn render(ctx: &Context, settings: &mut Settings) {
    Window::new("moros").show(ctx, |ui| {
        tabs(ui, settings);

        match settings.tab {
            Tab::Visuals => visuals_tab(ui, &mut settings.visuals),
            Tab::Misc => misc_tab(ui, &mut settings.misc),
        }
    });
}

fn tabs(ui: &mut Ui, settings: &mut Settings) {
    ui.horizontal(|ui| {
        if ui
            .selectable_label(settings.tab == Tab::Visuals, "visuals")
            .clicked()
        {
            settings.tab = Tab::Visuals;
        }

        if ui
            .selectable_label(settings.tab == Tab::Misc, "misc")
            .clicked()
        {
            settings.tab = Tab::Misc;
        }
    });
}

fn visuals_tab(ui: &mut Ui, settings: &mut VisualsSettings) {
    ui.label("esp");

    ui.checkbox(&mut settings.esp.enabled, "enable");

    ui.horizontal(|ui| {
        ui.checkbox(&mut settings.esp.draw_boxes, "box");
        ui.color_edit_button_srgba(&mut settings.esp.box_color);
    });

    ui.checkbox(&mut settings.esp.draw_nametags, "name");
    ui.checkbox(&mut settings.esp.draw_health, "health");
    ui.checkbox(&mut settings.esp.draw_money, "money");
}

fn misc_tab(mut ui: &mut Ui, settings: &mut MiscSettings) {
    if ui.button("unload").clicked() {
        cheat::unload();
    }
}
