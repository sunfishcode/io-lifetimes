//! A simple testcase that prints a few messages to the console, demonstrating
//! the io-lifetimes API.

#![cfg_attr(not(rustc_attrs), allow(unused_imports))]

use io_lifetimes::example_ffi::*;
use std::fs::File;
use std::io::{self, Write};

#[cfg(unix)]
use io_lifetimes::{AsFd, FromFd, IntoFd, OwnedFd};

#[cfg(windows)]
use io_lifetimes::{AsHandle, FromHandle, IntoHandle, OwnedHandle};
#[cfg(windows)]
use std::{convert::TryInto, ptr::null_mut};

#[cfg(all(rustc_attrs, unix))]
fn main() -> io::Result<()> {
    let fd = unsafe {
        // Open a file, which returns an `Option<OwnedFd>`, which we can
        // maybe convert into an `OwnedFile`.
        let fd: OwnedFd = open("/dev/stdout\0".as_ptr() as *const _, O_WRONLY | O_CLOEXEC)
            .ok_or_else(io::Error::last_os_error)?;

        // Borrow the fd to write to it.
        let result = write(fd.as_fd(), "hello, world\n".as_ptr() as *const _, 13);
        match result {
            -1 => return Err(io::Error::last_os_error()),
            13 => (),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }

        fd
    };

    // Convert into a `File`. No `unsafe` here!
    let mut file = File::from_fd(fd);
    writeln!(&mut file, "greetings, y'all")?;

    // We can borrow a `BorrowedFd` from a `File`.
    unsafe {
        let result = write(file.as_fd(), "sup?\n".as_ptr() as *const _, 5);
        match result {
            -1 => return Err(io::Error::last_os_error()),
            5 => (),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }
    }

    // Now back to `OwnedFd`.
    let fd = file.into_fd();

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
            ['C' as u16, 'O' as _, 'N' as _, 0].as_ptr(),
            FILE_GENERIC_WRITE,
            0,
            null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        )
        .try_into()
        .map_err(|()| io::Error::last_os_error())?;

        // Borrow the handle to write to it.
        let mut number_of_bytes_written = 0;
        let result = WriteFile(
            handle.as_handle(),
            "hello, world\n".as_ptr() as *const _,
            13,
            &mut number_of_bytes_written,
            null_mut(),
        );
        match (result, number_of_bytes_written) {
            (FALSE, _) => return Err(io::Error::last_os_error()),
            (_, 13) => (),
            (_, _) => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }

        handle
    };

    // Convert into a `File`. No `unsafe` here!
    let mut file = File::from_handle(handle);
    writeln!(&mut file, "greetings, y'all")?;

    // We can borrow a `BorrowedFd` from a `File`.
    unsafe {
        let mut number_of_bytes_written = 0;
        let result = WriteFile(
            file.as_handle(),
            "sup?\n".as_ptr() as *const _,
            5,
            &mut number_of_bytes_written,
            null_mut(),
        );
        match (result, number_of_bytes_written) {
            (FALSE, _) => return Err(io::Error::last_os_error()),
            (_, 5) => (),
            (_, _) => return Err(io::Error::new(io::ErrorKind::Other, "short write")),
        }
    }

    // Now back to `OwnedFd`.
    let handle = file.into_handle();

    unsafe {
        // This isn't needed, since `handle` is owned and would close itself on
        // drop automatically, but it makes a nice demo of passing an `OwnedHandle`
        // into an FFI call.
        CloseHandle(handle);
    }

    Ok(())
}

#[cfg(not(any(windows, rustc_attrs)))]
fn main() {
    println!("On Unix, this example requires Rust nightly (for `rustc_attrs`).");
}
