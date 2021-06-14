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
impl<'fd> AsFd<'fd> for &'fd tokio::fs::File {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'handle> AsHandle<'handle> for &'handle tokio::fs::File {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'handle> {
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
impl<'fd> AsFd<'fd> for &'fd tokio::net::TcpStream {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'socket> AsSocket<'socket> for &'socket tokio::net::TcpStream {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'socket> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd tokio::net::TcpListener {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'socket> AsSocket<'socket> for &'socket tokio::net::TcpListener {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'socket> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd tokio::net::UdpSocket {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'socket> AsSocket<'socket> for &'socket tokio::net::UdpSocket {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'socket> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd tokio::io::Stdin {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'handle> AsHandle<'handle> for &'handle tokio::io::Stdin {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'handle> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd tokio::io::Stdout {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'handle> AsHandle<'handle> for &'handle tokio::io::Stdout {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'handle> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd tokio::io::Stderr {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'handle> AsHandle<'handle> for &'handle tokio::io::Stderr {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'handle> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(unix)]
impl<'fd> AsFd<'fd> for &'fd tokio::net::UnixStream {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'fd> AsFd<'fd> for &'fd tokio::net::UnixListener {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'fd> AsFd<'fd> for &'fd tokio::net::UnixDatagram {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd tokio::process::ChildStdout {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'handle> AsHandle<'handle> for &'handle tokio::process::ChildStdin {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'handle> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl<'handle> AsHandle<'handle> for &'handle tokio::process::ChildStdout {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'handle> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl<'handle> AsHandle<'handle> for &'handle tokio::process::ChildStderr {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'handle> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}
