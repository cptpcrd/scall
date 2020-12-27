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
/// **Note**: You should use [`syscall!`] or [`syscall_nofail!`] in most cases!
///
/// On Linux (on **most** architectures), this returns a `usize` representing the return value of
/// the syscall (if an error occurred, the error code is encoded into the result). On macOS and
/// FreeBSD (and on some architectures on Linux), it returns a `(usize, bool)` tuple indicating 1)
/// the return value and 2) whether an error occurred. ([`RawResult`] is an alias for this type,
/// and it can be "decoded" into a `Result<usize, i32>` with [`decode_raw_result()`].)
///
/// Note: [`syscall!`] or [`syscall_nofail!`] should be preferred for most purposes. However, this
/// macro may be useful if you need to make a series of syscalls quickly, *then* check the return
/// values. In this case, you can call `syscall_raw!`, store the [`RawResult`]s from each, and
/// *then* decode and check them with [`decode_raw_result()`].
///
/// [`syscall!`]: ./macro.syscall.html
/// [`syscall_nofail!`]: ./macro.syscall_nofail.html
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

/// Make a syscall that should never fail (or for which the result should be completely ignored).
///
/// Assuming that the syscall won't fail saves the overhead involved with checking for errors, so
/// this macro may see minor performance boosts over [`syscall!`].
///
/// This macro will return the result of the syscall as a `usize`. It is assumed that either:
///
/// 1. The syscall will never fail (like `sync()`, `sched_yield()`, or `getpid()`). In this case
///    the result can be used safely.
/// 2. The result will be ignored (like `close()` often is).
///
/// **Note**: If the syscall fails, the value returned by this function is **completely
/// unspecified** and should be immediately discarded. On some platforms, it may even be a value
/// that otherwise looks like an ordinary return value for this function. Do **NOT** use this macro
/// to call syscalls like `open()` which can easily fail for a variety of reasons and whose
/// results must be checked carefully.
///
/// Example usage:
/// ```
/// # use scall::syscall_nofail;
/// unsafe {
///     // getpid() will never fail
///     let pid = syscall_nofail!(GETPID);
///
///     // Completely ignore the result of close()
///     // (In reality, a valid file descriptor would be used here)
///     syscall_nofail!(CLOSE, -1i32);
/// }
/// ```
///
/// [`syscall!`]: ./macro.syscall.html
#[macro_export]
macro_rules! syscall_nofail {
    ($nr:ident) => {
        $crate::syscall0_nofail($crate::nr::$nr)
    };

    ($nr:ident, $a1:expr) => {
        $crate::syscall1_nofail($crate::nr::$nr, $a1 as usize)
    };

    ($nr:ident, $a1:expr, $a2:expr) => {
        $crate::syscall2_nofail($crate::nr::$nr, $a1 as usize, $a2 as usize)
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::syscall3_nofail($crate::nr::$nr, $a1 as usize, $a2 as usize, $a3 as usize)
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::syscall4_nofail(
            $crate::nr::$nr,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
        )
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        $crate::syscall5_nofail(
            $crate::nr::$nr,
            $a1 as usize,
            $a2 as usize,
            $a3 as usize,
            $a4 as usize,
            $a5 as usize,
        )
    };

    ($nr:ident, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        $crate::syscall6_nofail(
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
        $crate::syscall7_nofail(
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
