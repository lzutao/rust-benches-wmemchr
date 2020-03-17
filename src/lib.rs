const RATIO: usize = core::mem::size_of::<u16>();

// Ported from <https://github.com/bminor/glibc/blob/master/wcsmbs/wmemchr.c>.
pub fn unrollled_wmemchr(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();

    // For performance reasons unfold the loop eight times.
    for chunk in &mut haystack.chunks(8) {
        let mut iter = chunk.iter();
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / RATIO);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / RATIO);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / RATIO);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / RATIO);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / RATIO);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / RATIO);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / RATIO);
            }
        }
        if let Some(c) = iter.next() {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / RATIO);
            }
        }
    }
    None
}

pub fn unrollled_wmemchr_iterative(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    let mut len = haystack.len();
    let mut start = &haystack[..];

    /* For performance reasons unfold the loop four times.  */
    while len >= 8 {
        if start[0] == needle {
            return Some((start.as_ptr() as usize - ptr as usize) / RATIO);
        }
        if start[1] == needle {
            return Some((start[1..].as_ptr() as usize - ptr as usize) / RATIO);
        }
        if start[2] == needle {
            return Some((start[2..].as_ptr() as usize - ptr as usize) / RATIO);
        }
        if start[3] == needle {
            return Some((start[3..].as_ptr() as usize - ptr as usize) / RATIO);
        }
        if start[4] == needle {
            return Some((start[4..].as_ptr() as usize - ptr as usize) / RATIO);
        }
        if start[5] == needle {
            return Some((start[5..].as_ptr() as usize - ptr as usize) / RATIO);
        }
        if start[6] == needle {
            return Some((start[6..].as_ptr() as usize - ptr as usize) / RATIO);
        }
        if start[7] == needle {
            return Some((start[7..].as_ptr() as usize - ptr as usize) / RATIO);
        }

        start = &start[8..];
        len -= 8;
    }

    for (i, c) in start.iter().enumerate() {
        if *c == needle {
            return Some((start.as_ptr() as usize - ptr as usize) / RATIO + i);
        }
    }
    None
}

#[test]
fn test_implementation() {
    let r = unrolled_find_u16s(0, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(r, None);

    let r = unrolled_find_u16s(0, &[0, 2, 3, 4, 5, 6, 7, 8, 9]);
    assert_eq!(r, Some(0));

    let r = unrolled_find_u16s(0, &[1, 2, 0, 4, 5, 6, 7, 8, 9]);
    assert_eq!(r, Some(2));

    let r = unrolled_find_u16s(0, &[1, 2, 3, 4, 5, 6, 7, 0, 9]);
    assert_eq!(r, Some(7));

    let r = unrolled_find_u16s(0, &[1, 2, 3, 4, 5, 6, 7, 8, 0]);
    assert_eq!(r, Some(8));
}
