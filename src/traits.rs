#[cfg(unix)]
use crate::{BorrowedFd, OptionFd, OwnedFd};
#[cfg(windows)]
use crate::{
    BorrowedHandle, BorrowedSocket, OptionFileHandle, OptionHandle, OptionSocket, OwnedHandle,
    OwnedSocket,
};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(windows)]
use std::os::windows::io::{
    AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
};

/// A trait to borrow the file descriptor from an underlying object.
#[cfg(unix)]
pub trait AsBorrowedFd {
    /// Extracts the file descriptor.
    fn as_borrowed_fd(&self) -> BorrowedFd<'_>;
}

/// A trait to borrow the handle from an underlying object.
#[cfg(windows)]
pub trait AsBorrowedHandle {
    /// Extracts the handle.
    fn as_borrowed_handle(&self) -> BorrowedHandle<'_>;
}

/// A trait to borrow the socket from an underlying object.
#[cfg(windows)]
pub trait AsBorrowedSocket {
    /// Extracts the socket.
    fn as_borrowed_socket(&self) -> BorrowedSocket<'_>;
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its file descriptor.
#[cfg(unix)]
pub trait IntoOwnedFd {
    /// Consumes this object, returning the underlying file descriptor.
    fn into_owned_fd(self) -> OwnedFd;
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its handle.
#[cfg(windows)]
pub trait IntoOwnedHandle {
    /// Consumes this object, returning the underlying handle.
    fn into_owned_handle(self) -> OwnedHandle;
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its socket.
#[cfg(windows)]
pub trait IntoOwnedSocket {
    /// Consumes this object, returning the underlying socket.
    fn into_owned_socket(self) -> OwnedSocket;
}

/// A trait to express the ability to construct an object from a file
/// descriptor.
#[cfg(unix)]
pub trait FromOwnedFd {
    /// Constructs a new instance of `Self` from the given file descriptor.
    fn from_owned_fd(owned: OwnedFd) -> Self;
}

/// A trait to express the ability to construct an object from a handle.
#[cfg(windows)]
pub trait FromOwnedHandle {
    /// Constructs a new instance of `Self` from the given handle.
    fn from_owned_handle(owned: OwnedHandle) -> Self;
}

/// A trait to express the ability to construct an object from a socket.
#[cfg(windows)]
pub trait FromOwnedSocket {
    /// Constructs a new instance of `Self` from the given socket.
    fn from_owned_socket(owned: OwnedSocket) -> Self;
}

#[cfg(unix)]
impl AsBorrowedFd for BorrowedFd<'_> {
    #[inline]
    fn as_borrowed_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsBorrowedHandle for BorrowedHandle<'_> {
    #[inline]
    fn as_borrowed_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl AsBorrowedSocket for BorrowedSocket<'_> {
    #[inline]
    fn as_borrowed_socket(&self) -> BorrowedSocket<'_> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(unix)]
impl AsBorrowedFd for OwnedFd {
    #[inline]
    fn as_borrowed_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsBorrowedHandle for OwnedHandle {
    #[inline]
    fn as_borrowed_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl AsBorrowedSocket for OwnedSocket {
    #[inline]
    fn as_borrowed_socket(&self) -> BorrowedSocket<'_> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(unix)]
impl IntoOwnedFd for OwnedFd {
    #[inline]
    fn into_owned_fd(self) -> OwnedFd {
        unsafe { Self::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoOwnedHandle for OwnedHandle {
    #[inline]
    fn into_owned_handle(self) -> OwnedHandle {
        unsafe { Self::from_raw_handle(self.into_raw_handle()) }
    }
}

#[cfg(windows)]
impl IntoOwnedSocket for OwnedSocket {
    #[inline]
    fn into_owned_socket(self) -> OwnedSocket {
        unsafe { Self::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(unix)]
impl FromOwnedFd for OwnedFd {
    #[inline]
    fn from_owned_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromOwnedHandle for OwnedHandle {
    #[inline]
    fn from_owned_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(windows)]
impl FromOwnedSocket for OwnedSocket {
    #[inline]
    fn from_owned_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(unix)]
impl FromOwnedFd for OptionFd {
    #[inline]
    fn from_owned_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromOwnedHandle for OptionHandle {
    #[inline]
    fn from_owned_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(windows)]
impl FromOwnedHandle for OptionFileHandle {
    #[inline]
    fn from_owned_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(windows)]
impl FromOwnedSocket for OptionSocket {
    #[inline]
    fn from_owned_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}

#[cfg(unix)]
impl AsBorrowedFd for std::fs::File {
    #[inline]
    fn as_borrowed_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsBorrowedHandle for std::fs::File {
    #[inline]
    fn as_borrowed_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(unix)]
impl IntoOwnedFd for std::fs::File {
    #[inline]
    fn into_owned_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoOwnedHandle for std::fs::File {
    #[inline]
    fn into_owned_handle(self) -> OwnedHandle {
        unsafe { OwnedHandle::from_raw_handle(self.into_raw_handle()) }
    }
}

#[cfg(unix)]
impl FromOwnedFd for std::fs::File {
    #[inline]
    fn from_owned_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromOwnedHandle for std::fs::File {
    #[inline]
    fn from_owned_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(unix)]
impl AsBorrowedFd for std::net::TcpStream {
    #[inline]
    fn as_borrowed_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsBorrowedSocket for std::net::TcpStream {
    #[inline]
    fn as_borrowed_socket(&self) -> BorrowedSocket<'_> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(unix)]
impl IntoOwnedFd for std::net::TcpStream {
    #[inline]
    fn into_owned_fd(self) -> OwnedFd {
        unsafe { OwnedFd::from_raw_fd(self.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl IntoOwnedSocket for std::net::TcpStream {
    #[inline]
    fn into_owned_socket(self) -> OwnedSocket {
        unsafe { OwnedSocket::from_raw_socket(self.into_raw_socket()) }
    }
}

#[cfg(unix)]
impl FromOwnedFd for std::net::TcpStream {
    #[inline]
    fn from_owned_fd(owned: OwnedFd) -> Self {
        unsafe { Self::from_raw_fd(owned.into_raw_fd()) }
    }
}

#[cfg(windows)]
impl FromOwnedSocket for std::net::TcpStream {
    #[inline]
    fn from_owned_socket(owned: OwnedSocket) -> Self {
        unsafe { Self::from_raw_socket(owned.into_raw_socket()) }
    }
}
