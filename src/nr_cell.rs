//! Provider of [`NrCell`].

use crate::nw_cell::NwCell;
use crate::{Nr, Nw};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::Rc;

/// Base object type.
type Base<T> = Nr<RefCell<T>>;

/// Strong reference to cell node.
#[derive(Debug, Default)]
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
        let conv_ret = RefCell::new;
        let conv_arg = |w: &_| NwCell::from_base(Nw::clone(w));
        let data_fn = |w: &_| conv_ret(data_fn(&conv_arg(w)));
        let base = Base::new_cyclic(data_fn);
        Self(base)
    }
}

impl<T: ?Sized> NrCell<T> {
    /// Get base pointer.
    #[inline]
    pub fn bp(this: &Self) -> &Rc<RefCell<T>> {
        Nr::bp(&this.0)
    }

    /// Create weak pointer to this node.
    #[must_use]
    pub fn downgrade(this: &Self) -> NwCell<T> {
        NwCell::from_base(Nr::downgrade(&this.0))
    }

    /// Get the number of strong pointer to this node.
    pub fn strong_count(this: &Self) -> usize {
        Base::strong_count(&this.0)
    }

    /// Get the number of weak pointer to this node.
    pub fn weak_count(this: &Self) -> usize {
        Base::weak_count(&this.0)
    }

    /// Create instance from base object.
    #[inline(always)]
    pub(crate) fn from_base(base: Base<T>) -> Self {
        Self(base)
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
