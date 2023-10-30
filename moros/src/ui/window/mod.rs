use windows::Win32::{
    Foundation::{BOOL, FALSE, HWND, LPARAM, TRUE},
    System::{Console::GetConsoleWindow, Threading::GetCurrentProcessId},
    UI::WindowsAndMessaging::{
        EnumWindows, GetWindow, GetWindowThreadProcessId, IsWindowVisible, GW_OWNER,
    },
};

unsafe fn is_main_window(window: HWND) -> bool {
    GetWindow(window, GW_OWNER).0 == 0 && IsWindowVisible(window).into()
}

unsafe extern "system" fn enum_window(window: HWND, lparam: LPARAM) -> BOOL {
    let mut window_proc_id = 0;
    let _ = GetWindowThreadProcessId(window, Some(&mut window_proc_id));

    if GetCurrentProcessId() != window_proc_id
        || !is_main_window(window)
        || window == GetConsoleWindow()
    {
        return TRUE;
    }

    let lparam = std::mem::transmute::<_, *mut HWND>(lparam);

    *lparam = window;

    FALSE
}

pub fn setup() -> anyhow::Result<()> {
    let mut hwnd: HWND = Default::default();

    let _ = unsafe { EnumWindows(Some(enum_window), LPARAM(&mut hwnd as *mut HWND as isize)) };

    log::info!("found window: {}", hwnd.0);

    Ok(())
}
