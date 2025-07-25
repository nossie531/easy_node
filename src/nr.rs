//! Provider of [`Nr`].

use crate::prelude::*;
use crate::util::cmp_ptr;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::mem;
use std::ops::Deref;
use std::rc::{Rc, Weak};

/// Strong reference to node.
#[repr(transparent)]
#[derive(Debug, Default)]
pub struct Nr<T: ?Sized>(Rc<T>);

impl<T> Nr<T> {
    /// Creates a new instance.
    pub fn new(value: T) -> Self {
        Self(Rc::new(value))
    }

    /// Creates self-referencing instance.
    pub fn new_cyclic<F>(data_fn: F) -> Self
    where
        F: FnOnce(&Nw<T>) -> T,
    {
        let conv_arg = |w: &_| Nw::from_base(Weak::clone(w));
        let base = Rc::new_cyclic(|w| data_fn(&conv_arg(w)));
        Self(base)
    }
}

impl<T: ?Sized> Nr<T> {
    /// Creates reference from base object.
    #[must_use]
    pub fn as_base(base: &Rc<T>) -> &Self {
        unsafe { mem::transmute(base) }
    }

    /// Creates instance from base object.
    #[must_use]
    pub fn from_base(base: Rc<T>) -> Self {
        Self(base)
    }

    /// Returns a raw pointer to the data.
    #[must_use]
    pub fn as_ptr(this: &Self) -> *const T {
        Rc::as_ptr(&this.0)
    }

    /// Returns base object.
    #[must_use]
    pub fn base(this: &Self) -> &Rc<T> {
        &this.0
    }

    /// Creates weak pointer to this node.
    #[must_use]
    pub fn downgrade(this: &Self) -> Nw<T> {
        Nw::from_base(Rc::downgrade(&this.0))
    }

    /// Returns the number of strong pointer to this node.
    pub fn strong_count(this: &Self) -> usize {
        Rc::strong_count(&this.0)
    }

    /// Returns the number of weak pointer to this node.
    pub fn weak_count(this: &Self) -> usize {
        Rc::weak_count(&this.0)
    }
}

impl<T: ?Sized> Clone for Nr<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: ?Sized> Deref for Nr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T: ?Sized + Display> Display for Nr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.0.fmt(f)
    }
}

impl<T: ?Sized> Eq for Nr<T> {}

impl<T: ?Sized> Hash for Nr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Self::as_ptr(self).hash(state);
    }
}

impl<T: ?Sized> Ord for Nr<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_ptr(Self::as_ptr(self), Self::as_ptr(other))
    }
}

impl<T: ?Sized> PartialEq for Nr<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: ?Sized> PartialOrd for Nr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
