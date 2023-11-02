use std::sync::OnceLock;

use anyhow::bail;
use egui_win32::InputManager;
use encryption::x;
use parking_lot::Mutex;
use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC, WNDPROC},
};

use super::window::find_window;

static WNDPROC: OnceLock<WNDPROC> = OnceLock::new();
pub static INPUT: OnceLock<Mutex<InputManager>> = OnceLock::new();

pub fn setup(window: HWND) -> anyhow::Result<()> {
    if WNDPROC
        .set(unsafe {
            std::mem::transmute(SetWindowLongPtrA(window, GWLP_WNDPROC, wndproc_hk as isize))
        })
        .is_err()
    {
        bail!(x!("WNDPROC is already initialized"));
    }

    if INPUT.set(Mutex::new(InputManager::new(window))).is_err() {
        bail!(x!("INPUT is already initialized"));
    }

    Ok(())
}

pub fn destroy() -> anyhow::Result<()> {
    let window = find_window()?;

    let Some(Some(wndproc)) = WNDPROC.get() else {
        bail!(x!("WNDPROC is not initialized"));
    };

    unsafe {
        SetWindowLongPtrA(window, GWLP_WNDPROC, *wndproc as isize);
    };

    Ok(())
}

extern "system" fn wndproc_hk(window: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    log::debug!("wndproc_hk");
    let mut input = INPUT.get().expect(&x!("INPUT is not initialized")).lock();
    log::debug!("wndproc got input");

    input.process(msg, wparam.0, lparam.0);

    let wndproc = WNDPROC.get().expect(&x!("WNDPROC is not initialized"));

    log::debug!("wndproc dropped input");

    unsafe { CallWindowProcW(*wndproc, window, msg, wparam, lparam) }
}
