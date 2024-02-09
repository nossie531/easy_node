/*! Smart pointer for graph nodes.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This crate provides some smart pointers optimized for
managing graph data structures.

* [`Nr`] - like [`Rc`]
* [`Nw`] - like [`Weak`].
* [`NrCell`] - newtype of [`Nr<RefCell<T>>`](Nr).
* [`NwCell`] - newtype of [`Nw<RefCell<T>>`](Nw).

These smart pointers behavior is similar to [`Rc`] and [`Weak`].<br/>
However, there are several important differences between them.

## Point1 - Comparison of smart pointers

This crate smart pointer comparison is based on location.<br/>
This allows smart pointers to be used as [`HashSet`] values, etc.

For example, comparison of [`Nr::eq`] is based on identity of node address.<br/>
On the other hand comparison of [`Rc::eq`] is based on inner value.

## Point2 - Upgrade from weak reference

This crate weak reference upgrade returns strong reference location.<br/>
This allows smart pointers act as dynamic object directly.

For example, [`Nw::upgrade`] returns [`Nr`] location.<br/>
On the other hand [`Weak::upgrade`] returns [`Rc`] itself.

[`Rc`]: std::rc::Rc
[`Rc::eq`]: std::rc::Rc::eq
[`Weak`]: std::rc::Weak
[`Weak::upgrade`]: std::rc::Weak::upgrade
[`HashSet`]: std::collections::HashSet
*/

mod node;
mod nr;
mod nr_cell;
mod nw;
mod nw_cell;

pub use nr::*;
pub use nr_cell::*;
pub use nw::*;
pub use nw_cell::*;
