// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use scall::{syscall, syscall_nofail, syscall_raw};

#[test]
fn test_ebadf() {
    static MESSAGE: &'static str = "Hello, world!";

    unsafe {
        assert_eq!(
            syscall!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            Err(libc::EBADF)
        );

        // Note: This is `assert_ne`, NOT `assert_eq`.
        assert_ne!(
            syscall_nofail!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            0
        );

        #[cfg(target_os = "linux")]
        assert_eq!(
            syscall_raw!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            -libc::EBADF as usize
        );

        #[cfg(any(target_os = "freebsd", target_os = "macos"))]
        assert_eq!(
            syscall_raw!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            (libc::EBADF as usize, true)
        );
    }
}

#[test]
fn test_fsync() {
    unsafe {
        use std::os::unix::io::AsRawFd;
        let file = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();

        assert_eq!(syscall!(FSYNC, file.as_raw_fd()), Ok(0));
        assert_eq!(syscall_nofail!(FSYNC, file.as_raw_fd()), 0);
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

        assert_eq!(syscall_nofail!(KILL, 0, 0), 0);
    }
}

#[test]
fn test_getpid() {
    let pid = unsafe { syscall!(GETPID) }.unwrap();

    assert_eq!(pid, std::process::id() as usize);

    assert_eq!(unsafe { syscall_nofail!(GETPID) }, pid);

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
    unsafe {
        assert_eq!(
            syscall!(FACCESSAT, libc::AT_FDCWD, b"/\0".as_ptr(), libc::F_OK, 0),
            Ok(0)
        );

        assert_eq!(
            syscall_nofail!(FACCESSAT, libc::AT_FDCWD, b"/\0".as_ptr(), libc::F_OK, 0),
            0
        );
        #[cfg(target_os = "linux")]
        {
            let res = syscall!(FACCESSAT2, libc::AT_FDCWD, b"/\0".as_ptr(), libc::F_OK, 0);

            assert!(res == Ok(0) || res == Err(libc::ENOSYS), "{:?}", res);
        }
    }
}

#[cfg(target_os = "linux")]
#[test]
fn test_prctl() {
    unsafe {
        let old_keepcaps = syscall!(PRCTL, libc::PR_GET_KEEPCAPS, 0, 0, 0, 0).unwrap();

        syscall!(PRCTL, libc::PR_SET_KEEPCAPS, 0, 0, 0, 0).unwrap();

        assert_eq!(syscall!(PRCTL, libc::PR_GET_KEEPCAPS, 0, 0, 0, 0), Ok(0));
        assert_eq!(syscall_nofail!(PRCTL, libc::PR_GET_KEEPCAPS, 0, 0, 0, 0), 0);

        syscall!(PRCTL, libc::PR_SET_KEEPCAPS, 1, 0, 0, 0).unwrap();

        assert_eq!(syscall!(PRCTL, libc::PR_GET_KEEPCAPS, 0, 0, 0, 0), Ok(1));
        assert_eq!(syscall_nofail!(PRCTL, libc::PR_GET_KEEPCAPS, 0, 0, 0, 0), 1);

        syscall!(PRCTL, libc::PR_GET_KEEPCAPS, old_keepcaps, 0, 0, 0).unwrap();
    }
}

#[cfg(target_os = "freebsd")]
#[test]
fn test_procctl() {
    const PROC_REAP_ACQUIRE: usize = 2;
    const PROC_REAP_RELEASE: usize = 3;

    unsafe {
        let pid = syscall!(GETPID).unwrap();

        syscall!(PROCCTL, libc::P_PID, pid, PROC_REAP_RELEASE, 0);

        assert_eq!(
            syscall!(PROCCTL, libc::P_PID, pid, PROC_REAP_ACQUIRE, 0),
            Ok(0)
        );
        assert_eq!(
            syscall!(PROCCTL, libc::P_PID, pid, PROC_REAP_ACQUIRE, 0),
            Err(libc::EBUSY)
        );

        assert_eq!(
            syscall!(PROCCTL, libc::P_PID, pid, PROC_REAP_RELEASE, 0),
            Ok(0)
        );
        assert_eq!(
            syscall!(PROCCTL, libc::P_PID, pid, PROC_REAP_RELEASE, 0),
            Err(libc::EINVAL)
        );
    }
}
