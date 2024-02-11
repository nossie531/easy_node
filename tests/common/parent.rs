use crate::Child;
use easy_node::Nr;

#[derive(Debug)]
pub struct Parent<T> {
    _value: T,
    child: Nr<Child<T>>,
}

impl<T> Parent<T> {
    pub fn new(value: T, child: Nr<Child<T>>) -> Self {
        Self {
            _value: value,
            child,
        }
    }

    pub fn child(&self) -> &Nr<Child<T>> {
        &self.child
    }
}
