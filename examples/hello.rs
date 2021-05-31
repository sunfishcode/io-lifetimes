use std::convert::TryInto;
use std::fs::File;
use std::io::{self, Write};

#[cfg(unix)]
use io_experiment::{AsBorrowedFd, BorrowedFd, FromOwnedFd, IntoOwnedFd, OptionFd, OwnedFd};

#[cfg(windows)]
use io_experiment::{
    AsBorrowedHandle, BorrowedHandle, FromOwnedHandle, IntoOwnedHandle, OptionFileHandle,
    OwnedHandle,
};

#[cfg(unix)]
use libc::{c_char, c_int, c_void, size_t, ssize_t};

#[cfg(windows)]
use std::ptr;
#[cfg(windows)]
use winapi::{
    shared::minwindef::{BOOL, DWORD, FALSE, LPCVOID, LPDWORD},
    shared::ntdef::{HANDLE, LPCWSTR},
    um::fileapi::OPEN_EXISTING,
    um::minwinbase::{LPOVERLAPPED, LPSECURITY_ATTRIBUTES},
    um::winnt::{FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_WRITE},
};

/// Declare a few FFI functions ourselves, to show off the FFI ergonomics.
#[cfg(unix)]
extern "C" {
    pub fn open(pathname: *const c_char, flags: c_int, ...) -> OptionFd;
    pub fn write(fd: BorrowedFd, ptr: *const c_void, size: size_t) -> ssize_t;
    pub fn close(fd: OwnedFd) -> c_int;
}

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
        hFile: BorrowedHandle,
        lpBuffer: LPCVOID,
        nNumberOfBytesToWrite: DWORD,
        lpNumberOfBytesWritten: LPDWORD,
        lpOverlapped: LPOVERLAPPED,
    ) -> BOOL;
    pub fn CloseHandle(handle: OwnedHandle) -> BOOL;
}

/// A simple testcase that prints a few messages to the console, demonstrating
/// the io-experiment API.
#[cfg(unix)]
fn main() -> io::Result<()> {
    let fd = unsafe {
        // Open a file, which returns an `OptionFd`, which we can fallibly
        // convert into an `OwnedFile`.
        let fd: OwnedFd = open("/dev/stdout\0".as_ptr() as *const _, libc::O_WRONLY)
            .try_into()
            .map_err(|()| io::Error::last_os_error())?;

        // Borrow the fd to write to it.
        let result = write(
            fd.as_borrowed_fd(),
            "hello, world\n".as_ptr() as *const _,
            13,
        );
        match result {
            -1 => return Err(io::Error::last_os_error()),
            13 => (),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }

        fd
    };

    // Convert into a `File`. No `unsafe` here!
    let mut file = File::from_owned_fd(fd);
    writeln!(&mut file, "greetings, y'all")?;

    // We can borrow a `BorrowedFd` from a `File`.
    unsafe {
        let result = write(file.as_borrowed_fd(), "sup?\n".as_ptr() as *const _, 5);
        match result {
            -1 => return Err(io::Error::last_os_error()),
            5 => (),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }
    }

    // Now back to `OwnedFd`.
    let fd = file.into_owned_fd();

    unsafe {
        // This isn't needed, since `fd` is owned and would close itself on
        // drop automatically, but it makes a nice demo of passing an `OwnedFd`
        // into an FFI call.
        close(fd);
    }

    Ok(())
}

/// The Windows analog of the above.
#[cfg(windows)]
fn main() -> io::Result<()> {
    let handle = unsafe {
        // Open a file, which returns an `OptionFileHandle`, which we can fallibly
        // convert into an `OwnedFile`.
        let handle: OwnedHandle = CreateFileW(
            ['C' as u16, 'O' as u16, 'N' as u16, 0].as_ptr(),
            FILE_GENERIC_WRITE,
            0,
            ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            ptr::null_mut(),
        )
        .try_into()
        .map_err(|()| io::Error::last_os_error())?;

        // Borrow the handle to write to it.
        let mut number_of_bytes_written: DWORD = 0;
        let result = WriteFile(
            handle.as_borrowed_handle(),
            "hello, world\n".as_ptr() as *const _,
            13,
            &mut number_of_bytes_written as *mut _,
            ptr::null_mut(),
        );
        match (result, number_of_bytes_written) {
            (FALSE, _) => return Err(io::Error::last_os_error()),
            (_, 13) => (),
            (_, _) => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }

        handle
    };

    // Convert into a `File`. No `unsafe` here!
    let mut file = File::from_owned_handle(handle);
    writeln!(&mut file, "greetings, y'all")?;

    // We can borrow a `BorrowedFd` from a `File`.
    unsafe {
        let mut number_of_bytes_written: DWORD = 0;
        let result = WriteFile(
            file.as_borrowed_handle(),
            "sup?\n".as_ptr() as *const _,
            5,
            &mut number_of_bytes_written as *mut _,
            ptr::null_mut(),
        );
        match (result, number_of_bytes_written) {
            (FALSE, _) => return Err(io::Error::last_os_error()),
            (_, 5) => (),
            (_, _) => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }
    }

    // Now back to `OwnedFd`.
    let handle = file.into_owned_handle();

    unsafe {
        // This isn't needed, since `handle` is owned and would close itself on
        // drop automatically, but it makes a nice demo of passing an `OwnedHandle`
        // into an FFI call.
        CloseHandle(handle);
    }

    Ok(())
}
