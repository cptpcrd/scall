// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use scall::{syscall, syscall_raw};

#[test]
fn test_ebadf() {
    static MESSAGE: &'static str = "Hello, world!";

    unsafe {
        assert_eq!(
            syscall!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            Err(9)
        );

        #[cfg(target_os = "linux")]
        assert_eq!(
            syscall_raw!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            -9isize as usize
        );

        #[cfg(any(target_os = "freebsd", target_os = "macos"))]
        assert_eq!(
            syscall_raw!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            (9, true)
        );
    }
}

#[test]
fn test_fsync() {
    unsafe {
        use std::os::unix::io::AsRawFd;
        let file = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();

        assert_eq!(syscall!(FSYNC, file.as_raw_fd()), Ok(0));
    }
}

#[test]
fn test_kill() {
    unsafe {
        assert_eq!(syscall!(KILL, 0, 0), Ok(0));
        assert_eq!(syscall!(KILL, std::process::id(), 0), Ok(0));

        #[cfg(target_os = "linux")]
        assert_eq!(syscall_raw!(KILL, 0, 0), 0);
        #[cfg(any(target_os = "freebsd", target_os = "macos"))]
        assert_eq!(syscall_raw!(KILL, 0, 0), (0, false));
    }
}

#[test]
fn test_getpid() {
    let pid = unsafe { syscall!(GETPID) }.unwrap();

    assert_eq!(pid, std::process::id() as usize);

    #[cfg(target_os = "linux")]
    {
        assert_eq!(unsafe { scall::syscall0(scall::nr::GETPID) }, pid);
        assert_eq!(unsafe { syscall_raw!(GETPID) }, pid);
    }

    #[cfg(any(target_os = "freebsd", target_os = "macos"))]
    {
        assert_eq!(unsafe { scall::syscall0(scall::nr::GETPID) }, (pid, false));
        assert_eq!(unsafe { syscall_raw!(GETPID) }, (pid, false));
    }
}

#[test]
fn test_faccessat() {
    #[cfg(any(target_os = "freebsd", target_os = "linux"))]
    const AT_FDCWD: i32 = -100;
    #[cfg(target_os = "macos")]
    const AT_FDCWD: i32 = -2;

    const F_OK: i32 = 0;

    #[cfg(target_os = "linux")]
    const ENOSYS: i32 = 38;

    unsafe {
        assert_eq!(
            syscall!(FACCESSAT, AT_FDCWD, b"/\0".as_ptr(), F_OK, 0),
            Ok(0)
        );

        #[cfg(target_os = "linux")]
        {
            let res = syscall!(FACCESSAT2, AT_FDCWD, b"/\0".as_ptr(), F_OK, 0);

            assert!(res == Ok(0) || res == Err(ENOSYS), "{:?}", res);
        }
    }
}

#[cfg(target_os = "linux")]
#[test]
fn test_prctl() {
    const PR_GET_KEEPCAPS: usize = 7;
    const PR_SET_KEEPCAPS: usize = 8;

    unsafe {
        let old_keepcaps = syscall!(PRCTL, PR_GET_KEEPCAPS, 0, 0, 0, 0).unwrap();

        syscall!(PRCTL, PR_SET_KEEPCAPS, 0, 0, 0, 0).unwrap();
        assert_eq!(syscall!(PRCTL, PR_GET_KEEPCAPS, 0, 0, 0, 0), Ok(0));

        syscall!(PRCTL, PR_SET_KEEPCAPS, 1, 0, 0, 0).unwrap();
        assert_eq!(syscall!(PRCTL, PR_GET_KEEPCAPS, 0, 0, 0, 0), Ok(1));

        syscall!(PRCTL, PR_GET_KEEPCAPS, old_keepcaps, 0, 0, 0).unwrap();
    }
}

#[cfg(target_os = "freebsd")]
#[test]
fn test_procctl() {
    const P_PID: usize = 0;
    const PROC_REAP_ACQUIRE: usize = 2;
    const PROC_REAP_RELEASE: usize = 3;
    const EBUSY: i32 = 16;
    const EINVAL: i32 = 22;

    unsafe {
        let pid = syscall!(GETPID).unwrap();

        assert_eq!(syscall!(PROCCTL, P_PID, pid, PROC_REAP_ACQUIRE, 0), Ok(0));
        assert_eq!(
            syscall!(PROCCTL, P_PID, pid, PROC_REAP_ACQUIRE, 0),
            Err(EINVAL)
        );

        assert_eq!(syscall!(PROCCTL, P_PID, pid, PROC_REAP_RELEASE, 0), Ok(0));
        assert_eq!(
            syscall!(PROCCTL, P_PID, pid, PROC_REAP_RELEASE, 0),
            Err(EBUSY)
        );
    }
}
