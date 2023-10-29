use windows::Win32::{
    Foundation::HMODULE,
    System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
};

#[macro_export]
macro_rules! w {
    ($expr:expr) => {
        windows::core::PCSTR::from_raw($expr.as_ptr())
    };
}

#[macro_export]
macro_rules! xw {
    ($s:literal) => {
        windows::core::PCSTR::from_raw(encryption::x!($s).as_ptr())
    };
}

pub fn get_module_handle(module: &str) -> Result<HMODULE, windows::core::Error> {
    unsafe { GetModuleHandleA(w!(module)) }
}

pub fn get_proc_address<T>(module: HMODULE, proc_name: &str) -> Option<*const T> {
    unsafe {
        GetProcAddress(module, w!(proc_name)).map(|proc| std::mem::transmute::<_, *const T>(proc))
    }
}
