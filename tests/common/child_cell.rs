use crate::ParentCell;
use easy_node::NwCell;

#[derive(Debug)]
pub struct ChildCell<T> {
    _value: T,
    parent: NwCell<ParentCell<T>>,
}

impl<T> ChildCell<T> {
    pub fn new(value: T, parent: NwCell<ParentCell<T>>) -> Self {
        Self {
            _value: value,
            parent,
        }
    }

    pub fn parent(&self) -> &NwCell<ParentCell<T>> {
        &self.parent
    }
}
