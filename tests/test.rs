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
    let pid = unsafe { scall::syscall!(GETPID) }.unwrap();

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
