//! Provider of [`Nr`].

use crate::node::Node;
use crate::Nw;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::rc::{Rc, Weak};

/// Strong node reference.
#[derive(Debug)]
pub struct Nr<T: ?Sized> {
    /// Base object.
    base: Rc<Node<T>>,

    /// For Self-referencing.
    for_self_ref: bool,
}

impl<T> Nr<T> {
    /// Create new instance.
    pub fn new(value: T) -> Self {
        let base = Rc::new(Node::new(value));
        Self::from_base(base)
    }

    /// Create self-referencing instance.
    pub fn new_cyclic<F>(data_fn: F) -> Self
    where
        F: FnOnce(&Nw<T>) -> T,
    {
        let conv_arg = |w: &_| Nw::from_base(Weak::clone(w));
        let base = Rc::new_cyclic(|w| Node::new(data_fn(&conv_arg(w))));
        Self::from_base(base)
    }
}

impl<T: ?Sized> Nr<T> {
    /// Create weak reference of this node.
    #[must_use]
    pub fn downgrade(this: &Self) -> Nw<T> {
        this.base.set_self_ref(Nr::clone_as_self_ref(this));
        Nw::from_base(Rc::downgrade(&this.base))
    }

    /// Get the number of strong reference of this node.
    pub fn strong_count(this: &Self) -> usize {
        let self_ref_count = if this.base.has_self_ref() { 1 } else { 0 };
        Rc::strong_count(&this.base) - self_ref_count
    }

    /// Get the number of weak reference of this node.
    pub fn weak_count(this: &Self) -> usize {
        Rc::weak_count(&this.base)
    }

    /// Create instance from base object.
    fn from_base(base: Rc<Node<T>>) -> Self {
        Self {
            base,
            for_self_ref: false,
        }
    }

    /// Clone reference as self-reference.
    fn clone_as_self_ref(this: &Self) -> Self {
        let mut result = this.clone();
        result.for_self_ref = true;
        result
    }
}

impl<T: ?Sized> Clone for Nr<T> {
    fn clone(&self) -> Self {
        Self {
            base: Rc::clone(&self.base),
            for_self_ref: self.for_self_ref,
        }
    }
}

impl<T: ?Sized> Deref for Nr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.base.value()
    }
}

impl<T: ?Sized> Drop for Nr<T> {
    fn drop(&mut self) {
        if self.for_self_ref {
            return;
        }

        if Self::strong_count(self) <= 1 {
            self.base.drop_self_ref();
        }
    }
}

impl<T: Default> Default for Nr<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: ?Sized + Display> Display for Nr<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&**self, f)
    }
}

impl<T: ?Sized> Eq for Nr<T> {}

impl<T: ?Sized> Hash for Nr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.base).hash(state);
    }
}

impl<T: ?Sized> Ord for Nr<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        Rc::as_ptr(&self.base).cmp(&Rc::as_ptr(&other.base))
    }
}

impl<T: ?Sized> PartialEq for Nr<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.base, &other.base)
    }
}

impl<T: ?Sized> PartialOrd for Nr<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
