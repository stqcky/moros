use anyhow::Context;
use encryption_procmacro::encrypt;
use platform::windows::find_window;

mod win32;
mod dx11;
mod fonts;

#[encrypt]
pub fn setup() -> anyhow::Result<()> {
    let window = find_window().context("could not find window")?;

    fonts::setup()?;
    win32::setup(window)?;
    dx11::setup(window)?;

    Ok(())
}

pub fn destroy() -> anyhow::Result<()> {
    win32::destroy()?;
    dx11::destroy()?;

    Ok(())
}
