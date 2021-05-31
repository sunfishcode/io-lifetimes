//! On Unix, "everything is a file descriptor". On Windows, file/pipe/process
//! handles are distinct from socket descriptors. This file provides a minimal
//! layer of portability over this difference.
//!
//! TODO: Should this layer be folded into types.rs/traits.rs?

#[cfg(any(unix, target_os = "wasi"))]
use crate::{AsBorrowedFd, BorrowedFd, FromOwnedFd, IntoOwnedFd, OwnedFd};
#[cfg(windows)]
use crate::{
    AsBorrowedHandle, AsBorrowedSocket, BorrowedHandle, BorrowedSocket, FromOwnedHandle,
    FromOwnedSocket, IntoOwnedHandle, IntoOwnedSocket, OwnedHandle, OwnedSocket,
};

/// A borrowed filelike reference.
#[cfg(any(unix, target_os = "wasi"))]
pub type BorrowedFilelike<'owned> = BorrowedFd<'owned>;

/// A borrowed filelike reference.
#[cfg(windows)]
pub type BorrowedFilelike<'owned> = BorrowedHandle<'owned>;

/// A borrowed socketlike reference.
#[cfg(any(unix, target_os = "wasi"))]
pub type BorrowedSocketlike<'owned> = BorrowedFd<'owned>;

/// A borrowed socketlike reference.
#[cfg(windows)]
pub type BorrowedSocketlike<'owned> = BorrowedSocket<'owned>;

/// An owned filelike object.
#[cfg(any(unix, target_os = "wasi"))]
pub type OwnedFilelike = OwnedFd;

/// An owned filelike object.
#[cfg(windows)]
pub type OwnedFilelike = OwnedHandle;

/// An owned socketlike object.
#[cfg(any(unix, target_os = "wasi"))]
pub type OwnedSocketlike = OwnedFd;

/// An owned socketlike object.
#[cfg(windows)]
pub type OwnedSocketlike = OwnedSocket;

/// A trait to borrow a filelike reference from an underlying object.
#[cfg(any(unix, target_os = "wasi"))]
pub trait AsBorrowedFilelike: AsBorrowedFd {
    /// Extracts the filelike reference.
    fn as_borrowed_filelike(&self) -> BorrowedFilelike<'_>;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: AsBorrowedFd> AsBorrowedFilelike for T {
    #[inline]
    fn as_borrowed_filelike(&self) -> BorrowedFilelike<'_> {
        self.as_borrowed_fd()
    }
}

/// A trait to borrow a filelike reference from an underlying object.
#[cfg(windows)]
pub trait AsBorrowedFilelike: AsBorrowedHandle {
    /// Extracts the filelike reference.
    fn as_borrowed_filelike(&self) -> BorrowedFilelike<'_>;
}

#[cfg(windows)]
impl<T: AsBorrowedHandle> AsBorrowedFilelike for T {
    #[inline]
    fn as_borrowed_filelike(&self) -> BorrowedFilelike<'_> {
        self.as_borrowed_handle()
    }
}

/// A trait to borrow a socketlike reference from an underlying object.
#[cfg(any(unix, target_os = "wasi"))]
pub trait AsBorrowedSocketlike: AsBorrowedFd {
    /// Extracts the socketlike reference.
    fn as_borrowed_socketlike(&self) -> BorrowedSocketlike<'_>;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: AsBorrowedFd> AsBorrowedSocketlike for T {
    #[inline]
    fn as_borrowed_socketlike(&self) -> BorrowedSocketlike<'_> {
        self.as_borrowed_fd()
    }
}

/// A trait to borrow a socketlike reference from an underlying object.
#[cfg(windows)]
pub trait AsBorrowedSocketlike: AsBorrowedSocket {
    /// Extracts the socketlike reference.
    fn as_borrowed_socketlike(&self) -> BorrowedSocketlike<'_>;
}

