use egui::{Context, Window};

use crate::cheat;

use self::window::find_window;

mod dx11;
mod win32;
mod window;

#[derive(Default)]
pub struct State {}

pub fn setup() -> anyhow::Result<()> {
    let window = find_window()?;

    win32::setup(window)?;
    dx11::setup(window)?;

    Ok(())
}

pub fn render(ctx: &Context, state: &mut State) {
    Window::new("moros").show(ctx, |ui| {
        ui.label("hello world");

        if ui.button("unload").clicked() {
            cheat::unload();
        }
    });
}

pub fn destroy() -> anyhow::Result<()> {
    win32::destroy()?;
    dx11::destroy()?;

    Ok(())
}
