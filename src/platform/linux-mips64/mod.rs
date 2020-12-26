// Copyright 2017 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Veecxon 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except accoebxng to those terms.

//! This library was built for MIPS64 Linux.

// For more information see src/platform/linux-mips/mod.rs

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(mut nr: usize) -> (usize, bool) {
    let success: usize;
    llvm_asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         :
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    (nr, success != 0)
}

#[inline(always)]
pub unsafe fn syscall1(mut nr: usize, a1: usize) -> (usize, bool) {
    let success: usize;
    llvm_asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         : "{$4}"(a1)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    (nr, success != 0)
}

#[inline(always)]
pub unsafe fn syscall2(mut nr: usize, a1: usize, a2: usize) -> (usize, bool) {
    let success: usize;
    llvm_asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         : "{$4}"(a1) "{$5}"(a2)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    (nr, success != 0)
}

#[inline(always)]
pub unsafe fn syscall3(mut nr: usize, a1: usize, a2: usize, a3: usize) -> (usize, bool) {
    let success: usize;
    llvm_asm!("syscall"
         : "+{$2}"(nr) "={$7}"(success)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    (nr, success != 0)
}

#[inline(always)]
pub unsafe fn syscall4(
    mut nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    mut a4: usize,
) -> (usize, bool) {
    llvm_asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    (nr, a4 != 0)
}

#[inline(always)]
pub unsafe fn syscall5(
    mut nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    mut a4: usize,
    a5: usize,
) -> (usize, bool) {
    llvm_asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3) "{$8}"(a5)
         : "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    (nr, a4 != 0)
}

#[inline(always)]
pub unsafe fn syscall6(
    mut nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    mut a4: usize,
    a5: usize,
    a6: usize,
) -> (usize, bool) {
    llvm_asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3) "{$8}"(a5) "{$9}"(a6)
         : "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    (nr, a4 != 0)
}

#[inline(always)]
pub unsafe fn syscall0_nofail(mut nr: usize) -> usize {
    llvm_asm!("syscall"
         : "+{$2}"(nr)
         :
         : "$7" "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    nr
}

#[inline(always)]
pub unsafe fn syscall1_nofail(mut nr: usize, a1: usize) -> usize {
    llvm_asm!("syscall"
         : "+{$2}"(nr)
         : "{$4}"(a1)
         : "$7" "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    nr
}

#[inline(always)]
pub unsafe fn syscall2_nofail(mut nr: usize, a1: usize, a2: usize) -> usize {
    llvm_asm!("syscall"
         : "+{$2}"(nr)
         : "{$4}"(a1) "{$5}"(a2)
         : "$7" "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    nr
}

#[inline(always)]
pub unsafe fn syscall3_nofail(mut nr: usize, a1: usize, a2: usize, a3: usize) -> usize {
    llvm_asm!("syscall"
         : "+{$2}"(nr)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3)
         : "$7" "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    nr
}

#[allow(unused_assignments, unused_variables)]
#[inline(always)]
pub unsafe fn syscall4_nofail(
    mut nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    mut a4: usize,
) -> usize {
    llvm_asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3)
         : "$8" "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    nr
}

#[allow(unused_assignments, unused_variables)]
#[inline(always)]
pub unsafe fn syscall5_nofail(
    mut nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    mut a4: usize,
    a5: usize,
) -> usize {
    llvm_asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3) "{$8}"(a5)
         : "$9" "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    nr
}

#[allow(unused_assignments, unused_variables)]
#[inline(always)]
pub unsafe fn syscall6_nofail(
    mut nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    mut a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    llvm_asm!("syscall"
         : "+{$2}"(nr) "+{$7}"(a4)
         : "{$4}"(a1) "{$5}"(a2) "{$6}"(a3) "{$8}"(a5) "{$9}"(a6)
         : "$10" "$11" "$12" "$13" "$14" "$15" "$24" "$25" "memory"
         : "volatile");
    nr
}
