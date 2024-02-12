//! Provider of [`Nw`].

use crate::node::Node;
use crate::Nr;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::rc::Weak;

/// Weak reference to node.
#[derive(Debug)]
pub struct Nw<T: ?Sized> {
    /// Base object.
    base: Weak<Node<T>>,
}

impl<T> Nw<T> {
    /// Create new empty instance.
    ///
    /// Calling [`upgrade`] and [`upgrade_ref`] on the return value always gives [`None`].
    ///
    /// [`upgrade`]: Self::upgrade
    /// [`upgrade_ref`]: Self::upgrade_ref
    #[must_use]
    pub fn new() -> Self {
        Self { base: Weak::new() }
    }
}

impl<T: ?Sized> Nw<T> {
    /// Create strong pointer to this node.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    #[must_use]
    pub fn upgrade(&self) -> Option<Nr<T>> {
        self.base.upgrade().map(Nr::from_base)
    }

    /// Get the reference of strong pointer to this node.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    #[must_use]
    pub fn upgrade_ref(&self) -> Option<&Nr<T>> {
        Node::to_self_ref(self)
    }

    /// Get the number of strong pointer to this node.
    #[must_use]
    pub fn strong_count(&self) -> usize {
        let self_ref_count = self.base.weak_count().min(1);
        self.base.strong_count() - self_ref_count
    }

    /// Get the number of weak pointer to this node.
    #[must_use]
    pub fn weak_count(&self) -> usize {
        self.base.weak_count()
    }

    /// Create instance from base object.
    #[inline(always)]
    pub(crate) fn from_base(base: Weak<Node<T>>) -> Self {
        Self { base }
    }

    /// Get base object.
    #[inline(always)]
    pub(crate) fn base(&self) -> &Weak<Node<T>> {
        &self.base
    }
}

impl<T: ?Sized> Clone for Nw<T> {
    fn clone(&self) -> Self {
        Self {
            base: Weak::clone(&self.base),
        }
    }
}

impl<T> Default for Nw<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ?Sized> Drop for Nw<T> {
    fn drop(&mut self) {
        if self.base.weak_count() == 1 {
            let rc = self.base.upgrade().unwrap();
            rc.drop_self_ref();
        }
    }
}

impl<T: ?Sized> Eq for Nw<T> {}

impl<T: ?Sized> Hash for Nw<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.as_ptr().hash(state);
    }
}

impl<T: ?Sized> Ord for Nw<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.base.as_ptr().cmp(&other.base.as_ptr())
    }
}

impl<T: ?Sized> PartialEq for Nw<T> {
    fn eq(&self, other: &Self) -> bool {
        self.base.ptr_eq(&other.base)
    }
}

impl<T: ?Sized> PartialOrd for Nw<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
