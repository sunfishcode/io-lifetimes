//! A simple example implementing the main traits for a type.

use io_lifetimes::OwnedFilelike;
#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, FromFd, OwnedFd};
#[cfg(windows)]
use io_lifetimes::{AsHandle, BorrowedHandle, FromHandle, OwnedHandle};

/// A wrapper around a file descriptor.
///
/// Implementing `AsFd`, `Into<OwnedFd>`, and `From<OwnedFd>` for a type that
/// wraps an `Owned*` is straightforward. `Owned*` types also automatically
/// close the handle in its `Drop`.
///
/// Should owning wrappers implement `AsRawFd`, `IntoRawFd`, and `FromRawFd`
/// too? They can, and there's no need to remove them from a type that already
/// implements them. But for new code, they can be omitted. Users that really
/// need the raw value can always do `as_fd().as_raw_fd()`,
/// `.into_fd().into_raw_fd()`, or `T::from_fd(OwnedFd::from_raw_fd(raw_fd))`.
/// But if possible, users should use just `as_fd`, `into_fd`, and `from_fd`
/// and avoid working with raw values altogether.
struct Thing {
    filelike: OwnedFilelike,
}

#[cfg(not(windows))]
impl AsFd for Thing {
    #[inline]
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.filelike.as_fd()
    }
}

#[cfg(not(windows))]
impl From<Thing> for OwnedFd {
    #[inline]
    fn from(owned: Thing) -> Self {
        owned.filelike
    }
}

#[cfg(not(windows))]
impl From<OwnedFd> for Thing {
    #[inline]
    fn from(filelike: OwnedFd) -> Self {
        Self { filelike }
    }
}

#[cfg(windows)]
impl AsHandle for Thing {
    #[inline]
    fn as_handle(&self) -> BorrowedHandle<'_> {
        self.filelike.as_handle()
    }
}

#[cfg(windows)]
impl From<Thing> for OwnedHandle {
    #[inline]
    fn from(owned: Thing) -> Self {
        owned.filelike
    }
}

#[cfg(windows)]
impl From<OwnedHandle> for Thing {
    #[inline]
    fn from(filelike: OwnedHandle) -> Self {
        Self { filelike }
    }
}

fn main() {
    use io_lifetimes::{AsFilelike, FromFilelike, IntoFilelike};

    // Minimally exercise `Thing`'s Posix-ish API.
    #[cfg(not(windows))]
    {
        let file = std::fs::File::open("Cargo.toml").unwrap();
        let thing = Thing::from_into_fd(file);
        let _ = thing.as_fd();
        let _: OwnedFd = thing.into();
    }

    // Minimally exercise `Thing`'s Windows API.
    #[cfg(windows)]
    {
        let file = std::fs::File::open("Cargo.toml").unwrap();
        let thing = Thing::from_into_handle(file);
        let _ = thing.as_handle();
        let _: OwnedHandle = thing.into();
    }

    // Implementing the above traits makes the blanket impls for the portable
    // `Filelike` traits available too.
    {
        let file = std::fs::File::open("Cargo.toml").unwrap();
        let thing = Thing::from_into_filelike(file);
        let _ = thing.as_filelike();
        let _ = thing.into_filelike();
    }
}
