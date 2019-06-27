use crate::{Error, ErrorKind};
use libc::{flock, EWOULDBLOCK, LOCK_EX, LOCK_NB, LOCK_UN};
use std::os::unix::io::AsRawFd;
use std::ops;

/// A guard that unlocks the file descriptor when it goes out of scope.
#[derive(Debug)]
pub struct FdLockGuard<'fdlock, T: AsRawFd> {
    lock: &'fdlock mut FdLock<T>,
}

impl<T: AsRawFd> ops::Deref for FdLockGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.t
    }
}

impl<T: AsRawFd> ops::DerefMut for FdLockGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock.t
    }
}

impl<T: AsRawFd> Drop for FdLockGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let fd = self.lock.t.as_raw_fd();
        if unsafe { flock(fd, LOCK_UN | LOCK_NB) } != 0 {
            panic!("Could not unlock the file descriptor");
        }
    }
}

/// A file descriptor lock.
#[derive(Debug)]
pub struct FdLock<T: AsRawFd> {
    t: T
}

impl<T: AsRawFd> FdLock<T> {
    /// Create a new instance.
    #[inline]
    pub fn new(t: T) -> Self {
        FdLock { t }
    }

    /// Acquires a new lock, blocking the current thread until it's able to do so.
    ///
    /// This function will block the local thread until it is available to acquire the lock. Upon
    /// returning, the thread is the only thread with the lock held. An RAII guard is returned to allow
    /// scoped unlock of the lock. When the guard goes out of scope, the lock will be unlocked.
    #[inline]
    pub fn lock(&mut self) -> Result<FdLockGuard<'_, T>, Error> {
        let fd = self.t.as_raw_fd();
        match unsafe { flock(fd, LOCK_EX) } {
            0 => Ok(FdLockGuard { lock: self }),
            _ => Err(ErrorKind::Other.into()),
        }
    }

    /// Attempts to acquire this lock.
    ///
    /// If the lock could not be acquired at this time, then `Err` is returned. Otherwise, an RAII
    /// guard is returned. The lock will be unlocked when the guard is dropped.
    ///
    /// This function does not block.
    #[inline]
    pub fn try_lock(&mut self) -> Result<FdLockGuard<'_, T>, Error> {
        let fd = self.t.as_raw_fd();
        match unsafe { flock(fd, LOCK_EX | LOCK_NB) } {
            0 => Ok(FdLockGuard { lock: self }),
            EWOULDBLOCK => Err(ErrorKind::Locked.into()),
            _ => Err(ErrorKind::Other.into()),
        }
    }
}
