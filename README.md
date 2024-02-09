easy_node
===

Smart pointer for graph nodes.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

This crate provides some smart pointers optimized for
managing graph data structures. 

* `Nr` - like `Rc`
* `Nw` - like `Weak`.
* `NrCell` - newtype of `Nr<RefCell<T>>`.
* `NwCell` - newtype of `Nw<RefCell<T>>`.

These smart pointers behavior is similar to `Rc` and `Weak`.<br/>
However, there are several important differences between them.<br/>
See the API documentation for details.
