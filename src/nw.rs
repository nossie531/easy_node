//! Provider of [`Nw`].

use crate::prelude::*;
use crate::util::cmp_ptr;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::mem;
use std::rc::Weak;

/// Weak reference to node.
#[repr(transparent)]
#[derive(Debug, Default)]
pub struct Nw<T: ?Sized>(Weak<T>);

impl<T> Nw<T> {
    /// Creates new empty instance.
    ///
    /// Calling [`upgrade`] on the return value always gives [`None`].
    ///
    /// [`upgrade`]: Self::upgrade
    #[must_use]
    pub fn new() -> Self {
        Self(Weak::new())
    }
}

impl<T: ?Sized> Nw<T> {
    /// Creates reference from base object.
    #[must_use]
    pub fn as_base(base: &Weak<T>) -> &Self {
        unsafe { mem::transmute(base) }
    }

    /// Creates instance from base object.
    #[must_use]
    pub fn from_base(base: Weak<T>) -> Self {
        Self(base)
    }

    /// Returns a raw pointer to the data.
    ///
    /// The pointer is valid only if there are some strong references.
    /// The pointer may be dangling, unaligned or even [`null`] otherwise.
    ///
    /// [`null`]: std::ptr::null
    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        Weak::as_ptr(&self.0)
    }

    /// Returns base object.
    #[must_use]
    pub fn base(&self) -> &Weak<T> {
        &self.0
    }

    /// Creates strong pointer to this node.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    #[must_use]
    pub fn upgrade(&self) -> Option<Nr<T>> {
        self.0.upgrade().map(Nr::from_base)
    }

    /// Returns the number of strong pointer to this node.
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.0.strong_count()
    }

    /// Returns the number of weak pointer to this node.
    #[must_use]
    pub fn weak_count(&self) -> usize {
        self.0.weak_count()
    }
}

impl<T: ?Sized> Clone for Nw<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> Eq for Nw<T> {}

impl<T: ?Sized> Hash for Nw<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ptr().hash(state);
    }
}

impl<T: ?Sized> Ord for Nw<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_ptr(self.as_ptr(), other.as_ptr())
    }
}

impl<T: ?Sized> PartialEq for Nw<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl<T: ?Sized> PartialOrd for Nw<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
