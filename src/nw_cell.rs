//! Provider of [`NwCell`].

use crate::{NrCell, Nw};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// Base object type.
type Base<T> = Nw<RefCell<T>>;

/// Weak reference to cell node.
#[derive(Debug)]
pub struct NwCell<T: ?Sized>(pub(crate) Base<T>);

impl<T> NwCell<T> {
    /// Create new instance.
    ///
    /// Calling [`upgrade`] and [`upgrade_ref`] on the return value always gives [`None`].
    ///
    /// [`upgrade`]: Self::upgrade
    /// [`upgrade_ref`]: Self::upgrade_ref
    #[must_use]
    pub fn new() -> Self {
        Self(Nw::new())
    }
}

impl<T: ?Sized> NwCell<T> {
    /// Create strong pointer to this node.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    #[must_use]
    pub fn upgrade(&self) -> Option<NrCell<T>> {
        self.0.upgrade().map(NrCell::from_base)
    }

    /// Get the reference of strong pointer to this node.
    ///
    /// Returns [`None`] if the inner value has since been dropped.
    #[must_use]
    pub fn upgrade_ref(&self) -> Option<&NrCell<T>> {
        self.0.upgrade_ref().map(NrCell::as_self)
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
}

impl<T: ?Sized> Clone for NwCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for NwCell<T> {
    fn default() -> Self {
        Self(Base::default())
    }
}

impl<T: ?Sized> Eq for NwCell<T> {}

impl<T: ?Sized> Hash for NwCell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: ?Sized> Ord for NwCell<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ?Sized> PartialEq for NwCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: ?Sized> PartialOrd for NwCell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
