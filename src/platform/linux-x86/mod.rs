// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for x86 Linux.

pub mod eno;
pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> usize {
    let ret: usize;
    asm!(
        "int $$0x80",
        inout("eax") n => ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> usize {
    let ret: usize;
    asm!(
        "int $$0x80",
        inout("eax") n => ret,
        in("ebx") a1,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    asm!(
        "int $$0x80",
        inout("eax") n => ret,
        in("ebx") a1,
        in("ecx") a2,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    asm!(
        "int $$0x80",
        inout("eax") n => ret,
        in("ebx") a1,
        in("ecx") a2,
        in("edx") a3,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    // esi is LLVM-reserved, so we have to save and restore it
    let ret: usize;
    asm!(
        "push esi",
        "mov esi, edi",
        "int $$0x80",
        "pop esi",
        inout("eax") n => ret,
        in("ebx") a1,
        in("ecx") a2,
        in("edx") a3,
        in("edi") a4,
    );
    ret
}

// XXX: Because ebp and esi need to be restored, syscall5() and syscall6() have to place the last
// few arguments in an array and then mov them to the proper locations (restoring the values from
// the stack afterward)

#[inline(always)]
pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
    let ret: usize;
    let final_args = [a4, a5];

    asm!(
        "push esi",
        "mov esi, [edi + 0]",
        "mov edi, [edi + 4]",
        "int $$0x80",
        "pop esi",
        inout("eax") n => ret,
        in("ebx") a1,
        in("ecx") a2,
        in("edx") a3,
        inout("edi") final_args.as_ptr() => _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall6(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    let ret: usize;
    let final_args = [a4, a5, a6];

    asm!(
        "push ebp",
        "push esi",
        "mov ebp, [edi + 8]",
        "mov esi, [edi + 0]",
        "mov edi, [edi + 4]",
        "int $$0x80",
        "pop esi",
        "pop ebp",
        inout("eax") n => ret,
        in("ebx") a1,
        in("ecx") a2,
        in("edx") a3,
        inout("edi") final_args.as_ptr() => _,
    );
    ret
}

pub use syscall0 as syscall0_nofail;
pub use syscall1 as syscall1_nofail;
pub use syscall2 as syscall2_nofail;
pub use syscall3 as syscall3_nofail;
pub use syscall4 as syscall4_nofail;
pub use syscall5 as syscall5_nofail;
pub use syscall6 as syscall6_nofail;
