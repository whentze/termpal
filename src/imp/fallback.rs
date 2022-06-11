use crate::imp::{lab::Lab, tab};

#[inline]
pub(crate) fn nearest_ansi256(l: Lab) -> u8 {
    let r = nearest_impl(l, &tab::LAB_PALETTE_ANSI256[..]);
    debug_assert!(r < 256, "{}", r);
    r as u8
}

#[inline]
#[cfg(feature = "88color")]
pub(crate) fn nearest_ansi88(l: Lab) -> u8 {
    let r = nearest_impl(l, &tab::LAB_PALETTE_ANSI88[..]);
    debug_assert!(r < 88, "{}", r);
    r as u8
}

#[inline]
pub(crate) fn nearest_impl(v: Lab, table: &[Lab]) -> usize {
    debug_assert!(!table.is_empty());
    if table.is_empty() {
        return 0;
    }
    let mut bi = 0;
    let mut bm = f32::MAX;
    for (i, c) in table.iter().enumerate() {
        let m = euc_dist_sq(c, &v);
        if m < bm {
            bm = m;
            bi = i;
        }
    }
    bi + 16
}

#[inline]
pub(crate) fn euc_dist_sq(a: &Lab, b: &Lab) -> f32 {
    let dl = a.l - b.l;
    let da = a.a - b.a;
    let db = a.b - b.b;
    (dl * dl) + (da * da) + (db * db)
}