use std::convert::TryFrom;
use std::marker::PhantomData;
use std::mem;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{
    AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
    RawHandle, RawSocket,
};

/// A borrowed file descriptor.
///
/// This has a lifetime parameter to tie it to the lifetime of something that
/// owns the file descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as an argument and is not captured or consumed.
#[cfg(unix)]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BorrowedFd<'owned> {
    raw: RawFd,
    _phantom: PhantomData<&'owned ()>,
}

/// A borrowed handle.
///
/// This has a lifetime parameter to tie it to the lifetime of something that
/// owns the handle.
///
/// This uses `repr(transparent)` and has the representation of a host handle,
/// so it can be used in FFI in places where a handle is passed as an argument
/// and is not captured or consumed.
#[cfg(windows)]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BorrowedHandle<'owned> {
    raw: RawHandle,
    _phantom: PhantomData<&'owned ()>,
}

/// A borrowed socket.
///
/// This has a lifetime parameter to tie it to the lifetime of something that
/// owns the socket.
///
/// This uses `repr(transparent)` and has the representation of a host socket,
/// so it can be used in FFI in places where a socket is passed as an argument
/// and is not captured or consumed.
#[cfg(windows)]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BorrowedSocket<'owned> {
    raw: RawSocket,
    _phantom: PhantomData<&'owned ()>,
}

/// An owned file descriptor.
///
/// This closes the file descriptor on drop.
//
// TODO: This doesn't use `repr(transparent)` because it's intended to use
// `rustc_layout_scalar_valid_range_*` optimizations in `std`.
#[cfg(unix)]
pub struct OwnedFd {
    raw: RawFd,
}

/// An owned handle.
///
/// This closes the handle on drop.
#[cfg(windows)]
pub struct OwnedHandle {
    raw: RawHandle,
}

/// An owned socket.
///
/// This closes the socket on drop.
#[cfg(windows)]
pub struct OwnedSocket {
    raw: RawSocket,
}

/// Either an owned file descriptor, or an empty sentry value which typically
/// indicates an error.
///
/// If this holds an owned file descriptor, it closes the file descriptor on
/// drop.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// returned and which may fail.
#[cfg(unix)]
#[repr(transparent)]
pub struct OptionFd {
    raw: libc::c_int,
}

/// Either an owned handle, or an empty sentry value which typically indicates
/// an error.
///
/// If this holds an owned handle, it closes the handle on drop.
///
/// This uses `repr(transparent)` and has the representation of a host handle,
/// so it can be used in FFI in places where a handle is returned and which may
/// fail.
#[cfg(windows)]
#[repr(transparent)]
pub struct OptionHandle {
    raw: winapi::um::winnt::HANDLE,
}

/// Either an owned socket, or an empty sentry value which typically indicates
/// an error.
///
/// If this holds an owned socket, it closes the socket on drop.
///
/// This uses `repr(transparent)` and has the representation of a host socket,
/// so it can be used in FFI in places where a socket is returned and which may
/// fail.
#[cfg(windows)]
#[repr(transparent)]
pub struct OptionSocket {
    raw: winapi::um::winsock2::SOCKET,
}

#[cfg(unix)]
impl<'owned> BorrowedFd<'owned> {
    /// # Safety
    ///
    /// The resource pointed to by `raw` must remain open for the duration of
    /// the returned `BorrowedFd`.
    #[inline]
    pub unsafe fn borrow_raw_fd(raw: RawFd) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }
}

#[cfg(windows)]
impl<'owned> BorrowedHandle<'owned> {
    /// # Safety
    ///
    /// The resource pointed to by `raw` must remain open for the duration of
    /// the returned `BorrowedFd`.
    #[inline]
    pub unsafe fn borrow_raw_handle(raw: RawHandle) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }
}

#[cfg(windows)]
impl<'owned> BorrowedSocket<'owned> {
    /// # Safety
    ///
    /// The resource pointed to by `raw` must remain open for the duration of
    /// the returned `BorrowedFd`.
    #[inline]
    pub unsafe fn borrow_raw_socket(raw: RawSocket) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }
}

#[cfg(unix)]
impl OptionFd {
    /// Return an empty `OptionFd` with no resource.
    #[inline]
    pub const fn none() -> Self {
        Self { raw: -1 }
    }
}

#[cfg(windows)]
impl OptionHandle {
    /// Return an empty `OptionHandle` with no resource.
    #[inline]
    pub const fn none() -> Self {
        Self {
            raw: winapi::um::handleapi::INVALID_HANDLE_VALUE,
        }
    }
}

#[cfg(windows)]
impl OptionSocket {
    #[inline]
    /// Return an empty `OptionSocket` with no resource.
    pub const fn none() -> Self {
        Self {
            raw: winapi::um::winsock2::INVALID_SOCKET,
        }
    }
}

#[cfg(unix)]
impl TryFrom<OptionFd> for OwnedFd {
    type Error = ();

