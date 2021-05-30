#[cfg(unix)]
use std::mem::size_of;

#[cfg(unix)]
use io_experiment::{BorrowedFd, OwnedFd};

#[cfg(unix)]
use std::os::unix::io::RawFd;

#[cfg(unix)]
#[test]
fn test_niche_optimizations() {
    assert_eq!(size_of::<Option<OwnedFd>>(), size_of::<RawFd>());
    assert_eq!(
        size_of::<Option<BorrowedFd<'static>>>(),
        size_of::<BorrowedFd<'static>>()
    );
}
