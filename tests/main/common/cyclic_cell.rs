use easy_node::NwCell;

#[derive(Debug)]
pub struct CyclicCell<T> {
    val: T,
    me: NwCell<Self>,
}

impl<T> CyclicCell<T> {
    pub fn new(val: T, nw: &NwCell<Self>) -> Self {
        Self {
            val,
            me: nw.clone(),
        }
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn me(&self) -> &NwCell<Self> {
        &self.me
    }
}
