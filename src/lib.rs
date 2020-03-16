// #[cfg(not(windows))]
// compile_error!("This crate only works on Windows.");

// #[cfg(target_env = "gnu")]
// pub fn wmemchr(needle: u16, haystack: &[u16]) -> Option<usize> {
//     extern "C" {
//         fn wmemchr(s: *const u16, c: u16, n: usize) -> *mut u16;
//     }

//     let len = haystack.len();
//     let ptr = haystack.as_ptr();
//     let p = unsafe { wmemchr(ptr, needle, len) };
//     if p.is_null() {
//         None
//     } else {
//         Some((p as usize - ptr as usize) / 2)
//     }
// }

// #[cfg(target_env = "msvc")]
pub fn wmemchr(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    let mut len = haystack.len();
    let mut start = &haystack[..];

    // For performance reasons unfold the loop eight times.
    while len >= 8 {
        if start[0] == needle {
            return Some((start.as_ptr() as usize - ptr as usize) / 2);
        }
        if start[1] == needle {
            return Some((start[1..].as_ptr() as usize - ptr as usize) / 2);
        }
        if start[2] == needle {
            return Some((start[2..].as_ptr() as usize - ptr as usize) / 2);
        }
        if start[3] == needle {
            return Some((start[3..].as_ptr() as usize - ptr as usize) / 2);
        }
        if start[4] == needle {
            return Some((start[4..].as_ptr() as usize - ptr as usize) / 2);
        }
        if start[5] == needle {
            return Some((start[5..].as_ptr() as usize - ptr as usize) / 2);
        }
        if start[6] == needle {
            return Some((start[6..].as_ptr() as usize - ptr as usize) / 2);
        }
        if start[7] == needle {
            return Some((start[7..].as_ptr() as usize - ptr as usize) / 2);
        }

        start = &start[8..];
        len -= 8;
    }

    for (i, c) in start.iter().enumerate() {
        if *c == needle {
            return Some((start.as_ptr() as usize - ptr as usize) / 2 + i);
        }
    }
    None
}
