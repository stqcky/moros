pub mod vulkan;
pub mod window;

pub fn setup() -> anyhow::Result<()> {
    window::setup()?;
    vulkan::setup()?;

    Ok(())
}
