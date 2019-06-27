//! Advisory cross-platform lock on a file using a file descriptor to it.
//!
//! ## Example
//! ```rust
//! // tbi
//! ```

#![forbid(future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

pub use raw_lock::*;

#[cfg(windows)]
mod raw_lock {
    use std::os::windows::raw::HANDLE;
    use winapi::um::fileapi::LockFile;

    pub fn lock_fd(handle: HANDLE) -> bool {
        unsafe { LockFile(handle, 0, 0, 1, 0) }
    }
}

#[cfg(unix)]
mod raw_lock {
    use libc::{flock, LOCK_EX, LOCK_NB};
    use std::os::unix::io::RawFd;

    /// Lock a file descriptor.
    pub fn lock_fd(fd: RawFd) -> bool {
        if unsafe { flock(fd, LOCK_EX | LOCK_NB) } == 0 {
            true
        } else {
            false
        }
    }
}
