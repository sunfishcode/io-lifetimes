use crate::{Error, ErrorKind};
use libc::{flock, EWOULDBLOCK, LOCK_EX, LOCK_NB, LOCK_UN};
use std::os::unix::io::AsRawFd;
use std::ops;

/// A guard that unlocks the file descriptor when it goes out of scope.
#[derive(Debug)]
pub struct LockGuard<T: AsRawFd> {
    t: T,
}

impl<T: AsRawFd> ops::Deref for LockGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

impl<T: AsRawFd> ops::DerefMut for LockGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.t
    }
}

impl <T: AsRawFd>Drop for LockGuard<T> {
    #[inline]
    fn drop(&mut self) {
        let fd = self.t.as_raw_fd();
        if unsafe { flock(fd, LOCK_UN | LOCK_NB) } != 0 {
            panic!("Could not unlock the file descriptor");
        }
    }
}

/// Lock a file descriptor.
#[inline]
pub fn lock<T: AsRawFd>(t: T) -> Result<LockGuard<T>, Error> {
    let fd = t.as_raw_fd();
    match unsafe { flock(fd, LOCK_EX | LOCK_NB) } {
        0 => Ok(LockGuard { t }),
        EWOULDBLOCK => Err(ErrorKind::Locked.into()),
        _ => Err(ErrorKind::Other.into()),
    }
}

/// Extend `AsRawFd` with advisory locking capabilities.
pub trait AsRawFdExt: AsRawFd + Sized {
    /// Lock the current file descriptor.
    #[inline]
    fn lock_file(self) -> Result<LockGuard<Self>, Error> {
        lock(self)
    }
}

impl<T: AsRawFd> AsRawFdExt for T {}