#[cfg(windows)]
impl<T: AsBorrowedSocket> AsBorrowedSocketlike for T {
    #[inline]
    fn as_borrowed_socketlike(&self) -> BorrowedSocketlike<'_> {
        self.as_borrowed_socket()
    }
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its filelike object.
#[cfg(any(unix, target_os = "wasi"))]
pub trait IntoOwnedFilelike: IntoOwnedFd {
    /// Consumes this object, returning the underlying filelike object.
    fn into_owned_filelike(self) -> OwnedFilelike;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: IntoOwnedFd> IntoOwnedFilelike for T {
    #[inline]
    fn into_owned_filelike(self) -> OwnedFilelike {
        self.into_owned_fd()
    }
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its filelike object.
#[cfg(windows)]
pub trait IntoOwnedFilelike: IntoOwnedHandle {
    /// Consumes this object, returning the underlying filelike object.
    fn into_owned_filelike(self) -> OwnedFilelike;
}

#[cfg(windows)]
impl<T: IntoOwnedHandle> IntoOwnedFilelike for T {
    #[inline]
    fn into_owned_filelike(self) -> OwnedFilelike {
        self.into_owned_handle()
    }
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its socketlike object.
#[cfg(any(unix, target_os = "wasi"))]
pub trait IntoOwnedSocketlike: IntoOwnedFd {
    /// Consumes this object, returning the underlying socketlike object.
    fn into_owned_socketlike(self) -> OwnedSocketlike;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: IntoOwnedFd> IntoOwnedSocketlike for T {
    #[inline]
    fn into_owned_socketlike(self) -> OwnedSocketlike {
        self.into_owned_fd()
    }
}

/// A trait to express the ability to consume an object and acquire ownership
/// of its socketlike object.
#[cfg(windows)]
pub trait IntoOwnedSocketlike: IntoOwnedSocket {
    /// Consumes this object, returning the underlying socketlike object.
    fn into_owned_socketlike(self) -> OwnedSocketlike;
}

#[cfg(windows)]
impl<T: IntoOwnedSocket> IntoOwnedSocketlike for T {
    #[inline]
    fn into_owned_socketlike(self) -> OwnedSocketlike {
        self.into_owned_socket()
    }
}

/// A trait to express the ability to construct an object from a filelike
/// object.
#[cfg(any(unix, target_os = "wasi"))]
pub trait FromOwnedFilelike: FromOwnedFd {
    /// Constructs a new instance of `Self` from the given filelike object.
    fn from_owned_filelike(owned: OwnedFilelike) -> Self;

    /// Constructs a new instance of `Self` from the given filelike object
    /// converted from `into_owned`.
    fn from_into_owned_filelike<Owned: IntoOwnedFilelike>(owned: Owned) -> Self;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: FromOwnedFd> FromOwnedFilelike for T {
    #[inline]
    fn from_owned_filelike(owned: OwnedFilelike) -> Self {
        Self::from_owned_fd(owned)
    }

    #[inline]
    fn from_into_owned_filelike<Owned: IntoOwnedFilelike>(owned: Owned) -> Self {
        Self::from_owned_filelike(owned.into_owned_filelike())
    }
}

/// A trait to express the ability to construct an object from a filelike
/// object.
#[cfg(windows)]
pub trait FromOwnedFilelike: FromOwnedHandle {
    /// Constructs a new instance of `Self` from the given filelike object.
    fn from_owned_filelike(owned: OwnedFilelike) -> Self;

    /// Constructs a new instance of `Self` from the given filelike object
    /// converted from `into_owned`.
    fn from_into_owned_filelike<Owned: IntoOwnedFilelike>(owned: Owned) -> Self;
}

#[cfg(windows)]
impl<T: FromOwnedHandle> FromOwnedFilelike for T {
    #[inline]
    fn from_owned_filelike(owned: OwnedFilelike) -> Self {
        Self::from_owned_handle(owned)
    }

    #[inline]
    fn from_into_owned_filelike<Owned: IntoOwnedFilelike>(owned: Owned) -> Self {
        Self::from_owned_filelike(owned.into_owned_filelike())
    }
}

/// A trait to express the ability to construct an object from a socketlike
/// object.
#[cfg(any(unix, target_os = "wasi"))]
pub trait FromOwnedSocketlike: FromOwnedFd {
    /// Constructs a new instance of `Self` from the given socketlike object.
    fn from_owned_socketlike(owned: OwnedSocketlike) -> Self;

    /// Constructs a new instance of `Self` from the given socketlike object
    /// converted from `into_owned`.
    fn from_into_owned_socketlike<Owned: IntoOwnedSocketlike>(owned: Owned) -> Self;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: FromOwnedFd> FromOwnedSocketlike for T {
    #[inline]
    fn from_owned_socketlike(owned: OwnedSocketlike) -> Self {
        Self::from_owned_fd(owned)
    }

    #[inline]
    fn from_into_owned_socketlike<Owned: IntoOwnedSocketlike>(owned: Owned) -> Self {
        Self::from_owned_socketlike(owned.into_owned_socketlike())
    }
}

/// A trait to express the ability to construct an object from a socketlike
/// object.
#[cfg(windows)]
pub trait FromOwnedSocketlike: FromOwnedSocket {
    /// Constructs a new instance of `Self` from the given socketlike object.
    fn from_owned_socketlike(owned: OwnedSocketlike) -> Self;

    /// Constructs a new instance of `Self` from the given socketlike object
    /// converted from `into_owned`.
    fn from_into_owned_socketlike<Owned: IntoOwnedSocketlike>(owned: Owned) -> Self;
}

#[cfg(windows)]
impl<T: FromOwnedSocket> FromOwnedSocketlike for T {
    #[inline]
    fn from_owned_socketlike(owned: OwnedSocketlike) -> Self {
        Self::from_owned_socket(owned)
    }

    #[inline]
    fn from_into_owned_socketlike<Owned: IntoOwnedSocketlike>(owned: Owned) -> Self {
        Self::from_owned_socketlike(owned.into_owned_socketlike())
    }
}
