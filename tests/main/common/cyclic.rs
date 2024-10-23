use easy_node::Nw;

#[derive(Debug)]
pub struct Cyclic<T> {
    val: T,
    me: Nw<Self>,
}

impl<T> Cyclic<T> {
    pub fn new(val: T, nw: &Nw<Self>) -> Self {
        Self {
            val,
            me: nw.clone(),
        }
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn me(&self) -> &Nw<Self> {
        &self.me
    }
}
