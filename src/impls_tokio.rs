//! Implementations of io-lifetimes' traits for tokio's types. In the
//! future, we'll prefer to have crates provide their own impls; this is
//! just a temporary measure.

#[cfg(any(unix, target_os = "wasi"))]
use crate::{AsFd, BorrowedFd, FromFd, OwnedFd};
#[cfg(windows)]
use crate::{AsHandle, AsSocket, BorrowedHandle, BorrowedSocket, FromHandle, OwnedHandle};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, AsRawSocket, FromRawHandle, IntoRawHandle};

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a tokio::fs::File {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a tokio::fs::File {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for tokio::fs::File {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromHandle for tokio::fs::File {
    #[inline]
    fn from_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a tokio::net::TcpStream {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsSocket<'a> for &'a tokio::net::TcpStream {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'a> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a tokio::net::TcpListener {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsSocket<'a> for &'a tokio::net::TcpListener {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'a> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a tokio::net::UdpSocket {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsSocket<'a> for &'a tokio::net::UdpSocket {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'a> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a tokio::io::Stdin {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a tokio::io::Stdin {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a tokio::io::Stdout {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a tokio::io::Stdout {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a tokio::io::Stderr {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a tokio::io::Stderr {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(unix)]
impl<'a> AsFd<'a> for &'a tokio::net::UnixStream {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'a> AsFd<'a> for &'a tokio::net::UnixListener {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'a> AsFd<'a> for &'a tokio::net::UnixDatagram {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a tokio::process::ChildStdout {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a tokio::process::ChildStdin {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a tokio::process::ChildStdout {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a tokio::process::ChildStderr {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}
