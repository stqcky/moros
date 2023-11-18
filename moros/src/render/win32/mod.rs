use std::sync::OnceLock;

use anyhow::{bail, Context};
use egui_win32::InputManager;
use encryption_procmacro::encrypt;
use parking_lot::Mutex;
use platform::windows::find_window;
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC},
};

static WNDPROC: OnceLock<WNDPROC> = OnceLock::new();
pub static INPUT: OnceLock<Mutex<InputManager>> = OnceLock::new();

#[encrypt]
pub fn setup(window: HWND) -> anyhow::Result<()> {
    #[allow(clippy::fn_to_numeric_cast)]
    if WNDPROC
        .set(unsafe {
            std::mem::transmute(SetWindowLongPtrA(window, GWLP_WNDPROC, wndproc_hk as isize))
        })
        .is_err()
    {
        bail!("WNDPROC is already initialized");
    }

    if INPUT.set(Mutex::new(InputManager::new(window))).is_err() {
        bail!("INPUT is already initialized");
    }

    Ok(())
}

#[encrypt]
pub fn destroy() -> anyhow::Result<()> {
    let window = find_window().context("could not find window")?;

    let Some(Some(wndproc)) = WNDPROC.get() else {
        bail!("WNDPROC is not initialized");
    };

    #[allow(clippy::fn_to_numeric_cast)]
    unsafe {
        SetWindowLongPtrA(window, GWLP_WNDPROC, *wndproc as isize);
    };

    Ok(())
}

#[encrypt]
extern "system" fn wndproc_hk(window: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    INPUT
        .get()
        .expect(&"INPUT is not initialized")
        .lock()
        .process(msg, wparam.0, lparam.0);

    let wndproc = WNDPROC.get().expect(&"WNDPROC is not initialized");

    unsafe { CallWindowProcW(*wndproc, window, msg, wparam, lparam) }
}
