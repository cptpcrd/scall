// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Make a syscall, and return the direct result (platform-specific).
///
/// On Linux, this returns a `usize` representing the return value of the syscall (if an error
/// occurred, the error code is encoded into the result). On macOS and FreeBSD, it returns a
/// `(usize, bool)` tuple indicating 1) the return value and 2) whether an error occurred.
/// ([`RawResult`] is an alias for this type, and it can be "decoded" with
/// [`decode_raw_result()`].)
///
/// Note: [`syscall!`] should be preferred for most purposes. However, this macro may be useful in
/// 2 cases:
///
/// 1. The syscall will never fail (like `sync()`) or you don't care about the result (like
///    possibly `close()`).
///
///    Since [`syscall!`] returns an `Result<usize, i32>` and `Result` is annotated with
///    `#[must_use]`, using `syscall_raw!` will simplify things slightly -- i.e.
///    `syscall_raw!(SYNC)` instead of `drop(syscall!(SYNC))` to avoid the compiler complaining.
///
/// 2. You need to make a series of syscalls quickly, *then* check the return values.
///
///    In this case, you can call `syscall_raw!`, store the [`RawResult`]s from each, and *then*
///    decode and check them with [`decode_raw_result()`].
///
/// [`syscall!`]: ./macro.syscall.html
/// [`RawResult`]: ./type.RawResult.html
/// [`decode_raw_result()`]: ./fn.decode_raw_result.html
#[macro_export]
macro_rules! syscall_raw {
    ($nr:ident) => {
        $crate::syscall0($crate::nr::$nr)
    };

    ($nr:ident, $a1:expr) => {
        $crate::syscall1($crate::nr::$nr, $a1 as usize)
    };

    ($nr:ident, $a1:expr, $a2:expr) => {
        $crate::syscall2($crate::nr::$nr, $a1 as usize, $a2 as usize)
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::syscall3($crate::nr::$nr, $a1 as usize, $a2 as usize, $a3 as usize)
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::syscall4(
            $crate::nr::$nr,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
        )
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        $crate::syscall5(
            $crate::nr::$nr,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
        )
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        $crate::syscall6(
            $crate::nr::$nr,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
            $a6 as usize,
        )
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr, $a7:expr) => {
        $crate::syscall7(
            $crate::nr::$nr,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
            $a6 as usize,
            $a7 as usize,
        )
    };

    ($nr:ident, $($args:expr,)*) => {
        $crate::syscall_raw!($nr$(, $args)*)
    };
}

/// Make a syscall, and return the result as a `Result<usize, i32>`.
///
/// This returns `Ok(retval)` on success, and `Err(errno)` on error. It's essentially equivalent to
/// `decode_raw_result(syscall_raw!(...))`.
///
/// Tip: If you're not in a `#![no_std]` crate, you can do something like `syscall!(SETRESUID, 0,
/// 0, 0).map_err(std::io::Error::from_raw_os_error)` to get an `io::Result<usize>`, which is
/// easier to work with.
#[macro_export]
macro_rules! syscall {
    ($nr:ident$(, $args:expr)*) => {
        $crate::decode_raw_result($crate::syscall_raw!($nr$(, $args)*))
    };

    ($nr:ident, $($args:expr,)*) => {
        $crate::syscall!($nr$(, $args)*)
    };
}
