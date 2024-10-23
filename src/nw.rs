//! Provider of [`Nw`].

use crate::util::cmp_ptr;
use crate::Nr;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::rc::Weak;

/// Weak reference to node.
#[derive(Debug, Default)]
pub struct Nw<T: ?Sized>(Weak<T>);

impl<T> Nw<T> {
    /// Create new empty instance.
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
    /// Get base pointer.
    #[inline(always)]
    pub fn bp(&self) -> &Weak<T> {
        &self.0
    }

    /// Create strong pointer to this node.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    #[must_use]
    pub fn upgrade(&self) -> Option<Nr<T>> {
        self.0.upgrade().map(Nr::from_base)
    }

    /// Get the number of strong pointer to this node.
    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.0.strong_count()
    }

    /// Get the number of weak pointer to this node.
    #[must_use]
    pub fn weak_count(&self) -> usize {
        self.0.weak_count()
    }

    /// Create instance from base object.
    #[inline(always)]
    pub(crate) fn from_base(base: Weak<T>) -> Self {
        Self(base)
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
        self.0.as_ptr().hash(state);
    }
}

impl<T: ?Sized> Ord for Nw<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_ptr(self.0.as_ptr(), other.0.as_ptr())
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
