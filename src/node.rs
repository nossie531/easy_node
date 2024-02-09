//! Provider of [`Node`].

use crate::{Nr, Nw};
use std::cell::RefCell;

/// Node body.
#[derive(Debug, Default)]
pub(crate) struct Node<T: ?Sized> {
    /// Node value.
    value: Box<T>,

    /// Self reference.
    self_ref: RefCell<Option<Nr<T>>>,
}

impl<T> Node<T> {
    /// Create new instance.
    pub fn new(value: T) -> Self {
        Self {
            value: Box::new(value),
            self_ref: RefCell::new(None),
        }
    }
}

impl<T: ?Sized> Node<T> {
    pub fn to_self_ref(nw: &Nw<T>) -> Option<&Nr<T>> {
        let rc = nw.base().upgrade()?;
        let me = rc.self_ref.borrow();
        let me = me.as_ref().unwrap();
        let me = unsafe { &*(me as *const _) };
        Some(me)
    }

    pub fn value(&self) -> &T {
        self.value.as_ref()
    }

    pub fn has_self_ref(&self) -> bool {
        self.self_ref.borrow().is_some()
    }

    pub fn set_self_ref(&self, value: Nr<T>) {
        *self.self_ref.borrow_mut() = Some(value);
    }

    pub fn drop_self_ref(&self) {
        *self.self_ref.borrow_mut() = None;
    }
}
