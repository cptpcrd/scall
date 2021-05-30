// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for arm Linux.

pub mod eno;
pub mod nr;

#[cfg(target_os = "android")]
mod thumb;
#[cfg(target_os = "android")]
pub use thumb::*;

#[cfg(not(target_os = "android"))]
mod nothumb;
#[cfg(not(target_os = "android"))]
pub use nothumb::*;
