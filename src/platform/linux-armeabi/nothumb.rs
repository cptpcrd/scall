#[inline(always)]
pub unsafe fn syscall0(n: usize) -> usize {
    let ret: usize;
    asm!(
        "swi #0",
        in("r7") n,
        out("r0") ret,
        out("lr") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> usize {
    let ret: usize;
    asm!(
        "swi #0",
        in("r7") n,
        inout("r0") a1 => ret,
        out("lr") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    asm!(
        "swi #0",
        in("r7") n,
        inout("r0") a1 => ret,
        in("r1") a2,
        out("lr") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let ret: usize;
    asm!(
        "swi #0",
        in("r7") n,
        inout("r0") a1 => ret,
        in("r1") a2,
        in("r2") a3,
        out("lr") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let ret: usize;
    asm!(
        "swi #0",
        in("r7") n,
        inout("r0") a1 => ret,
        in("r1") a2,
        in("r2") a3,
        in("r3") a4,
        out("lr") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
    let ret: usize;
    asm!(
        "swi #0",
        in("r7") n,
        inout("r0") a1 => ret,
        in("r1") a2,
        in("r2") a3,
        in("r3") a4,
        in("r4") a5,
        out("lr") _,
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
    asm!(
        "swi #0",
        in("r7") n,
        inout("r0") a1 => ret,
        in("r1") a2,
        in("r2") a3,
        in("r3") a4,
        in("r4") a5,
        in("r5") a6,
        out("lr") _,
    );
    ret
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
) -> usize {
    let ret: usize;
    asm!(
        "push {{r6}}",
        "mov r6, {}",
        "swi #0",
        "pop {{r6}}",
        in(reg) a7,
        in("r7") n,
        inout("r0") a1 => ret,
        in("r1") a2,
        in("r2") a3,
        in("r3") a4,
        in("r4") a5,
        in("r5") a6,
        out("lr") _,
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
pub use syscall7 as syscall7_nofail;
