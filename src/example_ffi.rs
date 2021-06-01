//! This is just a sample of what FFI using this crate can look like.

// Disable `improper_ctypes` warnings so that we don't lint about
// `Option<OwnedFd>` appearing in an FFI signature. In the future rustc
// would be modified to recognize this case.
#![allow(improper_ctypes)]
#![allow(missing_docs)]

#[cfg(any(unix, target_os = "wasi"))]
use crate::{BorrowedFd, OwnedFd};
#[cfg(windows)]
use crate::{BorrowedHandle, OptionFileHandle, OwnedHandle};

#[cfg(any(unix, target_os = "wasi"))]
use libc::{c_char, c_int, c_void, size_t, ssize_t};
#[cfg(windows)]
use winapi::{
    shared::minwindef::{BOOL, DWORD, LPCVOID, LPDWORD, LPVOID},
    shared::ntdef::{HANDLE, LPCWSTR},
    um::minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES},
};

/// Declare a few FFI functions ourselves, to show off the FFI ergonomics.
#[cfg(any(unix, target_os = "wasi"))]
extern "C" {
    pub fn open(pathname: *const c_char, flags: c_int, ...) -> Option<OwnedFd>;
    pub fn read(fd: BorrowedFd<'_>, ptr: *mut c_void, size: size_t) -> ssize_t;
    pub fn write(fd: BorrowedFd<'_>, ptr: *const c_void, size: size_t) -> ssize_t;
    pub fn close(fd: OwnedFd) -> c_int;
}
#[cfg(unix)]
pub use libc::O_CLOEXEC;
#[cfg(any(unix, target_os = "wasi"))]
pub use libc::{O_CREAT, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY};

// Define `O_CLOEXEC` for WASI manually; upstream fix submitted here:
// https://github.com/rust-lang/libc/pull/2210
#[cfg(target_os = "wasi")]
pub const O_CLOEXEC: c_int = 0;

/// The Windows analogs of the above. Note the use of [`OptionFileHandle`] as
/// the return type for `CreateFileW`, since that function is defined to return
/// [`INVALID_HANDLE_VALUE`] on error instead of null.
#[cfg(windows)]
extern "C" {
    pub fn CreateFileW(
        lpFileName: LPCWSTR,
        dwDesiredAccess: DWORD,
        dwShareMode: DWORD,
        lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
        dwCreationDisposition: DWORD,
        dwFlagsAndAttributes: DWORD,
        hTemplateFile: HANDLE,
    ) -> OptionFileHandle;
    pub fn ReadFile(
        hFile: HANDLE,
        lpBuffer: LPVOID,
        nNumberOfBytesToRead: DWORD,
        lpNumberOfBytesRead: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn WriteFile(
        hFile: BorrowedHandle<'_>,
        lpBuffer: LPCVOID,
        nNumberOfBytesToWrite: DWORD,
        lpNumberOfBytesWritten: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn CloseHandle(handle: OwnedHandle) -> BOOL;
}
#[cfg(windows)]
pub use winapi::{
    shared::minwindef::{FALSE, TRUE},
    um::fileapi::{CREATE_NEW, OPEN_EXISTING},
    um::winnt::{FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_GENERIC_WRITE},
};
