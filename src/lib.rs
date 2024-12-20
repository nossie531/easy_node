/*! Smart pointer for graph nodes.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

This crate provides some smart pointers optimized for
managing graph data structures.

* [`Nr`] - like [`Rc`]
* [`Nw`] - like [`Weak`].

These smart pointers behavior is similar to [`Rc`] and [`Weak`].<br/>
However, These smart pointer comparison is based on location.

## Comparison of smart pointers

This crate smart pointer comparison is based on location.<br/>
This allows smart pointers to be used as [`HashSet`] values, etc.

For example, comparison of [`Nr::eq`] is based on identity of node address.<br/>
On the other hand comparison of [`Rc::eq`] is based on inner value.

[`Rc`]: std::rc::Rc
[`Rc::eq`]: std::rc::Rc::eq
[`Weak`]: std::rc::Weak
[`HashSet`]: std::collections::HashSet
*/

#![warn(missing_docs)]

mod nr;
mod nw;
pub mod prelude;
mod util;

pub use nr::*;
pub use nw::*;
