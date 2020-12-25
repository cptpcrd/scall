// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for aarch64 Linux.

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> usize {
    let ret: usize;
    llvm_asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> usize {
    let ret: usize;
    llvm_asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    llvm_asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    llvm_asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2) "{x2}"(a3)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let ret: usize;
    llvm_asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2) "{x2}"(a3) "{x3}"(a4)
         : "memory" "cc"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
    let ret: usize;
    llvm_asm!("svc 0" : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2) "{x2}"(a3) "{x3}"(a4) "{x4}"(a5)
         : "memory" "cc"
         : "volatile");
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
    llvm_asm!("svc 0"
         : "={x0}"(ret)
         : "{x8}"(n) "{x0}"(a1) "{x1}"(a2) "{x2}"(a3) "{x3}"(a4) "{x4}"(a5)
           "{x5}"(a6)
         : "memory" "cc"
         : "volatile");
    ret
}

pub use syscall0 as syscall0_nofail;
pub use syscall1 as syscall1_nofail;
pub use syscall2 as syscall2_nofail;
pub use syscall3 as syscall3_nofail;
pub use syscall4 as syscall4_nofail;
pub use syscall5 as syscall5_nofail;
pub use syscall6 as syscall6_nofail;
