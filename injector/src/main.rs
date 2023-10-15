use windows::{core::s, Win32::UI::WindowsAndMessaging::*};

fn main() {
    println!("Hello, world!");

    unsafe {
        MessageBoxA(None, s!("header"), s!("caption"), MB_OK);
    }
}
