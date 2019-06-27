use std::os::windows::io::{AsRawHandle, RawHandle};
use winapi::um::fileapi::{LockFile, UnlockFile};

/// A guard that unlocks the file descriptor when it goes out of scope.
#[derive(Debug)]
pub struct LockGuard {
    handle: HANDLE,
}

impl Drop for LockGuard {
    #[inline]
    fn drop(&mut self) {
        unlock(self.fd).expect("Could not unlock the handle");
    }
}

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

/// Extend `AsRawHandle` with advisory locking capabilities.
pub trait AsRawHandleExt: AsRawHandle {
    /// Lock the current file descriptor.
    #[inline]
    fn lock_file(&mut self) -> Result<LockGuard, Error> {
        lock(self.as_raw_fd())
    }
}
