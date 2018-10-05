// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
#![allow(dead_code)]

// Test that a borrow that occurs after a yield in the same
// argument list is not treated as live across the yield by
// type-checking.

#![feature(generators)]

fn foo(_a: (), _b: &bool) {}

fn bar() {
    || {
        let b = true;
        foo(yield, &b);
    };
}

fn main() { }