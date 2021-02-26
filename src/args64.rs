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
///
/// # Safety
///
/// See [`syscall!`] (and also read "[Important notes](#important-notes)" above).
#[doc(cfg(target_os = "linux"))]
#[macro_export]
macro_rules! syscall_args64 {
    ($nr:ident$(, $arg1:expr)* $(, @u64 $arg2:expr)+ $(, $arg3:expr)* $(,)?) => {
        $crate::_scall_internal_syscall_args64!($nr, $($arg1,)* $(@u64 $arg2,)+ $($arg3,)*)
    };
}

// On 64-bit systems, we don't have to split 64-bit values
#[cfg(target_pointer_width = "64")]
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
