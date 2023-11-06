use crate::Result;

pub trait PlatformModule {
    fn get_proc_address<T, S: AsRef<str>>(&self, proc_name: S) -> Option<*const T>;
    fn free_library_and_exit_thread(&self, exit_code: u32);
    fn get_module_info(&self) -> Result<PlatformModuleInfo>;
}

pub struct PlatformModuleInfo {
    pub base: usize,
    pub entrypoint: usize,
    pub size: u32,
}
