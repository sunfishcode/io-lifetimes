use std::marker::PhantomData;
use std::mem::forget;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::{
    convert::TryFrom,
    ffi::c_void,
    os::windows::io::{
        AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
        RawHandle, RawSocket,
    },
    ptr::NonNull,
};
#[cfg(windows)]
use winapi::{um::handleapi::INVALID_HANDLE_VALUE, um::winsock2::INVALID_SOCKET};

/// A borrowed file descriptor.
///
/// This has a lifetime parameter to tie it to the lifetime of something that
/// owns the file descriptor.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as an argument, it is not captured or consumed, and it never has the
/// value `-1`.
#[cfg(any(unix, target_os = "wasi"))]
#[derive(Copy, Clone)]
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0)]
// libstd/os/raw/mod.rs assures me that every libstd-supported platform has a
// 32-bit c_int. Below is -2, in two's complement, but that only works out
// because c_int is 32 bits.
#[rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE)]
pub struct BorrowedFd<'owned> {
    raw: RawFd,
    _phantom: PhantomData<&'owned OwnedFd>,
}

/// A borrowed handle.
///
/// This has a lifetime parameter to tie it to the lifetime of something that
/// owns the handle.
///
/// This uses `repr(transparent)` and has the representation of a host handle,
/// so it can be used in FFI in places where a handle is passed as an argument,
/// it is not captured or consumed, and it is never null.
///
/// Note that it *may* have the value [`INVALID_HANDLE_VALUE`]. See [here] for
/// the full story.
///
/// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
#[cfg(windows)]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BorrowedHandle<'owned> {
    raw: NonNull<c_void>,
    _phantom: PhantomData<&'owned OwnedHandle>,
}

/// A borrowed socket.
///
/// This has a lifetime parameter to tie it to the lifetime of something that
/// owns the socket.
///
/// This uses `repr(transparent)` and has the representation of a host socket,
/// so it can be used in FFI in places where a socket is passed as an argument,
/// it is not captured or consumed, and it never has the value
/// [`INVALID_SOCKET`].
#[cfg(windows)]
#[derive(Copy, Clone)]
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0)]
// This is -2, in two's complement. -1 is `INVALID_SOCKET`.
#[cfg_attr(
    target_pointer_width = "32",
    rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE)
)]
#[cfg_attr(
    target_pointer_width = "64",
    rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FF_FF_FF_FF_FE)
)]
pub struct BorrowedSocket<'owned> {
    raw: RawSocket,
    _phantom: PhantomData<&'owned OwnedSocket>,
}

/// An owned file descriptor.
///
/// This closes the file descriptor on drop.
///
/// This uses `repr(transparent)` and has the representation of a host file
/// descriptor, so it can be used in FFI in places where a file descriptor is
/// passed as a consumed argument or returned as an owned value, and it never
/// has the value `-1`.
#[cfg(any(unix, target_os = "wasi"))]
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0)]
// libstd/os/raw/mod.rs assures me that every libstd-supported platform has a
// 32-bit c_int. Below is -2, in two's complement, but that only works out
// because c_int is 32 bits.
#[rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE)]
pub struct OwnedFd {
    raw: RawFd,
}

/// An owned handle.
///
/// This closes the handle on drop.
///
/// This uses `repr(transparent)` and has the representation of a host handle,
/// so it can be used in FFI in places where a handle is passed as a consumed
/// argument or returned as an owned value, and is never null.
///
/// Note that it *may* have the value [`INVALID_HANDLE_VALUE`]. See [here] for
/// the full story. For APIs like `CreateFileW` which report errors with
/// `INVALID_HANDLE_VALUE` instead of null, use [`OptionFileHandle`] instead
/// of `Option<OwnedHandle>`.
///
/// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
#[cfg(windows)]
#[repr(transparent)]
pub struct OwnedHandle {
    raw: NonNull<c_void>,
}

/// An owned socket.
///
/// This closes the socket on drop.
///
/// This uses `repr(transparent)` and has the representation of a host socket,
/// so it can be used in FFI in places where a socket is passed as a consumed
/// argument or returned as an owned value, and it never has the value
/// [`INVALID_SOCKET`].
#[cfg(windows)]
#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0)]
// This is -2, in two's complement. -1 is `INVALID_SOCKET`.
#[cfg_attr(
    target_pointer_width = "32",
    rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE)
)]
#[cfg_attr(
    target_pointer_width = "64",
    rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FF_FF_FF_FF_FE)
)]
pub struct OwnedSocket {
    raw: RawSocket,
}

/// Similar to `Option<OwnedHandle>`, but intended for use in FFI interfaces
/// where [`INVALID_HANDLE_VALUE`] is used as the sentry value.
///
/// If this holds an owned handle, it closes the handle on drop.
///
/// This uses `repr(transparent)` and has the representation of a host handle,
/// so it can be used in FFI in places where a handle is passed as a consumed
/// argument or returned as an owned value, or it is [`INVALID_HANDLE_VALUE`]
/// indicating an error or an otherwise absent value.
#[cfg(windows)]
#[repr(transparent)]
pub struct OptionFileHandle {
    // TODO: There is reason to guess that functions like `CreateFile`
    // never return NULL, even on success, however I haven't yet found
    // official documentation mentioning this. If it turns out that NULL
    // can be a valid value, we'll need to redesign how
    // `Option<OwnedHandle>` works.
    raw: NonNull<c_void>,
}

