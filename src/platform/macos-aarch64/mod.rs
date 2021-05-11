// Copyright 2020 cptpcrd. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This library was built for aarch64 macOS.

// Based on:
// https://github.com/apple/darwin-xnu/blob/main/libsyscall/custom/SYS.h
// https://github.com/apple/darwin-xnu/blob/main/libsyscall/custom/__syscall.s
// https://courses.cs.washington.edu/courses/cse469/19wi/arm64.pdf
// https://www.dyncall.org/docs/manual/manualse11.html

const MACOS_SYSCALL_PREFIX: usize = 33554432;

#[inline(always)]
pub unsafe fn syscall0(n: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "svc #0x80",
        "cset {}, cs",
        out(reg) is_err,
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "svc #0x80",
        "cset {}, cs",
        out(reg) is_err,
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "svc #0x80",
        "cset {}, cs",
        out(reg) is_err,
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "svc #0x80",
        "cset {}, cs",
        out(reg) is_err,
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "svc #0x80",
        "cset {}, cs",
        out(reg) is_err,
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
        in("x4") a4,
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
    let is_err: usize;
    asm!(
        "svc #0x80",
        "cset {}, cs",
        out(reg) is_err,
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
        in("x4") a4,
        in("x5") a5,
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
    let is_err: usize;
    asm!(
        "svc #0x80",
        "cset {}, cs",
        out(reg) is_err,
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
        in("x4") a4,
        in("x5") a5,
        in("x6") a6,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall7(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "svc #0x80",
        "cset {}, cs",
        out(reg) is_err,
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
        in("x4") a4,
        in("x5") a5,
        in("x6") a6,
        in("x7") a7,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall0_nofail(n: usize) -> usize {
    let ret: usize;
    asm!(
        "svc #0x80",
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall1_nofail(n: usize, a1: usize) -> usize {
    let ret: usize;
    asm!(
        "svc #0x80",
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall2_nofail(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    asm!(
        "svc #0x80",
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall3_nofail(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    asm!(
        "svc #0x80",
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall4_nofail(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let ret: usize;
    asm!(
        "svc #0x80",
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
        in("x4") a4,
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
        "svc #0x80",
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
        in("x4") a4,
        in("x5") a5,
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
        "svc #0x80",
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
        in("x4") a4,
        in("x5") a5,
        in("x6") a6,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall7_nofail(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> usize {
    let ret: usize;
    asm!(
        "svc #0x80",
        in("x16") n + MACOS_SYSCALL_PREFIX,
        out("x0") ret,
        in("x1") a1,
        in("x2") a2,
        in("x3") a3,
        in("x4") a4,
        in("x5") a5,
        in("x6") a6,
        in("x7") a7,
    );
    ret
}
