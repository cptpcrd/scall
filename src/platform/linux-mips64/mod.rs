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

pub mod eno;
pub mod nr;

#[inline(always)]
pub unsafe fn syscall0(nr: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        out("$7") is_err,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall1(nr: usize, a1: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        out("$7") is_err,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall2(nr: usize, a1: usize, a2: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        out("$7") is_err,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall3(nr: usize, a1: usize, a2: usize, a3: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        in("$6") a3,
        out("$7") is_err,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall4(nr: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        in("$6") a3,
        inout("$7") a4 => is_err,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall5(
    nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> (usize, bool) {
    let ret: usize;
    let is_err: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        in("$6") a3,
        inout("$7") a4 => is_err,
        inout("$8") a5 => _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall6(
    nr: usize,
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
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        in("$6") a3,
        inout("$7") a4 => is_err,
        inout("$8") a5 => _,
        inout("$9") a6 => _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    (ret, is_err != 0)
}

#[inline(always)]
pub unsafe fn syscall0_nofail(nr: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        out("$7") _,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall1_nofail(nr: usize, a1: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        out("$7") _,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall2_nofail(nr: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        out("$7") _,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall3_nofail(nr: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        in("$6") a3,
        out("$7") _,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall4_nofail(nr: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        in("$6") a3,
        inout("$7") a4 => _,
        out("$8") _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall5_nofail(
    nr: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        in("$6") a3,
        inout("$7") a4 => _,
        inout("$8") a5 => _,
        out("$9") _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall6_nofail(
    nr: usize,
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
        inout("$2") nr => ret,
        in("$4") a1,
        in("$5") a2,
        in("$6") a3,
        inout("$7") a4 => _,
        inout("$8") a5 => _,
        inout("$9") a6 => _,
        out("$10") _,
        out("$11") _,
        out("$12") _,
        out("$13") _,
        out("$14") _,
        out("$15") _,
        out("$24") _,
        out("$25") _,
    );
    ret
}
