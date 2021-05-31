#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

use std::mem::size_of;

#[cfg(any(unix, target_os = "wasi"))]
use io_experiment::{BorrowedFd, OwnedFd};
#[cfg(windows)]
use io_experiment::{BorrowedHandle, BorrowedSocket, OwnedHandle, OwnedSocket};

#[cfg(unix)]
use std::os::unix::io::RawFd;
#[cfg(target_os = "wasi")]
use std::os::wasi::io::RawFd;
#[cfg(windows)]
use std::os::windows::io::{RawHandle, RawSocket};

#[cfg(any(unix, target_os = "wasi"))]
#[test]
fn test_niche_optimizations() {
    assert_eq!(size_of::<Option<OwnedFd>>(), size_of::<RawFd>());
    assert_eq!(size_of::<Option<BorrowedFd<'static>>>(), size_of::<RawFd>());
}

#[cfg(windows)]
#[test]
fn test_niche_optimizations() {
    assert_eq!(size_of::<Option<OwnedHandle>>(), size_of::<RawHandle>());
    assert_eq!(
        size_of::<Option<BorrowedHandle<'static>>>(),
        size_of::<RawHandle>(),
    );
    assert_eq!(size_of::<Option<OwnedSocket>>(), size_of::<RawSocket>());
    assert_eq!(
        size_of::<Option<BorrowedSocket<'static>>>(),
        size_of::<RawSocket>(),
    );
}
