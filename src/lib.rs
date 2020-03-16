#[cfg(not(windows))]
compile_error!("This crate only works on Windows.");

#[cfg(target_env = "msvc")]
#[cfg(not(target_feature = "crt-static"))]
compile_error!("wmemchr in MSVC requires the C runtime to be statically linked");

pub fn wmemchr(needle: u16, haystack: &[u16]) -> Option<usize> {
    extern "C" {
        #[cfg(target_env = "gnu")]
        fn wmemchr(s: *const u16, c: u16, n: usize) -> *mut u16;
    }

    extern "C" {
        #[cfg(target_env = "msvc")]
        // FIXME: Hopefully this will fix the "external symbol wmemchr" on MSVC toolchain
        //        as clang include a builtin function for wmemchr.
        //
        // Try to uncomment the line below in case wmemchr symbol cannot be resolved.
        // #[link_name = "__builtin_wmemchr"]
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
