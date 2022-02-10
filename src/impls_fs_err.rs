//! Implementations of io-lifetimes' traits for fs_err's types. In the
//! future, we'll prefer to have crates provide their own impls; this is
//! just a temporary measure.

#[cfg(any(unix, target_os = "wasi"))]
use crate::{AsFd, BorrowedFd, IntoFd, OwnedFd};
#[cfg(windows)]
use crate::{AsHandle, BorrowedHandle, IntoHandle, OwnedHandle};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle};

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a fs_err::File {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a fs_err::File {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for fs_err::File {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoHandle for fs_err::File {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(self.into_raw_handle()) }
    }
}
