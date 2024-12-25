# getset2

[![Download](https://img.shields.io/crates/d/getset2)](https://crates.io/crates/getset2)
[![License](https://img.shields.io/crates/l/getset2)](https://github.com/andeya/getset2/blob/master/LICENSE)
[![Docs](https://docs.rs/getset2/badge.svg)](https://docs.rs/getset2/)
[![Coverage Status](https://coveralls.io/repos/github/andeya/getset2/badge.svg)](https://coveralls.io/github/andeya/getset2)

Getset2 is a derive macro, which is inspired by [getset](https://crates.io/crates/getset),
is designed for generating the most basic getters and setters on struct fields.

## Install

Run the following Cargo command in your project directory:

```sh
cargo add getset2
```

## Example

```rust
use getset2::Getset2;

#[derive(Default, Getset2)]
#[getset2(get_ref, set_with)]
pub struct Foo<T>
where
    T: Copy + Clone + Default,
{
    /// Doc comments are supported!
    /// Multiline, even.
    #[getset2(get_ref, set, get_mut, skip(set_with))]
    private: T,

    /// Doc comments are supported!
    /// Multiline, even.
    #[getset2(
        get_copy(pub),
        set(pub = "crate"),
        get_mut(pub = "super"),
        set_with(pub = "self")
    )]
    public: T,

    #[getset2(skip)]
    skip: (),
}

impl<T: Copy + Clone + Default> Foo<T> {
    fn with_private(mut self, private: T) -> Self {
        self.private = private;
        self
    }
    fn skip(&self) {
        self.skip
    }
}

// cargo expand --example simple

fn main() {
    let mut foo = Foo::default();
    foo.set_private(1);
    (*foo.private_mut()) += 1;
    assert_eq!(*foo.private(), 2);
    foo = foo.with_private(3);
    assert_eq!(*foo.private(), 3);
    foo.set_public(3);
    assert_eq!(foo.public(), 3);
    assert_eq!(foo.skip(), ());
}
```

Expand the source code above (Run `cargo expand --example simple`):

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use getset2::Getset2;
#[getset2(get_ref, set_with)]
pub struct Foo<T>
where
    T: Copy + Clone + Default,
{
    /// Doc comments are supported!
    /// Multiline, even.
    #[getset2(set, get_mut, skip(get_ref))]
    private: T,
    /// Doc comments are supported!
    /// Multiline, even.
    #[getset2(
        get_copy(pub),
        set(pub = "crate"),
        get_mut(pub = "super"),
        set_with(pub = "self")
    )]
    public: T,
    #[getset2(skip)]
    skip: (),
}
impl<T> Foo<T>
where
    T: Copy + Clone + Default,
{
    /// Doc comments are supported!
    /// Multiline, even.
    #[inline(always)]
    fn set_private(&mut self, val: T) -> &mut Self {
        self.private = val;
        self
    }
    /// Doc comments are supported!
    /// Multiline, even.
    #[inline(always)]
    fn private_mut(&mut self) -> &mut T {
        &mut self.private
    }
    /// Doc comments are supported!
    /// Multiline, even.
    #[inline(always)]
    fn with_private(mut self, val: T) -> Self {
        self.private = val;
        self
    }
    /// Doc comments are supported!
    /// Multiline, even.
    #[inline(always)]
    pub fn public(&self) -> T {
        self.public
    }
    /// Doc comments are supported!
    /// Multiline, even.
    #[inline(always)]
    pub(crate) fn set_public(&mut self, val: T) -> &mut Self {
        self.public = val;
        self
    }
    /// Doc comments are supported!
    /// Multiline, even.
    #[inline(always)]
    pub(crate) fn public_mut(&mut self) -> &mut T {
        &mut self.public
    }
    /// Doc comments are supported!
    /// Multiline, even.
    #[inline(always)]
    pub(self) fn with_public(mut self, val: T) -> Self {
        self.public = val;
        self
    }
}
#[automatically_derived]
impl<T: ::core::default::Default> ::core::default::Default for Foo<T>
where
    T: Copy + Clone + Default,
{
    #[inline]
    fn default() -> Foo<T> {
        Foo {
            private: ::core::default::Default::default(),
            public: ::core::default::Default::default(),
            skip: ::core::default::Default::default(),
        }
    }
}
impl<T: Copy + Clone + Default> Foo<T> {
    fn private(&self) -> &T {
        &self.private
    }
    fn skip(&self) {
        self.skip
    }
}
fn main() {
    let mut foo = Foo::default();
    foo.set_private(1);
    (*foo.private_mut()) += 1;
    assert_eq!(*foo.private(), 2);
    foo = foo.with_private(3);
    assert_eq!(*foo.private(), 3);
    foo.set_public(3);
    assert_eq!(foo.public(), 3);
    assert_eq!(foo.skip(), ());
}
```
