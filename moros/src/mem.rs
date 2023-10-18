pub unsafe fn slice_from_fn_ptr<'a>(
    fn_ptr: unsafe extern "system" fn() -> isize,
    len: usize,
) -> &'a [u8] {
    let ptr: *const u8 = std::mem::transmute(fn_ptr);
    std::slice::from_raw_parts(ptr, len)
}