    #[inline]
    fn try_from(option: OptionFd) -> Result<Self, ()> {
        let raw = option.raw;
        mem::forget(option);
        if raw != -1 {
            Ok(Self { raw })
        } else {
            Err(())
        }
    }
}

#[cfg(windows)]
impl TryFrom<OptionHandle> for OwnedHandle {
    type Error = ();

    #[inline]
    fn try_from(option: OptionHandle) -> Result<Self, ()> {
        let raw = option.raw;
        mem::forget(option);
        if raw != winapi::um::handleapi::INVALID_HANDLE_VALUE {
            Ok(Self { raw })
        } else {
            Err(())
        }
    }
}

#[cfg(windows)]
impl TryFrom<OptionSocket> for OwnedSocket {
    type Error = ();

    #[inline]
    fn try_from(option: OptionSocket) -> Result<Self, ()> {
        let raw = option.raw;
        mem::forget(option);
        if raw != winapi::um::winsock2::INVALID_SOCKET {
            Ok(Self {
                raw: raw as RawSocket,
            })
        } else {
            Err(())
        }
    }
}

#[cfg(unix)]
impl From<OwnedFd> for OptionFd {
    #[inline]
    fn from(owned: OwnedFd) -> Self {
        let raw = owned.raw;
        mem::forget(owned);
        Self { raw }
    }
}

#[cfg(windows)]
impl From<OwnedHandle> for OptionHandle {
    #[inline]
    fn from(owned: OwnedHandle) -> Self {
        let raw = owned.raw;
        mem::forget(owned);
        Self { raw }
    }
}

#[cfg(windows)]
impl From<OwnedSocket> for OptionSocket {
    #[inline]
    fn from(owned: OwnedSocket) -> Self {
        let raw = owned.raw;
        mem::forget(owned);
        Self {
            raw: raw as winapi::um::winsock2::SOCKET,
        }
    }
}

#[cfg(unix)]
impl<'owned> AsRawFd for BorrowedFd<'owned> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.raw
    }
}

#[cfg(windows)]
impl<'owned> AsRawHandle for BorrowedHandle<'owned> {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.raw
    }
}

#[cfg(windows)]
impl<'owned> AsRawSocket for BorrowedSocket<'owned> {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.raw
    }
}

#[cfg(unix)]
impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.raw
    }
}

#[cfg(windows)]
impl AsRawHandle for OwnedHandle {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.raw
    }
}

#[cfg(windows)]
impl AsRawSocket for OwnedSocket {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.raw
    }
}

#[cfg(unix)]
impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        let raw = self.raw;
        mem::forget(self);
        raw
    }
}

#[cfg(windows)]
impl IntoRawHandle for OwnedHandle {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        let raw = self.raw;
        mem::forget(self);
        raw
    }
}

#[cfg(windows)]
impl IntoRawSocket for OwnedSocket {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        let raw = self.raw;
        mem::forget(self);
        raw
    }
}

#[cfg(unix)]
impl FromRawFd for OwnedFd {
    #[inline]
    unsafe fn from_raw_fd(raw: RawFd) -> Self {
        Self { raw }
    }
}

#[cfg(windows)]
impl FromRawHandle for OwnedHandle {
    #[inline]
    unsafe fn from_raw_handle(raw: RawHandle) -> Self {
        Self { raw }
    }
}

#[cfg(windows)]
impl FromRawSocket for OwnedSocket {
    #[inline]
    unsafe fn from_raw_socket(raw: RawSocket) -> Self {
        Self { raw }
    }
}

#[cfg(unix)]
impl FromRawFd for OptionFd {
    #[inline]
    unsafe fn from_raw_fd(raw: RawFd) -> Self {
        Self { raw }
    }
}

#[cfg(windows)]
impl FromRawHandle for OptionHandle {
    #[inline]
    unsafe fn from_raw_handle(raw: RawHandle) -> Self {
        Self { raw }
    }
}

#[cfg(windows)]
impl FromRawSocket for OptionSocket {
    #[inline]
    unsafe fn from_raw_socket(raw: RawSocket) -> Self {
        Self {
            raw: raw as winapi::um::winsock2::SOCKET,
        }
    }
}

#[cfg(unix)]
impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = libc::close(self.raw);
        }
    }
}

#[cfg(windows)]
impl Drop for OwnedHandle {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = winapi::um::handleapi::CloseHandle(self.raw);
        }
    }
}

#[cfg(windows)]
impl Drop for OwnedSocket {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = winapi::um::winsock2::closesocket(self.raw as winapi::um::winsock2::SOCKET);
        }
    }
}

#[cfg(unix)]
impl Drop for OptionFd {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = libc::close(self.raw);
        }
    }
}

#[cfg(windows)]
impl Drop for OptionHandle {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = winapi::um::handleapi::CloseHandle(self.raw);
        }
    }
}

#[cfg(windows)]
impl Drop for OptionSocket {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = winapi::um::winsock2::closesocket(self.raw);
        }
    }
}
