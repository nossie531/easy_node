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

These smart pointers behavior is similar to `Rc` and `Weak`.<br/>
However, These smart pointer comparison is location based.

## Other options

[`by_address`] is a great crate with similar purpose.

It supports location based comparison too. But unlike this crate,
it can target any type that implements `Deref` trait. And therefore,
`Weak` that does not implement `Deref` need other [support][issue].

On the other hand, this crate specializes on `Rc` and `Weak`. Instead,
we discard other smart pointers like `Box`. This is because this crate
assumes only nodes in graph or network.

[`by_address`]: https://crates.io/crates/by_address
[issue]: https://github.com/mbrubeck/by_address/issues/3

## Unsize handling

As of 2024, to support unsize conversions with smart pointers, unstable
feature `CoerceUnsized` is required. So, `Rc<Type>` to `Rc<dyn Trait>`
conversion is supported, but `Nr<Type>` to `Nr<dyn Trait>` is not.

As workaround, we provide inter-conversions between `Nr` and `Rc`, and
between `Nw` and `Weak`. `Nr` and `Nw` functions `as_base`, `from_base`,
and `base` are them.

## What's New?

v0.3.2
* Edit documentation.

v0.3.1
* Add `prelude` module.
* Edit documentation.

v0.3.0
* Obsolete `upgrade_ref` method (unsafe misusing...).
* Obsolete `NrCell` and `NwCell` (not essential...).
* Add `base`, `as_base`, `from_base` method.
