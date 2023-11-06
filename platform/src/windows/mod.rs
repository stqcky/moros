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

use crate::module::{PlatformModule, PlatformModuleInfo};
use crate::Result;

use self::strings::windows_string;

pub struct WindowsModule(pub HMODULE);

impl From<HMODULE> for WindowsModule {
    fn from(module: HMODULE) -> Self {
        Self(module)
    }
}

impl PlatformModule for WindowsModule {
    fn get_proc_address<T>(&self, proc_name: &str) -> Option<*const T> {
        let proc_name = windows_string(proc_name);

        unsafe {
            GetProcAddress(self.0, proc_name).map(|proc| std::mem::transmute::<_, *const T>(proc))
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

pub fn get_module_handle(module_name: &str) -> Result<impl PlatformModule> {
    let module_name = windows_string(module_name);

    let module: WindowsModule = unsafe { GetModuleHandleA(module_name)?.into() };

    Ok(module)
}

pub fn message_box(header: &str, content: &str, ty: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT {
    let header = windows_string(header);
    let content = windows_string(content);

    unsafe { MessageBoxA(None, content, header, ty) }
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
