//! This is just a sample of what FFI using this crate can look like.

// Disable `improper_ctypes` warnings so that we don't lint about
// `Option<OwnedFd>` apperaing in an FFI signature. In the future rustc
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
    shared::minwindef::{BOOL, DWORD, LPCVOID, LPDWORD},
    shared::ntdef::{HANDLE, LPCWSTR},
    um::minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES},
};

/// Declare a few FFI functions ourselves, to show off the FFI ergonomics.
#[cfg(any(unix, target_os = "wasi"))]
extern "C" {
    pub fn open(pathname: *const c_char, flags: c_int, ...) -> Option<OwnedFd>;
    pub fn write(fd: BorrowedFd<'_>, ptr: *const c_void, size: size_t) -> ssize_t;
    pub fn close(fd: OwnedFd) -> c_int;
}
#[cfg(any(unix, target_os = "wasi"))]
pub use libc::{O_CREAT, O_RDONLY, O_RDWR, O_WRONLY};

/// The Windows analogs of the above.
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
