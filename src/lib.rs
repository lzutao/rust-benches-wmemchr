// #[cfg(not(windows))]
// compile_error!("This crate only works on Windows.");

// Ported from <https://github.com/bminor/glibc/blob/master/wcsmbs/wmemchr.c>.
pub fn unrolled_find_u16s(needle: u16, haystack: &[u16]) -> Option<usize> {
    let ptr = haystack.as_ptr();
    const RATIO: usize = core::mem::size_of::<u16>();

    // For performance reasons unfold the loop eight times.
    let mut chunks_8 = haystack.chunks_exact(8);
    for chunk in &mut chunks_8 {
        for c in chunk {
            if *c == needle {
                return Some((c as *const u16 as usize - ptr as usize) / RATIO);
            }
        }
    }

    for c in chunks_8.remainder() {
        if *c == needle {
            return Some((c as *const u16 as usize - ptr as usize) / RATIO);
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
