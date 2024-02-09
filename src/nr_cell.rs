//! Provider of [`NrCell`].

use crate::{nw_cell::NwCell, Nr, Nw};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

/// Base object type.
type Base<T> = Nr<RefCell<T>>;

/// Strong node reference with dynamic borrow checker.
#[derive(Debug)]
pub struct NrCell<T: ?Sized>(Base<T>);

impl<T> NrCell<T> {
    /// Create new instance.
    pub fn new(value: T) -> Self {
        Self(Base::new(RefCell::new(value)))
    }

    /// Create self-referencing instance.
    pub fn new_cyclic<F>(data_fn: F) -> Self
    where
        F: FnOnce(&NwCell<T>) -> T,
    {
        let data_fn = |w: &_| RefCell::new(data_fn(&NwCell(Nw::clone(w))));
        Self(Base::new_cyclic(data_fn))
    }
}

impl<T: ?Sized> NrCell<T> {
    /// Create weak reference of this node.
    pub fn downgrade(this: &Self) -> NwCell<T> {
        NwCell(Base::downgrade(&this.0))
    }

    /// Get the number of strong reference of this node.
    pub fn strong_count(this: &Self) -> usize {
        Base::strong_count(&this.0)
    }

    /// Get the number of weak reference of this node.
    pub fn weak_count(this: &Self) -> usize {
        Base::weak_count(&this.0)
    }
}

impl<T: ?Sized> Clone for NrCell<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: ?Sized> Deref for NrCell<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T: Default> Default for NrCell<T> {
    fn default() -> Self {
        Self(Base::default())
    }
}

impl<T: ?Sized> Eq for NrCell<T> {}

impl<T: ?Sized> Hash for NrCell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T: ?Sized> Ord for NrCell<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: ?Sized> PartialEq for NrCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T: ?Sized> PartialOrd for NrCell<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
