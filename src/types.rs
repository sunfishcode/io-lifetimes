use std::fmt;
use std::marker::PhantomData;
use std::mem::forget;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::{
    convert::TryFrom,
    os::windows::io::{
        AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
        RawHandle, RawSocket,
    },
};
#[cfg(all(windows, feature = "close"))]
use winapi::{um::handleapi::INVALID_HANDLE_VALUE, um::winsock2::INVALID_SOCKET};

#[cfg(all(windows, not(feature = "winapi")))]
const INVALID_HANDLE_VALUE: *mut core::ffi::c_void = !0 as _;
#[cfg(all(windows, not(feature = "winapi")))]
const INVALID_SOCKET: usize = !0 as _;

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
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_start(0))]
// libstd/os/raw/mod.rs assures me that every libstd-supported platform has a
// 32-bit c_int. Below is -2, in two's complement, but that only works out
// because c_int is 32 bits.
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE))]
pub struct BorrowedFd<'fd> {
    fd: RawFd,
    _phantom: PhantomData<&'fd OwnedFd>,
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
pub struct BorrowedHandle<'handle> {
    handle: RawHandle,
    _phantom: PhantomData<&'handle OwnedHandle>,
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
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_start(0))]
// This is -2, in two's complement. -1 is `INVALID_SOCKET`.
#[cfg_attr(
    all(rustc_attrs, target_pointer_width = "32"),
    rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE)
)]
#[cfg_attr(
    all(rustc_attrs, target_pointer_width = "64"),
    rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FF_FF_FF_FF_FE)
)]
pub struct BorrowedSocket<'socket> {
    socket: RawSocket,
    _phantom: PhantomData<&'socket OwnedSocket>,
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
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_start(0))]
// libstd/os/raw/mod.rs assures me that every libstd-supported platform has a
// 32-bit c_int. Below is -2, in two's complement, but that only works out
// because c_int is 32 bits.
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE))]
pub struct OwnedFd {
    fd: RawFd,
}

/// An owned handle.
///
/// This closes the handle on drop.
///
/// Note that it *may* have the value `INVALID_HANDLE_VALUE` (-1), which is
/// sometimes a valid handle value. See [here] for the full story.
///
/// And, it *may* have the value `NULL` (0), which can occur when consoles are
/// detached from processes, or when `windows_subsystem` is used.
///
/// `OwnedHandle` uses [`CloseHandle`] to close its handle on drop. As such,
/// it must not be used with handles to open registry keys which need to be
/// closed with [`RegCloseKey`] instead.
///
/// [`CloseHandle`]: https://docs.microsoft.com/en-us/windows/win32/api/handleapi/nf-handleapi-closehandle
/// [`RegCloseKey`]: https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey
///
/// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
#[cfg(windows)]
pub struct OwnedHandle {
    handle: RawHandle,
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
#[cfg_attr(rustc_attrs, rustc_layout_scalar_valid_range_start(0))]
// This is -2, in two's complement. -1 is `INVALID_SOCKET`.
#[cfg_attr(
    all(rustc_attrs, target_pointer_width = "32"),
    rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE)
)]
#[cfg_attr(
    all(rustc_attrs, target_pointer_width = "64"),
    rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FF_FF_FF_FF_FE)
)]
pub struct OwnedSocket {
    socket: RawSocket,
}

/// FFI type for handles in return values or out parameters, where `INVALID_HANDLE_VALUE` is used
/// as a sentry value to indicate errors, such as in the return value of `CreateFileW`. This uses
/// `repr(transparent)` and has the representation of a host handle, so that it can be used in such
/// FFI declarations.
///
/// The only thing you can usefully do with a `HandleOrInvalid` is to convert it into an
/// `OwnedHandle` using its [`TryFrom`] implementation; this conversion takes care of the check for
/// `INVALID_HANDLE_VALUE`. This ensures that such FFI calls cannot start using the handle without
/// checking for `INVALID_HANDLE_VALUE` first.
///
/// This type concerns any value other than `INVALID_HANDLE_VALUE` to be valid, including `NULL`.
/// This is because APIs that use `INVALID_HANDLE_VALUE` as their sentry value may return `NULL`
/// under `windows_subsystem = "windows"` or other situations where I/O devices are detached.
///
/// If this holds a valid handle, it will close the handle on drop.
#[cfg(windows)]
#[repr(transparent)]
#[derive(Debug)]
pub struct HandleOrInvalid(RawHandle);

