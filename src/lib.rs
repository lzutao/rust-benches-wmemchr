#[cfg(not(windows))]
compile_error!("This crate only works on Windows.");

#[cfg(target_env = "gnu")]
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

#[cfg(target_env = "msvc")]
pub fn wmemchr(needle: u16, haystack: &[u16]) -> Option<usize> {
    use std::ffi::c_void;
    use std::os::raw::c_int;
    extern "C" {
        fn memchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
    }

    let [low, high] = needle.to_le_bytes();
    let mut ptr = haystack.as_ptr() as *const u8;
    let mut len = haystack.len() * 2;
    let mut start = 0;

    loop {
        let p = unsafe { memchr(ptr as *const c_void, low as c_int, len) } as *const u8;
        if p.is_null() {
            return None;
        }

        let off = p as usize - ptr as usize;
        if off >= len {
            return None;
        }

        start += off;
        if unsafe { *p.add(1) == high } {
            // assert!(start % 2 == 1);
            return Some((start + 1) / 2);
        }

        len -= off;
        ptr = unsafe { ptr.add(off) };
    }
}
