//! Crate's utility.

use std::cmp::Ordering;

/// Compare two pointers.
pub fn cmp_ptr<X: ?Sized, Y: ?Sized>(x: *const X, y: *const Y) -> Ordering {
    let x = x.cast::<()>();
    let y = y.cast::<()>();
    x.cmp(&y)
}
