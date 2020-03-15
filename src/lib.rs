#[cfg(not(windows))]
compile_error!("This crate only works on Windows.");

pub fn wmemchr(needle: u16, haystack: &[u16]) -> Option<usize> {
    extern "C" {
        fn wmemchr(s: *const u16, c: u16, n: usize) -> *mut u16;
    }
    let len = haystack.len();
    let ptr = haystack.as_ptr();
    let p = unsafe { wmemchr(ptr, needle, len) };
    if p.is_null() {
        None
    } else {
        Some((p as usize - ptr as usize) / 2)
    }
}
