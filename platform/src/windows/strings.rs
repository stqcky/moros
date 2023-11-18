#[macro_export]
macro_rules! win_string {
    ($s:expr) => {
        windows::core::PCSTR::from_raw(
            std::ffi::CString::new($s)
                .unwrap()
                .as_bytes_with_nul()
                .as_ptr(),
        )
    };
}
