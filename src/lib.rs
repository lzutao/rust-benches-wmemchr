macro_rules! if_return {
    ($start:ident, $ptr:ident, $needle:ident, $($n:literal,)+) => {
        $(
            if $start[$n] == $needle {
                return Some((&$start[$n] as *const u16 as usize - $ptr as usize) / 2);
            }
        )+
    }
}

// Ported from <https://github.com/bminor/glibc/blob/master/wcsmbs/wmemchr.c>.
pub fn unrollled_wmemchr(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    // For performance reasons unfold the loop eight times.
    let mut chunks = haystack.chunks_exact(8);

    for chunk in &mut chunks {
        if_return!(chunk, ptr, needle, 0, 1, 2, 3, 4, 5, 6, 7,);
    }

    for c in chunks.remainder() {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    None
}

pub fn unrollled_wmemchr_iterative(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    let mut start = &haystack[..];
    /* For performance reasons unfold the loop four times.  */
    while start.len() >= 8 {
        if_return!(start, ptr, needle, 0, 1, 2, 3, 4, 5, 6, 7,);

        start = &start[8..];
    }

    for c in start {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / 2);
        }
    }
    None
}

#[test]
fn test_implementation() {
    let r = unrollled_wmemchr(0, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(r, None);

    let r = unrollled_wmemchr(0, &[0, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(r, Some(0));

    let r = unrollled_wmemchr(0, &[1, 2, 0, 4, 5, 6, 7, 8, 9]);
    assert_eq!(r, Some(2));

    let r = unrollled_wmemchr(0, &[1, 2, 3, 4, 5, 6, 7, 0, 9]);
    assert_eq!(r, Some(7));

    let r = unrollled_wmemchr(0, &[1, 2, 3, 4, 5, 6, 7, 8, 0]);
    assert_eq!(r, Some(8));
}
