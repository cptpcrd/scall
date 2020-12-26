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
    let is_err: u8;
    asm!(
        "syscall",
        "adc {0}, {0}",
        inout(reg_byte) 0u8 => is_err,
        inout("rax") n => ret,
        out("rcx") _,
        out("r11") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: u8;
    asm!(
        "syscall",
        "adc {0}, {0}",
        inout(reg_byte) 0u8 => is_err,
        inout("rax") n => ret,
        in("rdi") a1,
        out("rcx") _,
        out("r11") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: u8;
    asm!(
        "syscall",
        "adc {0}, {0}",
        inout(reg_byte) 0u8 => is_err,
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        out("rcx") _,
        out("r11") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: u8;
    asm!(
        "syscall",
        "adc {0}, {0}",
        inout(reg_byte) 0u8 => is_err,
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        out("rcx") _,
        out("r11") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: u8;
    asm!(
        "syscall",
        "adc {0}, {0}",
        inout(reg_byte) 0u8 => is_err,
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        out("rcx") _,
        out("r11") _,
    );
    (ret, is_err != 0)
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
    let is_err: u8;
    asm!(
        "syscall",
        "adc {0}, {0}",
        inout(reg_byte) 0u8 => is_err,
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        out("rcx") _,
        out("r11") _,
    );
    (ret, is_err != 0)
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
    let is_err: u8;
    asm!(
        "syscall",
        "adc {0}, {0}",
        inout(reg_byte) 0u8 => is_err,
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        in("r9") a6,
        out("rcx") _,
        out("r11") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall0_nofail(n: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("rax") n => ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall1_nofail(n: usize, a1: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall2_nofail(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall3_nofail(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall4_nofail(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        out("rcx") _,
        out("r11") _,
    );
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
) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        out("rcx") _,
        out("r11") _,
    );
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
) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("rax") n => ret,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        in("r9") a6,
        out("rcx") _,
        out("r11") _,
    );
    ret
}
