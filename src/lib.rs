// #[cfg(not(windows))]
// compile_error!("This crate only works on Windows.");

// Ported from <https://github.com/bminor/glibc/blob/master/wcsmbs/wmemchr.c>.
pub fn unrolled_find_u16s(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();

    // For performance reasons unfold the loop eight times.
    let mut chunks_8 = haystack.chunks_exact(8);
    for chunk in &mut chunks_8 {
        let mut iter = chunk.iter();
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / 2);
            }
        }
    }

    let mut remains = chunks_8.remainder().iter();
    if let Some(c) = remains.next() {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    if let Some(c) = remains.next() {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    if let Some(c) = remains.next() {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    if let Some(c) = remains.next() {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    if let Some(c) = remains.next() {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    if let Some(c) = remains.next() {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    if let Some(c) = remains.next() {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    None
}

#[test]
fn test_implementation() {
    let r = unrolled_find_u16s(0, [1, 2, 3, 4]);
    assert_eq!(r, None);

    let r = unrolled_find_u16s(0, [0, 2, 3, 4]);
    assert_eq!(r, Some(1));

    let r = unrolled_find_u16s(0, [1, 2, 0, 4]);
    assert_eq!(r, Some(2));

    let r = unrolled_find_u16s(0, [1, 2, 3, 0]);
    assert_eq!(r, Some(3));
}
