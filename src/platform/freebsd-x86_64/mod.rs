// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library was built for x86-64 FreeBSD.

pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: bool;
    llvm_asm!("
        syscall
        adcb %bl, %bl"
        : "={rax}"(ret), "={bl}"(is_err)
        : "{rax}"(n), "{bl}"(0)
        : "rcx", "r11", "memory"
        : "volatile");
    (ret, is_err)
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: bool;
    llvm_asm!("
        syscall
        adcb %bl, %bl" : "={rax}"(ret), "={bl}"(is_err)
        : "{rax}"(n), "{rdi}"(a1), "{bl}"(0)
        : "rcx", "r11", "memory"
        : "volatile");
    (ret, is_err)
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: bool;
    llvm_asm!("
        syscall
        adcb %bl, %bl" : "={rax}"(ret), "={bl}"(is_err)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{bl}"(0)
        : "rcx", "r11", "memory"
        : "volatile");
    (ret, is_err)
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: bool;
    llvm_asm!("
        syscall
        adcb %bl, %bl" : "={rax}"(ret), "={bl}"(is_err)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3), "{bl}"(0)
        : "rcx", "r11", "memory"
        : "volatile");
    (ret, is_err)
}

#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: bool;
    llvm_asm!("
        syscall
        adcb %bl, %bl" : "={rax}"(ret), "={bl}"(is_err)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{bl}"(0)
        : "rcx", "r11", "memory"
        : "volatile");
    (ret, is_err)
}

#[inline(always)]
pub unsafe fn syscall5(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> (usize, bool) {
    let ret: usize;
    let is_err: bool;
    llvm_asm!("
        syscall
        adcb %bl, %bl" : "={rax}"(ret), "={bl}"(is_err)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5), "{bl}"(0)
        : "rcx", "r11", "memory"
        : "volatile");
    (ret, is_err)
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
) -> (usize, bool) {
    let ret: usize;
    let is_err: bool;
    llvm_asm!("
        syscall
        adcb %bl, %bl" : "={rax}"(ret), "={bl}"(is_err)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5), "{r9}"(a6), "{bl}"(0)
        : "rcx", "r11", "memory"
        : "volatile");
    (ret, is_err)
}

#[inline(always)]
pub unsafe fn syscall0_nofail(n: usize) -> (usize, bool) {
    let ret: usize;
    llvm_asm!("syscall"
        : "={rax}"(ret)
        : "{rax}"(n)
        : "rcx", "r11", "memory"
        : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall1_nofail(n: usize, a1: usize) -> (usize, bool) {
    let ret: usize;
    llvm_asm!("syscall" : "={rax}"(ret)
        : "{rax}"(n), "{rdi}"(a1)
        : "rcx", "r11", "memory"
        : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall2_nofail(n: usize, a1: usize, a2: usize) -> (usize, bool) {
    let ret: usize;
    llvm_asm!("syscall" : "={rax}"(ret)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2)
        : "rcx", "r11", "memory"
        : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall3_nofail(n: usize, a1: usize, a2: usize, a3: usize) -> (usize, bool) {
    let ret: usize;
    llvm_asm!("syscall" : "={rax}"(ret)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3)
        : "rcx", "r11", "memory"
        : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall4_nofail(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> (usize, bool) {
    let ret: usize;
    llvm_asm!("syscall" : "={rax}"(ret)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4)
        : "rcx", "r11", "memory"
        : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall5_nofail(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> (usize, bool) {
    let ret: usize;
    llvm_asm!("syscall" : "={rax}"(ret)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5)
        : "rcx", "r11", "memory"
        : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn syscall6_nofail(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> (usize, bool) {
    let ret: usize;
    llvm_asm!("syscall" : "={rax}"(ret)
        : "{rax}"(n), "{rdi}"(a1), "{rsi}"(a2), "{rdx}"(a3),
                     "{r10}"(a4), "{r8}"(a5), "{r9}"(a6)
        : "rcx", "r11", "memory"
        : "volatile");
    ret
}
