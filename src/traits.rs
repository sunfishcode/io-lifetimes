#[cfg(any(unix, target_os = "wasi"))]
use crate::{BorrowedFd, OwnedFd};
#[cfg(windows)]
use crate::{BorrowedHandle, BorrowedSocket, OptionFileHandle, OwnedHandle, OwnedSocket};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd};
#[cfg(windows)]
use std::os::windows::io::{
    AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
};

/// A trait to borrow the file descriptor from an underlying object.
///
/// This is only available on unix platforms and must be imported in order to
/// call the method. Windows platforms have a corresponding `AsHandle` and
/// `AsSocket` set of traits.
#[cfg(any(unix, target_os = "wasi"))]
pub trait AsFd {
    /// Borrows the file descriptor.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{AsFd, BorrowedFd};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// let borrowed_fd: BorrowedFd<'_> = f.as_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn as_fd(&self) -> BorrowedFd<'_>;
}

/// A trait to borrow the handle from an underlying object.
#[cfg(windows)]
pub trait AsHandle {
    /// Borrows the handle.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{AsHandle, BorrowedHandle};
    ///
    /// let mut f = File::open("foo.txt")?;
    /// let borrowed_handle: BorrowedHandle<'_> = f.as_handle();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn as_handle(&self) -> BorrowedHandle<'_>;
}

/// A trait to borrow the socket from an underlying object.
#[cfg(windows)]
pub trait AsSocket {
    /// Borrows the socket.
    fn as_socket(&self) -> BorrowedSocket<'_>;
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its file descriptor.
#[cfg(any(unix, target_os = "wasi"))]
pub trait IntoFd {
    /// Consumes this object, returning the underlying file descriptor.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{IntoFd, OwnedFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// let owned_fd: OwnedFd = f.into_fd();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn into_fd(self) -> OwnedFd;
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its handle.
#[cfg(windows)]
pub trait IntoHandle {
    /// Consumes this object, returning the underlying handle.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{IntoHandle, OwnedHandle};
    ///
    /// let f = File::open("foo.txt")?;
    /// let owned_handle: OwnedHandle = f.into_handle();
    /// # Ok::<(), io::Error>(())
    /// ```
    fn into_handle(self) -> OwnedHandle;
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its socket.
#[cfg(windows)]
pub trait IntoSocket {
    /// Consumes this object, returning the underlying socket.
    fn into_socket(self) -> OwnedSocket;
}

/// A trait to express the ability to construct an object from a file
/// descriptor.
#[cfg(any(unix, target_os = "wasi"))]
pub trait FromFd {
    /// Constructs a new instance of `Self` from the given file descriptor.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{FromFd, IntoFd, OwnedFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// let owned_fd: OwnedFd = f.into_fd();
    /// let f = File::from_fd(owned_fd);
    /// # Ok::<(), io::Error>(())
    /// ```
    fn from_fd(owned: OwnedFd) -> Self;

    /// Constructs a new instance of `Self` from the given file descriptor
    /// converted from `into_owned`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{FromFd, IntoFd};
    ///
    /// let f = File::open("foo.txt")?;
    /// let f = File::from_into_fd(f);
    /// # Ok::<(), io::Error>(())
    /// ```
    #[inline]
    fn from_into_fd<Owned: IntoFd>(into_owned: Owned) -> Self
    where
        Self: Sized,
    {
        Self::from_fd(into_owned.into_fd())
    }
}

/// A trait to express the ability to construct an object from a handle.
#[cfg(windows)]
pub trait FromHandle {
    /// Constructs a new instance of `Self` from the given handle.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{FromHandle, IntoHandle, OwnedHandle};
    ///
    /// let f = File::open("foo.txt")?;
    /// let owned_handle: OwnedHandle = f.into_handle();
    /// let f = File::from_handle(owned_handle);
    /// # Ok::<(), io::Error>(())
    /// ```
    fn from_handle(owned: OwnedHandle) -> Self;

    /// Constructs a new instance of `Self` from the given handle converted
    /// from `into_owned`.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::fs::File;
    /// # use std::io;
    /// use io_lifetimes::{FromHandle, IntoHandle};
    ///
    /// let f = File::open("foo.txt")?;
    /// let f = File::from_into_handle(f);
    /// # Ok::<(), io::Error>(())
    /// ```
    #[inline]
    fn from_into_handle<Owned: IntoHandle>(into_owned: Owned) -> Self
    where
        Self: Sized,
    {
        Self::from_handle(into_owned.into_handle())
    }
}

/// A trait to express the ability to construct an object from a socket.
#[cfg(windows)]
pub trait FromSocket {
    /// Constructs a new instance of `Self` from the given socket.
    fn from_socket(owned: OwnedSocket) -> Self;

    /// Constructs a new instance of `Self` from the given socket converted
    /// from `into_owned`.
    #[inline]
    fn from_into_socket<Owned: IntoSocket>(into_owned: Owned) -> Self
    where
        Self: Sized,
    {
        Self::from_socket(into_owned.into_socket())
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsFd for BorrowedFd<'_> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsHandle for BorrowedHandle<'_> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl AsSocket for BorrowedSocket<'_> {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
        unsafe { BorrowedSocket::borrow_raw_socket(self.as_raw_socket()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsFd for OwnedFd {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsHandle for OwnedHandle {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(windows)]
impl AsSocket for OwnedSocket {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
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
impl FromHandle for OptionFileHandle {
    #[inline]
    fn from_handle(owned: OwnedHandle) -> Self {
        unsafe { Self::from_raw_handle(owned.into_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsFd for std::fs::File {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsHandle for std::fs::File {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
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
impl AsFd for std::net::TcpStream {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsSocket for std::net::TcpStream {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
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
impl AsFd for std::net::TcpListener {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsSocket for std::net::TcpListener {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket<'_> {
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
impl AsFd for std::net::UdpSocket {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsSocket for std::net::UdpSocket {
    #[inline]
    fn as_socket(&self) -> BorrowedSocket {
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
impl AsFd for std::io::Stdin {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsHandle for std::io::Stdin {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd for std::io::StdinLock<'a> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle for std::io::StdinLock<'a> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsFd for std::io::Stdout {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsHandle for std::io::Stdout {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd for std::io::StdoutLock<'a> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle for std::io::StdoutLock<'a> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsFd for std::io::Stderr {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsHandle for std::io::Stderr {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl<'a> AsFd for std::io::StderrLock<'a> {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl<'a> AsHandle for std::io::StderrLock<'a> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        unsafe { BorrowedHandle::borrow_raw_handle(self.as_raw_handle()) }
    }
}

#[cfg(unix)]
impl AsFd for std::process::ChildStdin {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsHandle for std::process::ChildStdin {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
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
impl AsFd for std::process::ChildStdout {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsHandle for std::process::ChildStdout {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
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
impl AsFd for std::process::ChildStderr {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw_fd(self.as_raw_fd()) }
    }
}

#[cfg(windows)]
impl AsHandle for std::process::ChildStderr {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
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
impl AsFd for std::os::unix::net::UnixStream {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
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
impl AsFd for std::os::unix::net::UnixListener {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
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
impl AsFd for std::os::unix::net::UnixDatagram {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
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
impl<T> AsHandle for std::thread::JoinHandle<T> {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
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
