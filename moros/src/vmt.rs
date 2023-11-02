// TODO: probably shouldn't clone.

#[macro_export]
macro_rules! vfunction {
    ($vmt:expr, $index:expr) => {
        unsafe {
            std::mem::transmute(
                (*std::mem::transmute::<_, *const *const usize>($vmt.clone()))
                    .offset($index)
                    .read(),
            )
        }
    };
}
