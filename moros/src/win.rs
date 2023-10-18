#[macro_export]
/// Creates a PCSTR from string.
macro_rules! w {
    ($s:expr) => {
        windows::core::PCSTR::from_raw($s.as_ptr())
    };
}