#[cfg(any(unix, target_os = "wasi"))]
impl BorrowedFd<'_> {
    /// Return a `BorrowedFd` holding the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not have the value `-1`.
    #[inline]
    pub unsafe fn borrow_raw_fd(raw: RawFd) -> Self {
        debug_assert_ne!(raw, -1_i32 as RawFd);
        Self {
            raw,
            _phantom: PhantomData,
        }
    }
}

#[cfg(windows)]
impl BorrowedHandle<'_> {
    /// Return a `BorrowedHandle` holding the given raw handle.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not be null.
    #[inline]
    pub unsafe fn borrow_raw_handle(raw: RawHandle) -> Self {
        debug_assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
            _phantom: PhantomData,
        }
    }
}

#[cfg(windows)]
impl BorrowedSocket<'_> {
    /// Return a `BorrowedSocket` holding the given raw socket.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not have the value
    /// [`INVALID_SOCKET`].
    #[inline]
    pub unsafe fn borrow_raw_socket(raw: RawSocket) -> Self {
        debug_assert_ne!(raw, INVALID_SOCKET as RawSocket);
        Self {
            raw,
            _phantom: PhantomData,
        }
    }
}

#[cfg(windows)]
impl OptionFileHandle {
    /// Return an empty `OptionFileHandle` with no resource.
    #[inline]
    pub const fn none() -> Self {
        let non_null = unsafe { NonNull::new_unchecked(INVALID_HANDLE_VALUE) };
        Self { raw: non_null }
    }
}

#[cfg(windows)]
impl TryFrom<OptionFileHandle> for OwnedHandle {
    type Error = ();

    #[inline]
    fn try_from(option: OptionFileHandle) -> Result<Self, ()> {
        let raw = option.raw;
        forget(option);
        if raw.as_ptr() != INVALID_HANDLE_VALUE {
            Ok(Self { raw })
        } else {
            Err(())
        }
    }
}

#[cfg(windows)]
impl From<OwnedHandle> for OptionFileHandle {
    #[inline]
    fn from(owned: OwnedHandle) -> Self {
        let raw = owned.raw;
        forget(owned);
        Self { raw }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsRawFd for BorrowedFd<'_> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.raw
    }
}

#[cfg(windows)]
impl AsRawHandle for BorrowedHandle<'_> {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.raw.as_ptr()
    }
}

#[cfg(windows)]
impl AsRawSocket for BorrowedSocket<'_> {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.raw
    }
}

#[cfg(any(unix, target_os = "wasi"))]
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
        self.raw.as_ptr()
    }
}

#[cfg(windows)]
impl AsRawSocket for OwnedSocket {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.raw
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        let raw = self.raw;
        forget(self);
        raw
    }
}

#[cfg(windows)]
impl IntoRawHandle for OwnedHandle {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        let raw = self.raw.as_ptr();
        forget(self);
        raw
    }
}

#[cfg(windows)]
impl IntoRawSocket for OwnedSocket {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        let raw = self.raw;
        forget(self);
        raw
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl FromRawFd for OwnedFd {
    /// Constructs a new instance of `Self` from the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must be open and suitable for assuming
    /// ownership.
    #[inline]
    unsafe fn from_raw_fd(raw: RawFd) -> Self {
        debug_assert_ne!(raw, -1_i32 as RawFd);
        Self { raw }
    }
}

#[cfg(windows)]
impl FromRawHandle for OwnedHandle {
    /// Constructs a new instance of `Self` from the given raw handle.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must be open and suitable for assuming
    /// ownership.
    #[inline]
    unsafe fn from_raw_handle(raw: RawHandle) -> Self {
        debug_assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

#[cfg(windows)]
impl FromRawSocket for OwnedSocket {
    /// Constructs a new instance of `Self` from the given raw socket.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must be open and suitable for assuming
    /// ownership.
    #[inline]
    unsafe fn from_raw_socket(raw: RawSocket) -> Self {
        debug_assert_ne!(raw, INVALID_SOCKET as RawSocket);
        Self { raw }
    }
}

#[cfg(windows)]
impl FromRawHandle for OptionFileHandle {
    /// Constructs a new instance of `Self` from the given raw handle.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must be either open and otherwise
    /// unowned, or equal to [`INVALID_FILE_HANDLE]`. Note that not all Windows
    /// APIs use [`INVALID_HANDLE_VALUE`] for errors; see [here] for the full
    /// story.
    ///
    /// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
    #[inline]
    unsafe fn from_raw_handle(raw: RawHandle) -> Self {
        debug_assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = libc::close(self.raw as libc::c_int);
        }
    }
}

#[cfg(windows)]
impl Drop for OwnedHandle {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = winapi::um::handleapi::CloseHandle(self.raw.as_ptr());
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

#[cfg(windows)]
impl Drop for OptionFileHandle {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _ = winapi::um::handleapi::CloseHandle(self.raw.as_ptr());
        }
    }
}
