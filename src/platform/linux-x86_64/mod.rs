// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library was built for x86-64 Linux.

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(mut n: usize) -> usize {
    llvm_asm!("syscall"
         : "+{rax}"(n)
         :
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall1(mut n: usize, a1: usize) -> usize {
    llvm_asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall2(mut n: usize, a1: usize, a2: usize) -> usize {
    llvm_asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall3(mut n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    llvm_asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall4(mut n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    llvm_asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall5(
    mut n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> usize {
    llvm_asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4) "{r8}"(a5)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

#[inline(always)]
pub unsafe fn syscall6(
    mut n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    llvm_asm!("syscall"
         : "+{rax}"(n)
         : "{rdi}"(a1) "{rsi}"(a2) "{rdx}"(a3) "{r10}"(a4) "{r8}"(a5)"{r9}"(a6)
         : "rcx", "r11", "memory"
         : "volatile");
    n
}

pub use syscall0 as syscall0_nofail;
pub use syscall1 as syscall1_nofail;
pub use syscall2 as syscall2_nofail;
pub use syscall3 as syscall3_nofail;
pub use syscall4 as syscall4_nofail;
pub use syscall5 as syscall5_nofail;
pub use syscall6 as syscall6_nofail;
