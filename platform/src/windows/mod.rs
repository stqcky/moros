mod strings;

use windows::Win32::{
    Foundation::{BOOL, FALSE, HMODULE, HWND, LPARAM, TRUE},
    System::{
        Console::{AllocConsole, FreeConsole, GetConsoleWindow},
        LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA, GetProcAddress},
        ProcessStatus::{GetModuleInformation, MODULEINFO},
        Threading::{GetCurrentProcess, GetCurrentProcessId},
    },
    UI::WindowsAndMessaging::{
        EnumWindows, GetWindow, GetWindowThreadProcessId, IsWindowVisible, MessageBoxA, GW_OWNER,
        MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
    },
};

use crate::Result;
use crate::{
    module::{PlatformModule, PlatformModuleInfo},
    win_string,
};

pub struct WindowsModule(pub HMODULE);

impl From<HMODULE> for WindowsModule {
    fn from(module: HMODULE) -> Self {
        Self(module)
    }
}

impl PlatformModule for WindowsModule {
    fn get_proc_address<T, S>(&self, proc_name: S) -> Option<*const T>
    where
        S: AsRef<str>,
    {
        unsafe {
            GetProcAddress(self.0, win_string!(proc_name.as_ref()))
                .map(|proc| std::mem::transmute::<_, *const T>(proc))
        }
    }

    fn free_library_and_exit_thread(&self, exit_code: u32) {
        unsafe { FreeLibraryAndExitThread(self.0, exit_code) }
    }

    fn get_module_info(&self) -> Result<PlatformModuleInfo> {
        let mut module_info = MODULEINFO::default();

        unsafe {
            GetModuleInformation(
                GetCurrentProcess(),
                self.0,
                &mut module_info,
                std::mem::size_of::<MODULEINFO>() as u32,
            )?
        }

        Ok(PlatformModuleInfo {
            base: module_info.lpBaseOfDll as usize,
            entrypoint: module_info.EntryPoint as usize,
            size: module_info.SizeOfImage,
        })
    }
}

pub fn get_module_handle<T>(module_name: T) -> Result<impl PlatformModule>
where
    T: AsRef<str>,
{
    let module: WindowsModule =
        unsafe { GetModuleHandleA(win_string!(module_name.as_ref()))?.into() };

    Ok(module)
}

pub fn message_box(header: &str, content: &str, ty: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT {
    unsafe { MessageBoxA(None, win_string!(header), win_string!(content), ty) }
}

pub fn alloc_console() -> Result<()> {
    Ok(unsafe { AllocConsole()? })
}

pub fn free_console() -> Result<()> {
    Ok(unsafe { FreeConsole()? })
}

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

pub fn find_window() -> Option<HWND> {
    let mut hwnd: HWND = Default::default();

    let _ = unsafe { EnumWindows(Some(enum_window), LPARAM(&mut hwnd as *mut HWND as isize)) };

    if hwnd.0 == 0 {
        None
    } else {
        Some(hwnd)
    }
}
