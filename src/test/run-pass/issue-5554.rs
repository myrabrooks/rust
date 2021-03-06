// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[feature(macro_rules)];

use std::num::Zero;

pub struct X<T> {
  a: T
}

// reordering these bounds stops the ICE
impl<T: Zero + Eq + Zero>
  Zero for X<T> {
    fn zero() -> X<T> {
      X { a: Zero::zero() }
    }
    fn is_zero(&self) -> bool {
        self.a.is_zero()
    }
}

macro_rules! constants {
  () => {
    let _0 : X<int> = Zero::zero();
   }
}


pub fn main() {
  constants!();
}
