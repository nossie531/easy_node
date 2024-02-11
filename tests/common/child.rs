use crate::Parent;
use easy_node::Nw;

#[derive(Debug)]
pub struct Child<T> {
    _value: T,
    parent: Nw<Parent<T>>,
}

impl<T> Child<T> {
    pub fn new(value: T, parent: Nw<Parent<T>>) -> Self {
        Self {
            _value: value,
            parent,
        }
    }

    pub fn parent(&self) -> &Nw<Parent<T>> {
        &self.parent
    }
}
