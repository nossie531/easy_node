use crate::common::ChildCell;
use easy_node::NrCell;

#[derive(Debug)]
pub struct ParentCell<T> {
    _value: T,
    child: NrCell<ChildCell<T>>,
}

impl<T> ParentCell<T> {
    pub fn new(value: T, child: NrCell<ChildCell<T>>) -> Self {
        Self {
            _value: value,
            child,
        }
    }

    pub fn child(&self) -> &NrCell<ChildCell<T>> {
        &self.child
    }
}
