// Copyright 2014 The syscall.rs Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use scall::{eno, syscall, syscall_nofail, syscall_raw};

#[test]
fn test_ebadf() {
    static MESSAGE: &str = "Hello, world!";

    unsafe {
        assert_eq!(
            syscall!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            Err(eno::EBADF)
        );

        // Note: This is `assert_ne`, NOT `assert_eq`.
        assert_ne!(
            syscall_nofail!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            0
        );

        #[cfg(scall_error = "packed")]
        assert_eq!(
            syscall_raw!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            -eno::EBADF as usize
        );

        #[cfg(scall_error = "flag")]
        assert_eq!(
            syscall_raw!(WRITE, -4isize, MESSAGE.as_ptr(), MESSAGE.len()),
            (eno::EBADF as usize, true)
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

        #[cfg(scall_error = "packed")]
        assert_eq!(syscall_raw!(KILL, 0, 0), 0);
        #[cfg(scall_error = "flag")]
        assert_eq!(syscall_raw!(KILL, 0, 0), (0, false));

        assert_eq!(syscall_nofail!(KILL, 0, 0), 0);
    }
}

#[test]
fn test_getpid() {
    let pid = unsafe { syscall!(GETPID) }.unwrap();

    assert_eq!(pid, std::process::id() as usize);

    assert_eq!(unsafe { syscall_nofail!(GETPID) }, pid);

    #[cfg(scall_error = "packed")]
    {
        assert_eq!(unsafe { scall::syscall0(scall::nr::GETPID) }, pid);
        assert_eq!(unsafe { syscall_raw!(GETPID) }, pid);
    }

    #[cfg(scall_error = "flag")]
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

            assert!(res == Ok(0) || res == Err(eno::ENOSYS), "{:?}", res);
        }
    }
}

#[cfg(target_os = "linux")]
#[test]
fn test_pread64() {
    use std::io::prelude::*;
    use std::os::unix::prelude::*;

    unsafe fn pread64(
        fd: libc::c_int,
        buf: &mut [u8],
        offset: libc::off_t,
    ) -> Result<usize, libc::c_int> {
        scall::syscall_args64!(PREAD64, fd, buf.as_mut_ptr(), buf.len(), @u64 offset)
    }

    unsafe {
        let mut file = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();

        let mut buf1 = [0; 1024];
        let mut buf2 = [0; 1024];

        file.read_exact(&mut buf1).unwrap();
        let n = pread64(file.as_raw_fd(), &mut buf2, 0).unwrap();
        assert!(n > 0);
        assert!(buf1.starts_with(&buf2[..n]));

        file.read_exact(&mut buf1).unwrap();
        let n = pread64(file.as_raw_fd(), &mut buf2, 1024).unwrap();
        assert!(n > 0);
        assert!(buf1.starts_with(&buf2[..n]));
    }
}

#[cfg(target_os = "linux")]
#[test]
fn test_readahead() {
    use std::os::unix::prelude::*;

    unsafe {
        let file = std::fs::File::open(std::env::current_exe().unwrap()).unwrap();

        scall::syscall_args64!(READAHEAD, file.as_raw_fd(), @u64 1024, 1024).unwrap();
        scall::syscall_args64!(READAHEAD, file.as_raw_fd(), @u64 1024, 1024,).unwrap();
    }
}

#[cfg(target_os = "linux")]
#[test]
fn test_sync_file_range() {
    use std::os::unix::prelude::*;

    unsafe {
        let file = std::fs::File::open("/").unwrap();

        #[cfg(not(any(
            target_arch = "powerpc",
            target_arch = "powerpc64",
            target_arch = "arm"
        )))]
        {
            scall::syscall_args64!(
                SYNC_FILE_RANGE,
                file.as_raw_fd(),
                @u64 1024,
                @u64 1024,
                libc::SYNC_FILE_RANGE_WRITE,
            )
            .unwrap();

            assert_eq!(
                scall::syscall_args64!(
                    SYNC_FILE_RANGE,
                    file.as_raw_fd(),
                    @u64 -1i64,
                    @u64 1024,
                    libc::SYNC_FILE_RANGE_WRITE,
                )
                .unwrap_err(),
                eno::EINVAL,
            );

            assert_eq!(
                scall::syscall_args64!(
                    SYNC_FILE_RANGE,
                    file.as_raw_fd(),
                    @u64 1024,
                    @u64 -1i64,
                    libc::SYNC_FILE_RANGE_WRITE,
                )
                .unwrap_err(),
                eno::EINVAL,
            );
        }

        #[cfg(any(
            target_arch = "powerpc",
            target_arch = "powerpc64",
            target_arch = "arm"
        ))]
        {
            scall::syscall_args64!(
                SYNC_FILE_RANGE2,
                file.as_raw_fd(),
                libc::SYNC_FILE_RANGE_WRITE,
                @u64 1024,
                @u64 1024,
            )
            .unwrap();

            assert_eq!(
                scall::syscall_args64!(
                    SYNC_FILE_RANGE2,
                    file.as_raw_fd(),
                    libc::SYNC_FILE_RANGE_WRITE,
                    @u64 -1i64,
                    @u64 1024,
                )
                .unwrap_err(),
                eno::EINVAL,
            );

            assert_eq!(
                scall::syscall_args64!(
                    SYNC_FILE_RANGE2,
                    file.as_raw_fd(),
                    libc::SYNC_FILE_RANGE_WRITE,
                    @u64 1024,
                    @u64 -1i64,
                )
                .unwrap_err(),
                eno::EINVAL,
            );
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

        syscall!(PRCTL, libc::PR_SET_KEEPCAPS, old_keepcaps, 0, 0, 0).unwrap();
    }
}

