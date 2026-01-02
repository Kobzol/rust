// This test tests that derive-macro execution is cached.

//@ aux-build:derive_nothing.rs
//@ revisions:rpass1 rpass2
//@ compile-flags: -Zquery-dep-graph -Zcache-derive-macros

#![feature(rustc_attrs)]

#[macro_use]
extern crate derive_nothing;

#[cfg(any(rpass1, rpass2))]
#[rustc_clean(cfg = "rpass2", loaded_from_disk = "derive_macro_expansion")]
#[derive(Nothing)]
pub struct Foo;

fn main() {}
