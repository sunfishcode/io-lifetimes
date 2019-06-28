use std::os::windows::io::{AsRawHandle, RawHandle};
use winapi::um::fileapi::{LockFile, LockFileEx, UnlockFile, LOCKFILE_EXCLUSIVE_LOCK};
use std::ops;

/// Lock a file descriptor.
#[inline]
pub fn lock(handle: RawHandle) -> Result<LockGuard, Error> {
    if unsafe { LockFile(handle, 0, 0, 1, 0) } {
        Ok(LockGuard { handle })
    } else {
        Err(ErrorKind::Locked.into())
    }
}

/// Unlock a file descriptor.
#[inline]
fn unlock(handle: HANDLE) -> Result<(), Error> {
    if unsafe { UnlockFile(handle, 0, 0, 1, 0) } {
        Ok(())
    } else {
        Err(ErrorKind::Locked.into())
    }
}

/// A guard that unlocks the file descriptor when it goes out of scope.
#[derive(Debug)]
pub struct FdLockGuard<'fdlock, T: AsRawHandle> {
    lock: &'fdlock mut FdLock<T>,
}

impl<T: AsRawHandle> ops::Deref for FdLockGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.t
    }
}

impl<T: AsRawHandle> ops::DerefMut for FdLockGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock.t
    }
}

impl<T: AsRawHandle> Drop for FdLockGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let fd = self.lock.t.as_raw_fd();
        if unsafe { !UnlockFile(handle, 0, 0, 1, 0) } {
            panic!("Could not unlock the file descriptor");
        }
    }
}

/// A file descriptor lock.
#[derive(Debug)]
pub struct FdLock<T: AsRawHandle> {
    t: T
}

impl<T: AsRawHandle> FdLock<T> {
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
        if unsafe { LockFileEx(handle, LOCKFILE_EXCLUSIVE_LOCK, 0, 1, 0) } {
            Ok(FdLockGuard { lock: self })
        } else {
            Err(ErrorKind::Other.into())
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
        if unsafe { LockFile(handle, 0, 0, 1, 0) } {
            Ok(FdLockGuard { lock: self })
        } else {
            Err(ErrorKind::Other.into())
        }
    }
}
