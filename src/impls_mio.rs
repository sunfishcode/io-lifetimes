//! Implementations of io-lifetimes' traits for mio's types. In the
//! future, we'll prefer to have crates provide their own impls; this is
//! just a temporary measure.

#[cfg(any(unix, target_os = "wasi"))]
use crate::{AsFd, BorrowedFd, FromFd, IntoFd, OwnedFd};
#[cfg(windows)]
use crate::{AsSocket, BorrowedSocket, FromSocket, IntoSocket, OwnedSocket};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket};

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd mio::net::TcpStream {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'socket> AsSocket<'socket> for &'socket mio::net::TcpStream {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'socket> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for mio::net::TcpStream {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoSocket for mio::net::TcpStream {
    #[inline]
    fn into_socket(self) -> OwnedSocket {
        unsafe { OwnedSocket::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for mio::net::TcpStream {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromSocket for mio::net::TcpStream {
    #[inline]
    fn from_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd mio::net::TcpListener {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'socket> AsSocket<'socket> for &'socket mio::net::TcpListener {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'socket> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for mio::net::TcpListener {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoSocket for mio::net::TcpListener {
    #[inline]
    fn into_socket(self) -> OwnedSocket {
        unsafe { OwnedSocket::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for mio::net::TcpListener {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromSocket for mio::net::TcpListener {
    #[inline]
    fn from_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd mio::net::TcpSocket {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'socket> AsSocket<'socket> for &'socket mio::net::TcpSocket {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'socket> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for mio::net::TcpSocket {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoSocket for mio::net::TcpSocket {
    #[inline]
    fn into_socket(self) -> OwnedSocket {
        unsafe { OwnedSocket::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for mio::net::TcpSocket {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromSocket for mio::net::TcpSocket {
    #[inline]
    fn from_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'fd> AsFd<'fd> for &'fd mio::net::UdpSocket {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'socket> AsSocket<'socket> for &'socket mio::net::UdpSocket {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'socket> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for mio::net::UdpSocket {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoSocket for mio::net::UdpSocket {
    #[inline]
    fn into_socket(self) -> OwnedSocket {
        unsafe { OwnedSocket::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for mio::net::UdpSocket {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromSocket for mio::net::UdpSocket {
    #[inline]
    fn from_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(unix)]
impl<'fd> AsFd<'fd> for &'fd mio::net::UnixDatagram {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl IntoFd for mio::net::UnixDatagram {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl FromFd for mio::net::UnixDatagram {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'fd> AsFd<'fd> for &'fd mio::net::UnixListener {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl IntoFd for mio::net::UnixListener {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl FromFd for mio::net::UnixListener {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'fd> AsFd<'fd> for &'fd mio::net::UnixStream {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl IntoFd for mio::net::UnixStream {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl FromFd for mio::net::UnixStream {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'fd> AsFd<'fd> for &'fd mio::unix::pipe::Receiver {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl IntoFd for mio::unix::pipe::Receiver {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl FromFd for mio::unix::pipe::Receiver {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'fd> AsFd<'fd> for &'fd mio::unix::pipe::Sender {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'fd> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl IntoFd for mio::unix::pipe::Sender {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl FromFd for mio::unix::pipe::Sender {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}
