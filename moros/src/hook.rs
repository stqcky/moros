#[macro_export]
macro_rules! hook {
    ($hook: ident, $original_fn: ident, $hook_fn: ident) => {
        (|| -> anyhow::Result<()> {
            let hook = unsafe { retour::GenericDetour::new($original_fn, $hook_fn) }
                .context(encryption::x!("could not hook"))?;

            unsafe {
                hook.enable()
                    .context(encryption::x!("could not enable hook"))
            }?;

            $hook
                .set(hook)
                .expect(&encryption::x!("could not set global hook"));

            Ok(())
        })()
    };
}

#[macro_export]
macro_rules! unhook {
    ($hook: ident) => {
        if let Some(hook) = $hook.get() {
            unsafe {
                hook.disable()
                    .context(encryption::x!("could not disable hook"))
            }
        } else {
            anyhow::bail!(encryption::x!("could not get global hook"))
        }
    };
}
