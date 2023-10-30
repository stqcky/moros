#[macro_export]
macro_rules! hook {
    ($hook: ident, $original_fn: ident, $hook_fn: ident) => {
        let __hook = unsafe { retour::GenericDetour::new($original_fn, $hook_fn) }.context(
            format!("{} {}", encryption::x!("could not hook"), stringify!($hook)),
        )?;

        unsafe {
            __hook.enable().context(format!(
                "{} {} {}",
                encryption::x!("could not enable"),
                stringify!($hook),
                encryption::x!("hook")
            ))
        }?;

        $hook.set(__hook).expect(&format!(
            "{}: {}",
            encryption::x!("could not set global hook"),
            stringify!($hook)
        ));
    };
}

// TODO: encrypt hook names.

#[macro_export]
macro_rules! unhook {
    ($hook: ident) => {
        if let Some(hook) = $hook.get() {
            unsafe {
                hook.disable().context(format!(
                    "{}: {}",
                    encryption::x!("could not disable hook"),
                    stringify!($hook)
                ))
            }?
        } else {
            anyhow::bail!(
                "{}: {}",
                encryption::x!("could not get global hook"),
                stringify!($hook)
            )
        }
    };
}