/// FFI type for handles in return values or out parameters, where `NULL` is used
/// as a sentry value to indicate errors, such as in the return value of `CreateThread`. This uses
/// `repr(transparent)` and has the representation of a host handle, so that it can be used in such
/// FFI declarations.
///
/// The only thing you can usefully do with a `HandleOrNull` is to convert it into an
/// `OwnedHandle` using its [`TryFrom`] implementation; this conversion takes care of the check for
/// `NULL`. This ensures that such FFI calls cannot start using the handle without
/// checking for `NULL` first.
///
/// This type concerns any value other than `NULL` to be valid, including `INVALID_HANDLE_VALUE`.
/// This is because APIs that use `NULL` as their sentry value don't treat `INVALID_HANDLE_VALUE`
/// as special.
///
/// If this holds a valid handle, it will close the handle on drop.
#[cfg(windows)]
#[repr(transparent)]
#[derive(Debug)]
pub struct HandleOrNull(RawHandle);

// The Windows [`HANDLE`] type may be transferred across and shared between
// thread boundaries (despite containing a `*mut void`, which in general isn't
// `Send` or `Sync`).
//
// [`HANDLE`]: std::os::windows::raw::HANDLE
#[cfg(windows)]
unsafe impl Send for OwnedHandle {}
#[cfg(windows)]
unsafe impl Send for HandleOrInvalid {}
#[cfg(windows)]
unsafe impl Send for HandleOrNull {}
#[cfg(windows)]
unsafe impl Send for BorrowedHandle<'_> {}
#[cfg(windows)]
unsafe impl Sync for OwnedHandle {}
#[cfg(windows)]
unsafe impl Sync for HandleOrInvalid {}
#[cfg(windows)]
unsafe impl Sync for HandleOrNull {}
#[cfg(windows)]
unsafe impl Sync for BorrowedHandle<'_> {}

#[cfg(any(unix, target_os = "wasi"))]
impl BorrowedFd<'_> {
    /// Return a `BorrowedFd` holding the given raw file descriptor.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `raw` must remain open for the duration of
    /// the returned `BorrowedFd`, and it must not have the value `-1`.
    #[inline]
    pub unsafe fn borrow_raw_fd(fd: RawFd) -> Self {
        debug_assert_ne!(fd, -1_i32 as RawFd);
        Self {
            fd,
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
    /// the returned `BorrowedHandle`, and it must not be null.
    #[inline]
    pub unsafe fn borrow_raw_handle(handle: RawHandle) -> Self {
        debug_assert!(!handle.is_null());
        Self {
            handle,
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
    /// the returned `BorrowedSocket`, and it must not have the value
    /// [`INVALID_SOCKET`].
    #[inline]
    pub unsafe fn borrow_raw_socket(socket: RawSocket) -> Self {
        debug_assert_ne!(socket, INVALID_SOCKET as RawSocket);
        Self {
            socket,
            _phantom: PhantomData,
        }
    }
}

#[cfg(windows)]
impl TryFrom<HandleOrInvalid> for OwnedHandle {
    type Error = ();

    #[inline]
    fn try_from(handle_or_invalid: HandleOrInvalid) -> Result<Self, ()> {
        let raw = handle_or_invalid.0;
        if raw == INVALID_HANDLE_VALUE {
            Err(())
        } else {
            Ok(OwnedHandle { handle: raw })
        }
    }
}

#[cfg(windows)]
impl TryFrom<HandleOrNull> for OwnedHandle {
    type Error = ();

    #[inline]
    fn try_from(handle_or_null: HandleOrNull) -> Result<Self, ()> {
        let raw = handle_or_null.0;
        if raw.is_null() {
            Err(())
        } else {
            Ok(OwnedHandle { handle: raw })
        }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsRawFd for BorrowedFd<'_> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[cfg(windows)]
impl AsRawHandle for BorrowedHandle<'_> {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.handle
    }
}

#[cfg(windows)]
impl AsRawSocket for BorrowedSocket<'_> {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.socket
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl AsRawFd for OwnedFd {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}

#[cfg(windows)]
impl AsRawHandle for OwnedHandle {
    #[inline]
    fn as_raw_handle(&self) -> RawHandle {
        self.handle
    }
}

#[cfg(windows)]
impl AsRawSocket for OwnedSocket {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.socket
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl IntoRawFd for OwnedFd {
    #[inline]
    fn into_raw_fd(self) -> RawFd {
        let fd = self.fd;
        forget(self);
        fd
    }
}

#[cfg(windows)]
impl IntoRawHandle for OwnedHandle {
    #[inline]
    fn into_raw_handle(self) -> RawHandle {
        let handle = self.handle;
        forget(self);
        handle
    }
}

#[cfg(windows)]
impl IntoRawSocket for OwnedSocket {
    #[inline]
    fn into_raw_socket(self) -> RawSocket {
        let socket = self.socket;
        forget(self);
        socket
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
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        debug_assert_ne!(fd, -1_i32 as RawFd);
        Self { fd }
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
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self { handle }
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
    unsafe fn from_raw_socket(socket: RawSocket) -> Self {
        debug_assert_ne!(socket, INVALID_SOCKET as RawSocket);
        Self { socket }
    }
}

#[cfg(windows)]
impl FromRawHandle for HandleOrInvalid {
    /// Constructs a new instance of `Self` from the given `RawHandle` returned
    /// from a Windows API that uses `INVALID_HANDLE_VALUE` to indicate
    /// failure, such as `CreateFileW`.
    ///
    /// Use `HandleOrNull` instead of `HandleOrInvalid` for APIs that
    /// use null to indicate failure.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `handle` must be either open and otherwise
    /// unowned, null, or equal to `INVALID_HANDLE_VALUE` (-1). Note that not
    /// all Windows APIs use `INVALID_HANDLE_VALUE` for errors; see [here] for
    /// the full story.
    ///
    /// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self(handle)
    }
}

#[cfg(windows)]
impl FromRawHandle for HandleOrNull {
    /// Constructs a new instance of `Self` from the given `RawHandle` returned
    /// from a Windows API that uses null to indicate failure, such as
    /// `CreateThread`.
    ///
    /// Use `HandleOrInvalid` instead of `HandleOrNull` for APIs that
    /// use `INVALID_HANDLE_VALUE` to indicate failure.
    ///
    /// # Safety
    ///
    /// The resource pointed to by `handle` must be either open and otherwise
    /// unowned, or null. Note that not all Windows APIs use null for errors;
    /// see [here] for the full story.
    ///
    /// [here]: https://devblogs.microsoft.com/oldnewthing/20040302-00/?p=40443
    #[inline]
    unsafe fn from_raw_handle(handle: RawHandle) -> Self {
        Self(handle)
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl Drop for OwnedFd {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "close")]
        unsafe {
            let _ = libc::close(self.fd as std::os::raw::c_int);
        }

        // If the `close` feature is disabled, we expect users to avoid letting
        // `OwnedFd` instances drop, so that we don't have to call `close`.
        #[cfg(not(feature = "close"))]
        {
            unreachable!("drop called without the \"close\" feature in io-lifetimes");
        }
    }
}

#[cfg(windows)]
impl Drop for OwnedHandle {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "close")]
        unsafe {
            let _ = winapi::um::handleapi::CloseHandle(self.handle);
        }

        // If the `close` feature is disabled, we expect users to avoid letting
        // `OwnedHandle` instances drop, so that we don't have to call `close`.
        #[cfg(not(feature = "close"))]
        {
            unreachable!("drop called without the \"close\" feature in io-lifetimes");
        }
    }
}

