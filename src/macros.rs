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
        $crate::syscall_nofail!($nr$(, $args)*)
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

/// A version of [`syscall!`] that makes it easier to pass 64-bit arguments.
///
/// **WARNING**: Make sure to read "[Important notes](#important-notes)" below before attempting to
/// use this macro!
///
/// # Background
///
/// On 32-bit architectures, system calls that accept 64-bit arguments (i.e. file offsets) must
/// split the argument between two registers. Additionally, some on architectures (for example,
/// ARM, PowerPC, and MIPS), the value must be aligned to an even pair of registers. More
/// information, including a list of affected syscalls, can be found in `syscall(2)`.
///
/// This macro doesn't completely eliminate the architecture-specific differences, but it makes
/// them easier to handle.
///
/// # Usage
///
/// This macro can be invoked just like [`syscall!`], except that any 64-bit arguments should have
/// `@u64` added before them. For example:
///
/// ```no_run
/// # use scall::syscall_args64;
/// unsafe fn readahead(fd: i32, offset: u64, count: usize) -> Result<(), i32> {
///     syscall_args64!(READAHEAD, fd, @u64 offset, count)?;
///     Ok(())
/// }
/// ```
///
/// On 32-bit systems, the `offset` argument will be split and/or aligned appropriately; on 64-bit
/// systems it will be passed directly.
///
/// # Important notes
///
/// - Due to limitations of Rust's macro system, at least one `@u64` argument must be passed. For
///   example, `syscall_args64!(READ, 0, 0, 0)` is invalid; you should use [`syscall!`] instead if
///   you don't need to pass 64-bit arguments.
///
///   Additionally, all of the `@u64` arguments must be listed consecutively; for example,
///   `syscall_args64!(READ, @u64 0, 0, @u64 0)` is invalid. In practice, this shouldn't be an
///   issue because all of the syscalls that require 64-bit arguments have them listed
///   consecutively.
///
/// - The prefix `@u64` should be used even if the kernel will interpret the argument as a signed
///   number (i.e. `i64`).
///
/// - This macro makes it easier to split 64-bit arguments between registers. However, some of
///   the syscalls that accept 64-bit arguments have other architecture-specific differences.
///   `scall` does not account for these differences; that is left to the user.
///
///   `sync_file_range(2)` is a good example of this. Architectures that require even-register
///   alignment have `sync_file_range2()`, which swaps the argument order to avoid requiring 7
///   arguments, instead of `sync_file_range()`.
///
/// - On 32-bit systems, `@u64` arguments will (out of necessity) be evaluated twice. As a result,
///   if evaluating the expression changes its value, behavior is undefined (a problem that may be
///   familiar to C/C++ users). For example, this code will behave differently on 32-bit and 64-bit
///   systems:
///
/// ```no_run
/// # use scall::syscall_args64;
/// unsafe {
///     let mut x = 0;
///     // DO **NOT** DO THIS!!!
///     syscall_args64!(READAHEAD, 0, @u64 { x += 1; x }, 0).unwrap();
/// }
/// ```
#[cfg(target_os = "linux")]
#[doc(cfg(target_os = "linux"))]
#[macro_export]
macro_rules! syscall_args64 {
    ($nr:ident$(, $arg1:expr)* $(, @u64 $arg2:expr)+ $(, $arg3:expr)* $(,)?) => {
        $crate::_scall_internal_syscall_args64!($nr, $($arg1,)* $(@u64 $arg2,)+ $($arg3,)*)
    };
}

// On 64-bit systems, we don't have to split 64-bit values
#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
#[doc(hidden)]
#[macro_export]
macro_rules! _scall_internal_syscall_args64 {
    ($nr:ident, $($sarg1:expr,)* $(@u64 $larg2:expr,)+ $($sarg3:expr,)*) => {
        $crate::syscall!(
            $nr, $($sarg1,)* $($larg2 as usize,)+ $($sarg3,)*
        )
    };
}

// Some 32-bit architectures require that 64-bit values be split, but not necessarily aligned
// This is the *little-endian* version
#[cfg(all(
    target_os = "linux",
    target_pointer_width = "32",
    not(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc")),
    target_endian = "little",
))]
#[doc(hidden)]
#[macro_export]
macro_rules! _scall_internal_syscall_args64 {
    ($nr:ident, $($sarg1:expr,)* $(@u64 $larg2:expr,)+ $($sarg3:expr,)*) => {
        $crate::syscall!(
            $nr, $($sarg1,)* $($larg2 as u32, $larg2 as u64 >> 32,)+ $($sarg3,)*
        )
    };
}

// Some 32-bit architectures require that 64-bit values be split, but not necessarily aligned
// This is the *big-endian* version
#[cfg(all(
    target_os = "linux",
    target_pointer_width = "32",
    not(any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc")),
    target_endian = "big",
))]
#[doc(hidden)]
#[macro_export]
macro_rules! _scall_internal_syscall_args64 {
    ($nr:ident, $($sarg1:expr,)* $(@u64 $larg2:expr,)+ $($sarg3:expr,)*) => {
        $crate::syscall!(
            $nr, $($sarg1,)* $($larg2 as u64 >> 32, $larg2 as u32,)+ $($sarg3,)*
        )
    };
}

// ARM/MIPS/PowerPC require that values be aligned to an even register pair
// This is the *little-endian* version
#[cfg(all(
    target_os = "linux",
    target_pointer_width = "32",
    any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc"),
    target_endian = "little",
))]
#[doc(hidden)]
#[macro_export]
macro_rules! _scall_internal_syscall_args64 {
    ($nr:ident, $($sarg1:expr, $sarg2:expr,)* $(@u64 $larg3:expr,)+ $($sarg4:expr,)*) => {
        $crate::syscall!(
            $nr, $($sarg1, $sarg2,)* $($larg3 as u32, $larg3 as u64 >> 32,)+ $($sarg4,)*
        )
    };

    ($nr:ident, $($sarg1:expr,)* $(@u64 $larg2:expr,)+ $($sarg3:expr,)*) => {
        $crate::syscall!(
            $nr, $($sarg1,)* 0, $($larg2 as u32, $larg2 as u64 >> 32,)+ $($sarg3,)*
        )
    };
}

// ARM/MIPS/PowerPC require that values be aligned to an even register pair
// This is the *big-endian* version
#[cfg(all(
    target_os = "linux",
    target_pointer_width = "32",
    any(target_arch = "arm", target_arch = "mips", target_arch = "powerpc"),
    target_endian = "big",
))]
#[doc(hidden)]
#[macro_export]
macro_rules! _scall_internal_syscall_args64 {
    ($nr:ident, $($sarg1:expr, $sarg2:expr,)* $(@u64 $larg3:expr,)+ $($sarg4:expr,)*) => {
        $crate::syscall!(
            $nr, $($sarg1, $sarg2,)* $($larg3 as u64 >> 32, $larg3 as u32,)+ $($sarg4,)*
        )
    };

    ($nr:ident, $($sarg1:expr,)* $(@u64 $larg2:expr,)+ $($sarg3:expr,)*) => {
        $crate::syscall!(
            $nr, $($sarg1,)* 0, $($larg2 as u64 >> 32, $larg2 as u32,)+ $($sarg3,)*
        )
    };
}
