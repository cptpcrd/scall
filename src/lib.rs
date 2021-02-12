// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Friendlier raw system calls for Rust.
//!
//! Example usage:
//!
//! ```
//! # use scall::{syscall, syscall_nofail};
//! unsafe {
//!     let pid = syscall_nofail!(GETPID);
//!     syscall!(KILL, pid, 0).unwrap();
//! }
//! ```
//!
//! *Note: This crate has several functions/macros, and looking through the documentation may seem
//! a little confusing. However, for most purposes, you'll just want to use the [`syscall!`] macro
//! (or sometimes [`syscall_nofail!`]). The other functions/macros are mostly present to support
//! more advanced use cases.*

#![deny(warnings)]
#![allow(clippy::missing_safety_doc)]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(doc_cfg)]
#![no_std]

pub use platform::*;

mod macros;

#[cfg(target_os = "linux")]
mod args64;

#[cfg_attr(
    all(target_os = "linux", target_arch = "aarch64"),
    path = "platform/linux-aarch64/mod.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "arm"),
    path = "platform/linux-armeabi/mod.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "mips"),
    path = "platform/linux-mips/mod.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "mips64"),
    path = "platform/linux-mips64/mod.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "powerpc"),
    path = "platform/linux-powerpc/mod.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "powerpc64"),
    path = "platform/linux-powerpc64/mod.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "sparc64"),
    path = "platform/linux-sparc64/mod.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "riscv64"),
    path = "platform/linux-riscv64/mod.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "x86"),
    path = "platform/linux-x86/mod.rs"
)]
#[cfg_attr(
    all(target_os = "linux", target_arch = "x86_64"),
    path = "platform/linux-x86_64/mod.rs"
)]
#[cfg_attr(
    all(target_os = "freebsd", target_arch = "x86_64"),
    path = "platform/freebsd-x86_64/mod.rs"
)]
#[cfg_attr(
    all(target_os = "macos", target_arch = "x86_64"),
    path = "platform/macos-x86_64/mod.rs"
)]
pub mod platform;

/// The type returned by [`syscall_raw!`] on this platform.
///
/// Can be "decoded" into a `Result<usize, i32>` with [`decode_raw_result()`].
///
/// [`syscall_raw!`]: ./macro.syscall_raw.html
/// [`decode_raw_result()`]: ./fn.decode_raw_result.html
#[cfg(scall_error = "packed")]
pub type RawResult = usize;
/// The type returned by [`syscall_raw!`].
///
/// Can be "decoded" into a `Result<usize, i32>` with [`decode_raw_result()`].
///
/// [`syscall_raw!`]: ./macro.syscall_raw.html
/// [`decode_raw_result()`]: ./fn.decode_raw_result.html
#[cfg(scall_error = "flag")]
pub type RawResult = (usize, bool);

/// "Decode" a result from [`syscall_raw!`].
///
/// This returns `Ok(retval)` for "success" results, and `Err(errno)` for "error" results.
///
/// It can also be used to decode the results of the `syscallX()` functions, like [`syscall0()`].
///
/// [`syscall_raw!`]: ./macro.syscall_raw.html
/// [`syscall0()`]: ./platform/fn.syscall0.html
#[inline(always)]
pub fn decode_raw_result(res: RawResult) -> Result<usize, i32> {
    #[cfg(scall_error = "packed")]
    return if res > -4096isize as usize {
        Err((!res + 1) as i32)
    } else {
        Ok(res)
    };

    #[cfg(scall_error = "flag")]
    return if res.1 { Err(res.0 as i32) } else { Ok(res.0) };
}
