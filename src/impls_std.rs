#[cfg(any(unix, target_os = "wasi"))]
use crate::{AsFd, FromFd, IntoFd};
#[cfg(windows)]
use crate::{AsHandle, AsSocket, FromHandle, FromSocket, IntoHandle, IntoSocket};
#[cfg(any(unix, target_os = "wasi"))]
use crate::{BorrowedFd, OwnedFd};
#[cfg(windows)]
use crate::{BorrowedHandle, BorrowedSocket, HandleOrInvalid, OwnedHandle, OwnedSocket};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(windows)]
use std::os::windows::io::{
    AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
};

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for BorrowedFd<'a> {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for BorrowedHandle<'a> {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl<'a> AsSocket<'a> for BorrowedSocket<'a> {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'a> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a OwnedFd {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a OwnedHandle {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl<'a> AsSocket<'a> for &'a OwnedSocket {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'a> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for OwnedFd {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { Self::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoHandle for OwnedHandle {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        unsafe { Self::from_raw_handle(self.into_raw_handle()) }
    }
}

#[cfg(windows)]
impl IntoSocket for OwnedSocket {
    #[inline]
    fn into_socket(self) -> OwnedSocket {
        unsafe { Self::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for OwnedFd {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromHandle for OwnedHandle {
    #[inline]
    fn from_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(windows)]
impl FromSocket for OwnedSocket {
    #[inline]
    fn from_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(windows)]
impl FromHandle for HandleOrInvalid {
    #[inline]
    fn from_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a std::fs::File {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a std::fs::File {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for std::fs::File {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoHandle for std::fs::File {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(self.into_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for std::fs::File {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromHandle for std::fs::File {
    #[inline]
    fn from_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a std::net::TcpStream {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsSocket<'a> for &'a std::net::TcpStream {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'a> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for std::net::TcpStream {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoSocket for std::net::TcpStream {
    #[inline]
    fn into_socket(self) -> OwnedSocket {
        unsafe { OwnedSocket::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for std::net::TcpStream {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromSocket for std::net::TcpStream {
    #[inline]
    fn from_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a std::net::TcpListener {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsSocket<'a> for &'a std::net::TcpListener {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'a> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for std::net::TcpListener {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoSocket for std::net::TcpListener {
    #[inline]
    fn into_socket(self) -> OwnedSocket {
        unsafe { OwnedSocket::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for std::net::TcpListener {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromSocket for std::net::TcpListener {
    #[inline]
    fn from_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a std::net::UdpSocket {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsSocket<'a> for &'a std::net::UdpSocket {
    #[inline]
    fn as_socket(self) -> BorrowedSocket<'a> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoFd for std::net::UdpSocket {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoSocket for std::net::UdpSocket {
    #[inline]
    fn into_socket(self) -> OwnedSocket {
        unsafe { OwnedSocket::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromFd for std::net::UdpSocket {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromSocket for std::net::UdpSocket {
    #[inline]
    fn from_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a std::io::Stdin {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a std::io::Stdin {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a, 'b> AsFd<'a> for &'a std::io::StdinLock<'b> {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a, 'b> AsHandle<'a> for &'a std::io::StdinLock<'b> {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a std::io::Stdout {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a std::io::Stdout {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a, 'b> AsFd<'a> for &'a std::io::StdoutLock<'b> {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a, 'b> AsHandle<'a> for &'a std::io::StdoutLock<'b> {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd<'a> for &'a std::io::Stderr {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a std::io::Stderr {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a, 'b> AsFd<'a> for &'a std::io::StderrLock<'b> {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a, 'b> AsHandle<'a> for &'a std::io::StderrLock<'b> {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(unix)]
impl<'a> AsFd<'a> for &'a std::process::ChildStdin {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a std::process::ChildStdin {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(unix)]
impl IntoFd for std::process::ChildStdin {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoHandle for std::process::ChildStdin {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(self.into_raw_handle()) }
    }
}

#[cfg(unix)]
impl<'a> AsFd<'a> for &'a std::process::ChildStdout {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a std::process::ChildStdout {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(unix)]
impl IntoFd for std::process::ChildStdout {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoHandle for std::process::ChildStdout {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(self.into_raw_handle()) }
    }
}

#[cfg(unix)]
impl<'a> AsFd<'a> for &'a std::process::ChildStderr {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a std::process::ChildStderr {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(unix)]
impl IntoFd for std::process::ChildStderr {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoHandle for std::process::ChildStderr {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(self.into_raw_handle()) }
    }
}

#[cfg(unix)]
impl FromFd for std::process::Stdio {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromHandle for std::process::Stdio {
    #[inline]
    fn from_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle<'a> for &'a std::process::Child {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl IntoHandle for std::process::Child {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(self.into_raw_handle()) }
    }
}

#[cfg(unix)]
impl<'a> AsFd<'a> for &'a std::os::unix::net::UnixStream {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl IntoFd for std::os::unix::net::UnixStream {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl FromFd for std::os::unix::net::UnixStream {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'a> AsFd<'a> for &'a std::os::unix::net::UnixListener {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl IntoFd for std::os::unix::net::UnixListener {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl FromFd for std::os::unix::net::UnixListener {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl<'a> AsFd<'a> for &'a std::os::unix::net::UnixDatagram {
    #[inline]
    fn as_fd(self) -> BorrowedFd<'a> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(unix)]
impl IntoFd for std::os::unix::net::UnixDatagram {
    #[inline]
    fn into_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(unix)]
impl FromFd for std::os::unix::net::UnixDatagram {
    #[inline]
    fn from_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a, T> AsHandle<'a> for &'a std::thread::JoinHandle<T> {
    #[inline]
    fn as_handle(self) -> BorrowedHandle<'a> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl<T> IntoHandle for std::thread::JoinHandle<T> {
    #[inline]
    fn into_handle(self) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(self.into_raw_handle()) }
    }
}