#[cfg(windows)]
impl Drop for HandleOrInvalid {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "close")]
        unsafe {
            let _ = winapi::um::handleapi::CloseHandle(self.0);
        }

        // If the `close` feature is disabled, we expect users to avoid letting
        // `HandleOrInvalid` instances drop, so that we don't have to call `close`.
        #[cfg(not(feature = "close"))]
        {
            unreachable!("drop called without the \"close\" feature in io-lifetimes");
        }
    }
}

#[cfg(windows)]
impl Drop for HandleOrNull {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "close")]
        unsafe {
            let _ = winapi::um::handleapi::CloseHandle(self.0);
        }

        // If the `close` feature is disabled, we expect users to avoid letting
        // `HandleOrNull` instances drop, so that we don't have to call `close`.
        #[cfg(not(feature = "close"))]
        {
            unreachable!("drop called without the \"close\" feature in io-lifetimes");
        }
    }
}

#[cfg(windows)]
impl Drop for OwnedSocket {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "close")]
        unsafe {
            let _ = winapi::um::winsock2::closesocket(self.socket as winapi::um::winsock2::SOCKET);
        }

        // If the `close` feature is disabled, we expect users to avoid letting
        // `OwnedSocket` instances drop, so that we don't have to call `close`.
        #[cfg(not(feature = "close"))]
        {
            unreachable!("drop called without the \"close\" feature in io-lifetimes");
        }
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl fmt::Debug for BorrowedFd<'_> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BorrowedFd").field("fd", &self.fd).finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for BorrowedHandle<'_> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BorrowedHandle")
            .field("handle", &self.handle)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for BorrowedSocket<'_> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BorrowedSocket")
            .field("socket", &self.socket)
            .finish()
    }
}

#[cfg(any(unix, target_os = "wasi"))]
impl fmt::Debug for OwnedFd {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedFd").field("fd", &self.fd).finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for OwnedHandle {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedHandle")
            .field("handle", &self.handle)
            .finish()
    }
}

#[cfg(windows)]
impl fmt::Debug for OwnedSocket {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OwnedSocket")
            .field("socket", &self.socket)
            .finish()
    }
}
