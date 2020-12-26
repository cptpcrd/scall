// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for PowerPC Linux.

// Reference: https://www.kernel.org/doc/Documentation/powerpc/syscall64-abi.txt

// Clobbers
// See Section 3-14 of https://refspecs.linux-foundation.org/elf/elfspec_ppc.pdf

#![allow(unused_assignments)]
#![allow(unused_variables)]

pub mod nr;

const CR0_ERROR_MASK: usize = 1 << (core::mem::size_of::<usize>() * 8 - 4);

#[inline(always)]
pub unsafe fn syscall0(mut n: usize) -> (usize, bool) {
    let ret: usize;
    let mut is_err: usize;
    llvm_asm!("sc"
         : "+{r0}"(n) "={cr0}"(is_err) "={r3}"(ret)
         :
         : "memory" "r4" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    (ret, is_err & CR0_ERROR_MASK != 0)
}

#[inline(always)]
pub unsafe fn syscall1(mut n: usize, mut a1: usize) -> (usize, bool) {
    let mut is_err: usize;
    llvm_asm!("sc"
         : "+{r0}"(n) "={cr0}"(is_err) "+{r3}"(a1)
         :
         : "cr0" "memory" "r4" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    (a1, is_err & CR0_ERROR_MASK != 0)
}

#[inline(always)]
pub unsafe fn syscall2(mut n: usize, mut a1: usize, mut a2: usize) -> (usize, bool) {
    let mut is_err: usize;
    llvm_asm!("sc"
         : "+{r0}"(n) "={cr0}"(is_err) "+{r3}"(a1) "+{r4}"(a2)
         :
         : "cr0" "memory" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    (a1, is_err & CR0_ERROR_MASK != 0)
}

#[inline(always)]
pub unsafe fn syscall3(mut n: usize, mut a1: usize, mut a2: usize, mut a3: usize) -> (usize, bool) {
    let mut is_err: usize;
    llvm_asm!("sc"
         : "+{r0}"(n) "={cr0}"(is_err) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3)
         :
         : "cr0" "memory" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    (a1, is_err & CR0_ERROR_MASK != 0)
}

#[inline(always)]
pub unsafe fn syscall4(
    mut n: usize,
    mut a1: usize,
    mut a2: usize,
    mut a3: usize,
    mut a4: usize,
) -> (usize, bool) {
    let mut is_err: usize;
    llvm_asm!("sc"
         : "+{r0}"(n) "={cr0}"(is_err) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
         :
         : "cr0" "memory" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    (a1, is_err & CR0_ERROR_MASK != 0)
}

#[inline(always)]
pub unsafe fn syscall5(
    mut n: usize,
    mut a1: usize,
    mut a2: usize,
    mut a3: usize,
    mut a4: usize,
    mut a5: usize,
) -> (usize, bool) {
    let mut is_err: usize;
    llvm_asm!("sc"
         : "+{r0}"(n) "={cr0}"(is_err) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
           "+{r7}"(a5)
         :
         : "cr0" "memory" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    (a1, is_err & CR0_ERROR_MASK != 0)
}

#[inline(always)]
pub unsafe fn syscall6(
    mut n: usize,
    mut a1: usize,
    mut a2: usize,
    mut a3: usize,
    mut a4: usize,
    mut a5: usize,
    mut a6: usize,
) -> (usize, bool) {
    let mut is_err: usize;
    llvm_asm!("sc"
         : "+{r0}"(n) "={cr0}"(is_err) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
           "+{r7}"(a5) "+{r8}"(a6)
         :
         : "cr0" "memory" "r9" "r10" "r11" "r12"
         : "volatile");
    (a1, is_err & CR0_ERROR_MASK != 0)
}

#[inline(always)]
pub unsafe fn syscall0_nofail(mut n: usize) -> usize {
    let ret: usize;
    llvm_asm!("sc"
         : "+{r0}"(n) "={r3}"(ret)
         :
         : "cr0" "memory" "r4" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1_nofail(mut n: usize, mut a1: usize) -> usize {
    llvm_asm!("sc"
         : "+{r0}"(n) "+{r3}"(a1)
         :
         : "cr0" "memory" "r4" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall2_nofail(mut n: usize, mut a1: usize, mut a2: usize) -> usize {
    llvm_asm!("sc"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2)
         :
         : "cr0" "memory" "r5" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall3_nofail(mut n: usize, mut a1: usize, mut a2: usize, mut a3: usize) -> usize {
    llvm_asm!("sc"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3)
         :
         : "cr0" "memory" "r6" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall4_nofail(
    mut n: usize,
    mut a1: usize,
    mut a2: usize,
    mut a3: usize,
    mut a4: usize,
) -> usize {
    llvm_asm!("sc"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
         :
         : "cr0" "memory" "r7" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall5_nofail(
    mut n: usize,
    mut a1: usize,
    mut a2: usize,
    mut a3: usize,
    mut a4: usize,
    mut a5: usize,
) -> usize {
    llvm_asm!("sc"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
           "+{r7}"(a5)
         :
         : "cr0" "memory" "r8" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}

#[inline(always)]
pub unsafe fn syscall6_nofail(
    mut n: usize,
    mut a1: usize,
    mut a2: usize,
    mut a3: usize,
    mut a4: usize,
    mut a5: usize,
    mut a6: usize,
) -> usize {
    llvm_asm!("sc"
         : "+{r0}"(n) "+{r3}"(a1) "+{r4}"(a2) "+{r5}"(a3) "+{r6}"(a4)
           "+{r7}"(a5) "+{r8}"(a6)
         :
         : "cr0" "memory" "r9" "r10" "r11" "r12"
         : "volatile");
    a1
}
