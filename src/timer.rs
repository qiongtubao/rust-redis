use libc::{c_int, c_void};
use mio::unix::EventedFd;
use mio::{Evented, Poll, PollOpt, Ready, Token};
use std::io;
use std::mem::MaybeUninit;
use std::os::unix::io::{AsRawFd, RawFd};
use std::time::Duration;

pub struct TimerFd {
    fd: c_int,
}
impl TimerFd {
    pub fn new(clockid: ClockId) -> io::Result<Self> {
        let flags = libc::TFD_NONBLOCK | libc::TFD_CLOEXEC;
        Self::create(clockid.into(), flags)
    }
    pub fn set_timeout(&mut self, timeout: &Duration) -> io::Result<()> {
        let new_value = libc::itimerspec {
            it_interval: libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            it_value: lib::timespec {
                tv_sec: timeout.as_secs() as libc::time_t,
                tv_nsec: timeout.subsec_nanos() as libc::c_long,
            },
        };
        self.settime(0, &new_value).map(|_old_value| ())
    }
    pub fn set_timeout_interval(&mut self, timeout: &Duration) -> io::Result<()> {
        let new_value = libc::itimerspec {
            it_interval: libc::timespec {
                tv_sec: timeout.as_secs() as libc::time_t,
                tv_nsec: timeout.subsec_nanos() as libc::c_long,
            },
            it_value: libc::timespec {
                tv_sec: timeout.as_secs() as libc::time_t,
                tv_nsec: timeout.subsec_nanos() as libc::c_long,
            },
        };
        self.settime(0, &new_value).map(|_old_value| ())
    }
    pub fn disarm(&mut self) -> io::Result<()> {
        self.set_timeout(&Duration::from_secs(0))
    }
    pub fn read(&self) -> io::Result<u64> {
        let mut buf = [0u8; 8];
        let ret = unsafe { libc::read(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len()) };
        if ret == 8 {
            Ok(u64::from_ne_bytes(buf))
        } else if ret == -1 {
            let errno = unsafe { *libc::__errno_location() };
            if errno == libc::EAGAIN {
                Ok(0)
            } else {
                Err(io::Error::from_raw_os_error(errno))
            }
        } else {
            panic!("reading a timerfd should never yield {} bytes", ret);
        }
    }
    pub fn create(clockid: c_int, flags: c_int) -> io::Result<Self> {
        let fd = unsafe { libc::timerfd_create(clockid, flags) };
        if fd == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self { fd })
        }
    }
    pub fn settime(
        &mut self,
        flags: c_int,
        new_value: &libc::itimerspec,
    ) -> io::Result<libc::itimerspec> {
        let mut old_spec_mem = MaybeUninit::<libc::itimerspec>::uninit();
        let ret =
            unsafe { libc::timerfd_settime(self.fd, flags, new_value, old_spec_mem.as_mut_ptr()) };
        if ret == -1 {
            Err(io::Error::last_os_error())
        } else {
            let old_spec = unsafe { old_spec_mem.assume_init() };
            Ok(old_spec)
        }
    }

    /// Wrapper of `timerfd_gettime` from timerfd_create(7)
    pub fn gettime(&self) -> io::Result<libc::itimerspec> {
        let mut old_spec_mem = MaybeUninit::<libc::itimerspec>::uninit();
        let ret = unsafe { libc::timerfd_gettime(self.fd, old_spec_mem.as_mut_ptr()) };
        if ret == -1 {
            Err(io::Error::last_os_error())
        } else {
            let old_spec = unsafe { old_spec_mem.assume_init() };
            Ok(old_spec)
        }
    }
}

impl AsRawFd for TimerFd {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

impl Evented for TimerFd {
    fn register(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.fd).register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.fd).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.fd).deregister(poll)
    }
}

impl Drop for TimerFd {
    fn drop(&mut self) {
        let _ = unsafe { libc::close(self.fd) };
    }
}

//
//
// ClockId
//
//

/// Clock used to mark the progress of the timer. timerfd_create(7)
#[derive(Copy, Clone)]
pub enum ClockId {
    RealTime,
    Monotonic,
    BootTime,
    RealTimeAlarm,
    BootTimeAlarm,
}

impl Into<c_int> for ClockId {
    fn into(self) -> c_int {
        match self {
            ClockId::RealTime => libc::CLOCK_REALTIME,
            ClockId::Monotonic => libc::CLOCK_MONOTONIC,
            ClockId::BootTime => libc::CLOCK_BOOTTIME,
            ClockId::RealTimeAlarm => libc::CLOCK_REALTIME_ALARM,
            ClockId::BootTimeAlarm => libc::CLOCK_BOOTTIME_ALARM,
        }
    }
}