#[cfg(target_os = "linux")]
#[test]
fn test_epoll() {
    unsafe fn epoll_wait(
        epfd: libc::c_int,
        events: &mut [libc::epoll_event],
        timeout: libc::c_int,
    ) -> Result<usize, libc::c_int> {
        syscall!(
            EPOLL_PWAIT,
            epfd,
            events.as_mut_ptr(),
            events.len(),
            timeout,
            std::ptr::null::<libc::sigset_t>(),
            std::mem::size_of::<libc::sigset_t>(),
        )
    }

    unsafe {
        let epfd = syscall!(EPOLL_CREATE1, libc::EPOLL_CLOEXEC).unwrap() as libc::c_int;

        let mut events = [std::mem::zeroed::<libc::epoll_event>(); 2];

        assert_eq!(epoll_wait(epfd, &mut events, 0), Ok(0));

        // We have to call into libc to set up the pipe so it can handle the architecture
        // differences for us
        let mut pipefds = [0i32; 2];
        assert_eq!(libc::pipe2(pipefds.as_mut_ptr(), libc::O_CLOEXEC), 0);

        // Watch for events on the read end of the pipe
        events[0].u64 = pipefds[0] as u64;
        events[0].events = libc::EPOLLIN as u32;
        syscall!(
            EPOLL_CTL,
            epfd,
            libc::EPOLL_CTL_ADD,
            pipefds[0],
            &mut events[0] as *mut _,
        )
        .unwrap();

        // No events yet
        assert_eq!(epoll_wait(epfd, &mut events, 0), Ok(0));

        // Now write() some data in, and it should poll as ready
        assert_eq!(syscall!(WRITE, pipefds[1], &b'0' as *const _, 1), Ok(1));
        assert_eq!(epoll_wait(epfd, &mut events, 0), Ok(1));
        assert_eq!(events[0].u64, pipefds[0] as u64);
        assert_eq!(events[0].events, libc::EPOLLIN as u32);

        // Close both ends of the pipe
        syscall_nofail!(CLOSE, pipefds[0]);
        syscall_nofail!(CLOSE, pipefds[1]);

        // Now no events
        assert_eq!(epoll_wait(epfd, &mut events, 0), Ok(0));

        syscall_nofail!(CLOSE, epfd);
    }
}

#[cfg(any(target_os = "freebsd", target_os = "macos"))]
#[test]
fn test_kqueue() {
    unsafe fn kevent(
        kq: libc::c_int,
        changes: &[libc::kevent],
        events: &mut [libc::kevent],
        timeout: Option<libc::timespec>,
    ) -> Result<usize, libc::c_int> {
        let raw_timeout = if let Some(ref timeout) = timeout {
            timeout
        } else {
            std::ptr::null()
        };

        syscall!(
            KEVENT,
            kq,
            changes.as_ptr(),
            changes.len(),
            events.as_mut_ptr(),
            events.len(),
            raw_timeout,
        )
    }

    unsafe {
        let timeout_0 = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };

        let kq = syscall!(KQUEUE).unwrap() as libc::c_int;

        let mut events = [std::mem::zeroed::<libc::kevent>(); 2];

        assert_eq!(kevent(kq, &[], &mut events, Some(timeout_0)), Ok(0));

        let mut pipefds = [0i32; 2];
        assert_eq!(libc::pipe(pipefds.as_mut_ptr()), 0);

        // Watch for events on the read end of the pipe
        events[0] = std::mem::zeroed();
        events[0].ident = pipefds[0] as _;
        events[0].filter = libc::EVFILT_READ as _;
        events[0].flags = libc::EV_ADD as _;
        assert_eq!(kevent(kq, &events[..1], &mut [], None), Ok(0));

        // No events yet
        assert_eq!(kevent(kq, &[], &mut events, Some(timeout_0)), Ok(0));

        // Now write() some data in, and it should poll as ready
        assert_eq!(syscall!(WRITE, pipefds[1], &b'0' as *const _, 1), Ok(1));
        assert_eq!(kevent(kq, &[], &mut events, Some(timeout_0)), Ok(1));
        assert_eq!(events[0].ident, pipefds[0] as _);
        assert_eq!(events[0].filter, libc::EVFILT_READ as _);

        // Close both ends of the pipe
        syscall_nofail!(CLOSE, pipefds[0]);
        syscall_nofail!(CLOSE, pipefds[1]);

        // Now no events
        assert_eq!(kevent(kq, &[], &mut events, Some(timeout_0)), Ok(0));

        syscall_nofail!(CLOSE, kq);
    }
}

#[cfg(target_os = "freebsd")]
#[test]
fn test_procctl() {
    const PROC_REAP_ACQUIRE: usize = 2;
    const PROC_REAP_RELEASE: usize = 3;

    unsafe {
        let pid = syscall!(GETPID).unwrap();

        syscall_nofail!(PROCCTL, libc::P_PID, pid, PROC_REAP_RELEASE, 0);

        assert_eq!(
            syscall!(PROCCTL, libc::P_PID, pid, PROC_REAP_ACQUIRE, 0),
            Ok(0)
        );
        assert_eq!(
            syscall!(PROCCTL, libc::P_PID, pid, PROC_REAP_ACQUIRE, 0),
            Err(eno::EBUSY)
        );

        assert_eq!(
            syscall!(PROCCTL, libc::P_PID, pid, PROC_REAP_RELEASE, 0),
            Ok(0)
        );
        assert_eq!(
            syscall!(PROCCTL, libc::P_PID, pid, PROC_REAP_RELEASE, 0),
            Err(eno::EINVAL)
        );
    }
}
