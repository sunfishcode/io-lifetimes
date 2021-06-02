//! On Unix, "everything is a file descriptor". On Windows, file/pipe/process
//! handles are distinct from socket descriptors. This file provides a minimal
//! layer of portability over this difference.
//!
//! TODO: Should this layer be folded into types.rs/traits.rs?

use crate::views::{FilelikeView, SocketlikeView};
#[cfg(any(unix, target_os = "wasi"))]
use crate::{AsFd, BorrowedFd, FromFd, IntoFd, OwnedFd};
#[cfg(windows)]
use crate::{
    AsHandle, AsSocket, BorrowedHandle, BorrowedSocket, FromHandle, FromSocket, IntoHandle,
    IntoSocket, OwnedHandle, OwnedSocket,
};
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(target_os = "wasi")]
use std::os::wasi::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{
    AsRawHandle, AsRawSocket, FromRawHandle, FromRawSocket, IntoRawHandle, IntoRawSocket,
    RawHandle, RawSocket,
};

/// A reference to a filelike object.
///
/// This is a portability abstraction over Unix-like [`BorrowedFd`] and
/// Windows' `BorrowedHandle`.
#[cfg(any(unix, target_os = "wasi"))]
pub type BorrowedFilelike<'owned> = BorrowedFd<'owned>;

/// A reference to a filelike object.
///
/// This is a portability abstraction over Unix-like `BorrowedFd` and
/// Windows' [`BorrowedHandle`].
#[cfg(windows)]
pub type BorrowedFilelike<'owned> = BorrowedHandle<'owned>;

/// A reference to a socketlike object.
///
/// This is a portability abstraction over Unix-like [`BorrowedFd`] and
/// Windows' `BorrowedSocket`.
#[cfg(any(unix, target_os = "wasi"))]
pub type BorrowedSocketlike<'owned> = BorrowedFd<'owned>;

/// A reference to a socketlike object.
///
/// This is a portability abstraction over Unix-like `BorrowedFd` and
/// Windows' [`BorrowedSocket`].
#[cfg(windows)]
pub type BorrowedSocketlike<'owned> = BorrowedSocket<'owned>;

/// An owned filelike object.
///
/// This is a portability abstraction over Unix-like [`OwnedFd`] and
/// Windows' `OwnedHandle`.
#[cfg(any(unix, target_os = "wasi"))]
pub type OwnedFilelike = OwnedFd;

/// An owned filelike object.
///
/// This is a portability abstraction over Unix-like `OwnedFd` and
/// Windows' [`OwnedHandle`].
#[cfg(windows)]
pub type OwnedFilelike = OwnedHandle;

/// An owned socketlike object.
///
/// This is a portability abstraction over Unix-like [`OwnedFd`] and
/// Windows' `OwnedSocket`.
#[cfg(any(unix, target_os = "wasi"))]
pub type OwnedSocketlike = OwnedFd;

/// An owned socketlike object.
///
/// This is a portability abstraction over Unix-like `OwnedFd` and
/// Windows' [`OwnedSocket`].
#[cfg(windows)]
pub type OwnedSocketlike = OwnedSocket;

#[cfg(any(unix, target_os = "wasi"))]
pub(crate) type RawFilelike = RawFd;

#[cfg(windows)]
pub(crate) type RawFilelike = RawHandle;

#[cfg(any(unix, target_os = "wasi"))]
pub(crate) type RawSocketlike = RawFd;

#[cfg(windows)]
pub(crate) type RawSocketlike = RawSocket;

/// A portable trait to borrow a reference from an underlying filelike object.
///
/// This is a portability abstraction over Unix-like [`AsFd`] and Windows'
/// `AsHandle`. It also provides the `as_filelike_view` convenience function
/// providing typed views.
#[cfg(any(unix, target_os = "wasi"))]
pub trait AsFilelike: AsFd {
    /// Borrows the reference.
    fn as_filelike(&self) -> BorrowedFilelike<'_>;

    /// Return a borrowing view of a resource which dereferences to a `&Target`
    /// or `&mut Target`.
    ///
    /// This creates a temporary instance of a `Target` within a
    /// `ManuallyDrop`, so any additional resources held by `Target` are
    /// leaked. Consequently, this function should only be used with types
    /// like [`File`] which do not acquire any additional resources.
    ///
    /// [`File`]: std::fs::File
    fn as_filelike_view<Target: FromFilelike>(&self) -> FilelikeView<'_, Target>;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: AsFd> AsFilelike for T {
    #[inline]
    fn as_filelike(&self) -> BorrowedFilelike<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_filelike_view<Target: FromFilelike>(&self) -> FilelikeView<'_, Target> {
        FilelikeView::new(self)
    }
}

/// A portable trait to borrow a reference from an underlying filelike object.
///
/// This is a portability abstraction over Unix-like `AsFd` and Windows'
/// [`AsHandle`]. It also provides the `as_filelike_view` convenience function
/// providing typed views.
#[cfg(windows)]
pub trait AsFilelike: AsHandle {
    /// Borrows the reference.
    fn as_filelike(&self) -> BorrowedFilelike<'_>;

    /// Return a borrowing view of a resource which dereferences to a `&Target`
    /// or `&mut Target`.
    ///
    /// This creates a temporary instance of a `Target` within a
    /// `ManuallyDrop`, so any additional resources held by `Target` are
    /// leaked. Consequently, this function should only be used with types
    /// like [`File`] which do not acquire any additional resources.
    ///
    /// [`File`]: std::fs::File
    fn as_filelike_view<Target: FromFilelike>(&self) -> FilelikeView<'_, Target>;
}

#[cfg(windows)]
impl<T: AsHandle> AsFilelike for T {
    #[inline]
    fn as_filelike(&self) -> BorrowedFilelike<'_> {
        self.as_handle()
    }

    #[inline]
    fn as_filelike_view<Target: FromFilelike>(&self) -> FilelikeView<'_, Target> {
        FilelikeView::new(self)
    }
}

/// A portable trait to borrow a reference from an underlying socketlike
/// object.
///
/// This is a portability abstraction over Unix-like [`AsFd`] and Windows'
/// `AsSocket`. It also provides the `as_socketlike_view` convenience
/// function providing typed views.
#[cfg(any(unix, target_os = "wasi"))]
pub trait AsSocketlike: AsFd {
    /// Borrows the reference.
    fn as_socketlike(&self) -> BorrowedSocketlike<'_>;

    /// Return a borrowing view of a resource which dereferences to a `&Target`
    /// or `&mut Target`.
    ///
    /// This creates a temporary instance of a `Target` within a
    /// `ManuallyDrop`, so any additional resources held by `Target` are
    /// leaked. Consequently, this function should only be used with types
    /// like [`TcpStream`] which do not acquire any additional resources.
    ///
    /// [`TcpStream`]: std::net::TcpStream
    fn as_socketlike_view<Target: FromSocketlike>(&self) -> SocketlikeView<'_, Target>;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: AsFd> AsSocketlike for T {
    #[inline]
    fn as_socketlike(&self) -> BorrowedSocketlike<'_> {
        self.as_fd()
    }

    #[inline]
    fn as_socketlike_view<Target: FromSocketlike>(&self) -> SocketlikeView<'_, Target> {
        SocketlikeView::new(self)
    }
}

/// A portable trait to borrow a reference from an underlying socketlike
/// object.
///
/// This is a portability abstraction over Unix-like `AsFd` and Windows'
/// [`AsSocket`]. It also provides the `as_socketlike_view` convenience
/// function providing typed views.
#[cfg(windows)]
pub trait AsSocketlike: AsSocket {
    /// Borrows the reference.
    fn as_socketlike(&self) -> BorrowedSocketlike<'_>;

    /// Return a borrowing view of a resource which dereferences to a `&Target`
    /// or `&mut Target`.
    ///
    /// This creates a temporary instance of a `Target` within a
    /// `ManuallyDrop`, so any additional resources held by `Target` are
    /// leaked. Consequently, this function should only be used with types
    /// like [`TcpStream`] which do not acquire any additional resources.
    ///
    /// [`TcpStream`]: std::net::TcpStream
    fn as_socketlike_view<Target: FromSocketlike>(&self) -> SocketlikeView<'_, Target>;
}

#[cfg(windows)]
impl<T: AsSocket> AsSocketlike for T {
    #[inline]
    fn as_socketlike(&self) -> BorrowedSocketlike<'_> {
        self.as_socket()
    }

    #[inline]
    fn as_socketlike_view<Target: FromSocketlike>(&self) -> SocketlikeView<'_, Target> {
        SocketlikeView::new(self)
    }
}

/// A portable trait to express the ability to consume an object and acquire
/// ownership of its filelike object.
///
/// This is a portability abstraction over Unix-like [`IntoFd`] and Windows'
/// `IntoHandle`.
#[cfg(any(unix, target_os = "wasi"))]
pub trait IntoFilelike: IntoFd {
    /// Consumes this object, returning the underlying filelike object.
    fn into_filelike(self) -> OwnedFilelike;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: IntoFd> IntoFilelike for T {
    #[inline]
    fn into_filelike(self) -> OwnedFilelike {
        self.into_fd()
    }
}

/// A portable trait to express the ability to consume an object and acquire
/// ownership of its filelike object.
///
/// This is a portability abstraction over Unix-like `IntoFd` and Windows'
/// [`IntoHandle`].
#[cfg(windows)]
pub trait IntoFilelike: IntoHandle {
    /// Consumes this object, returning the underlying filelike object.
    fn into_filelike(self) -> OwnedFilelike;
}

#[cfg(windows)]
impl<T: IntoHandle> IntoFilelike for T {
    #[inline]
    fn into_filelike(self) -> OwnedFilelike {
        self.into_handle()
    }
}

/// A portable trait to express the ability to consume an object and acquire
/// ownership of its socketlike object.
///
/// This is a portability abstraction over Unix-like [`IntoFd`] and Windows'
/// `IntoSocket`.
#[cfg(any(unix, target_os = "wasi"))]
pub trait IntoSocketlike: IntoFd {
    /// Consumes this object, returning the underlying socketlike object.
    fn into_socketlike(self) -> OwnedSocketlike;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: IntoFd> IntoSocketlike for T {
    #[inline]
    fn into_socketlike(self) -> OwnedSocketlike {
        self.into_fd()
    }
}

/// A portable trait to express the ability to consume an object and acquire
/// ownership of its socketlike object.
///
/// This is a portability abstraction over Unix-like `IntoFd` and Windows'
/// [`IntoSocket`].
#[cfg(windows)]
pub trait IntoSocketlike: IntoSocket {
    /// Consumes this object, returning the underlying socketlike object.
    fn into_socketlike(self) -> OwnedSocketlike;
}

#[cfg(windows)]
impl<T: IntoSocket> IntoSocketlike for T {
    #[inline]
    fn into_socketlike(self) -> OwnedSocketlike {
        self.into_socket()
    }
}

/// A portable trait to express the ability to construct an object from a
/// filelike object.
///
/// This is a portability abstraction over Unix-like [`FromFd`] and Windows'
/// `FromHandle`. It also provides the `from_into_filelike` convenience
/// function providing simplified from+into conversions.
#[cfg(any(unix, target_os = "wasi"))]
pub trait FromFilelike: FromFd {
    /// Constructs a new instance of `Self` from the given filelike object.
    fn from_filelike(owned: OwnedFilelike) -> Self;

    /// Constructs a new instance of `Self` from the given filelike object
    /// converted from `into_owned`.
    fn from_into_filelike<Owned: IntoFilelike>(owned: Owned) -> Self;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: FromFd> FromFilelike for T {
    #[inline]
    fn from_filelike(owned: OwnedFilelike) -> Self {
        Self::from_fd(owned)
    }

    #[inline]
    fn from_into_filelike<Owned: IntoFilelike>(owned: Owned) -> Self {
        Self::from_filelike(owned.into_filelike())
    }
}

/// A portable trait to express the ability to construct an object from a
/// filelike object.
///
/// This is a portability abstraction over Unix-like `FromFd` and Windows'
/// [`FromHandle`]. It also provides the `from_into_filelike` convenience
/// function providing simplified from+into conversions.
#[cfg(windows)]
pub trait FromFilelike: FromHandle {
    /// Constructs a new instance of `Self` from the given filelike object.
    fn from_filelike(owned: OwnedFilelike) -> Self;

    /// Constructs a new instance of `Self` from the given filelike object
    /// converted from `into_owned`.
    fn from_into_filelike<Owned: IntoFilelike>(owned: Owned) -> Self;
}

#[cfg(windows)]
impl<T: FromHandle> FromFilelike for T {
    #[inline]
    fn from_filelike(owned: OwnedFilelike) -> Self {
        Self::from_handle(owned)
    }

    #[inline]
    fn from_into_filelike<Owned: IntoFilelike>(owned: Owned) -> Self {
        Self::from_filelike(owned.into_filelike())
    }
}

/// A portable trait to express the ability to construct an object from a
/// socketlike object.
///
/// This is a portability abstraction over Unix-like [`FromFd`] and Windows'
/// `FromSocket`. It also provides the `from_into_socketlike` convenience
/// function providing simplified from+into conversions.
#[cfg(any(unix, target_os = "wasi"))]
pub trait FromSocketlike: FromFd {
    /// Constructs a new instance of `Self` from the given socketlike object.
    fn from_socketlike(owned: OwnedSocketlike) -> Self;

    /// Constructs a new instance of `Self` from the given socketlike object
    /// converted from `into_owned`.
    fn from_into_socketlike<Owned: IntoSocketlike>(owned: Owned) -> Self;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: FromFd> FromSocketlike for T {
    #[inline]
    fn from_socketlike(owned: OwnedSocketlike) -> Self {
        Self::from_fd(owned)
    }

    #[inline]
    fn from_into_socketlike<Owned: IntoSocketlike>(owned: Owned) -> Self {
        Self::from_socketlike(owned.into_socketlike())
    }
}

/// A portable trait to express the ability to construct an object from a
/// socketlike object.
///
/// This is a portability abstraction over Unix-like `FromFd` and Windows'
/// [`FromSocket`]. It also provides the `from_into_socketlike` convenience
/// function providing simplified from+into conversions.
#[cfg(windows)]
pub trait FromSocketlike: FromSocket {
    /// Constructs a new instance of `Self` from the given socketlike object.
    fn from_socketlike(owned: OwnedSocketlike) -> Self;

    /// Constructs a new instance of `Self` from the given socketlike object
    /// converted from `into_owned`.
    fn from_into_socketlike<Owned: IntoSocketlike>(owned: Owned) -> Self;
}

#[cfg(windows)]
impl<T: FromSocket> FromSocketlike for T {
    #[inline]
    fn from_socketlike(owned: OwnedSocketlike) -> Self {
        Self::from_socket(owned)
    }

    #[inline]
    fn from_into_socketlike<Owned: IntoSocketlike>(owned: Owned) -> Self {
        Self::from_socketlike(owned.into_socketlike())
    }
}

#[cfg(any(unix, target_os = "wasi"))]
pub(crate) trait AsRawFilelike: AsRawFd {
    fn as_raw_filelike(&self) -> RawFilelike;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: AsRawFd> AsRawFilelike for T {
    #[inline]
    fn as_raw_filelike(&self) -> RawFilelike {
        self.as_raw_fd()
    }
}

#[cfg(windows)]
pub(crate) trait AsRawFilelike: AsRawHandle {
    fn as_raw_filelike(&self) -> RawFilelike;
}

#[cfg(windows)]
impl<T: AsRawHandle> AsRawFilelike for T {
    #[inline]
    fn as_raw_filelike(&self) -> RawFilelike {
        self.as_raw_handle()
    }
}

#[cfg(any(unix, target_os = "wasi"))]
pub(crate) trait AsRawSocketlike: AsRawFd {
    fn as_raw_socketlike(&self) -> RawSocketlike;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: AsRawFd> AsRawSocketlike for T {
    #[inline]
    fn as_raw_socketlike(&self) -> RawSocketlike {
        self.as_raw_fd()
    }
}

#[cfg(windows)]
pub(crate) trait AsRawSocketlike: AsRawSocket {
    fn as_raw_socketlike(&self) -> RawSocketlike;
}

#[cfg(windows)]
impl<T: AsRawSocket> AsRawSocketlike for T {
    #[inline]
    fn as_raw_socketlike(&self) -> RawSocketlike {
        self.as_raw_socket()
    }
}

#[cfg(any(unix, target_os = "wasi"))]
pub(crate) trait IntoRawFilelike: IntoRawFd {
    fn into_raw_filelike(self) -> RawFilelike;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: IntoRawFd> IntoRawFilelike for T {
    #[inline]
    fn into_raw_filelike(self) -> RawFilelike {
        self.into_raw_fd()
    }
}

#[cfg(windows)]
pub(crate) trait IntoRawFilelike: IntoRawHandle {
    fn into_raw_filelike(self) -> RawFilelike;
}

#[cfg(windows)]
impl<T: IntoRawHandle> IntoRawFilelike for T {
    #[inline]
    fn into_raw_filelike(self) -> RawFilelike {
        self.into_raw_handle()
    }
}

#[cfg(windows)]
pub(crate) trait IntoRawSocketlike: IntoRawSocket {
    fn into_raw_socketlike(self) -> RawSocketlike;
}

#[cfg(windows)]
impl<T: IntoRawSocket> IntoRawSocketlike for T {
    #[inline]
    fn into_raw_socketlike(self) -> RawSocketlike {
        self.into_raw_socket()
    }
}

#[cfg(any(unix, target_os = "wasi"))]
pub(crate) trait FromRawFilelike: FromRawFd {
    unsafe fn from_raw_filelike(raw: RawFilelike) -> Self;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: FromRawFd> FromRawFilelike for T {
    #[inline]
    unsafe fn from_raw_filelike(raw: RawFilelike) -> Self {
        Self::from_raw_fd(raw)
    }
}

#[cfg(windows)]
pub(crate) trait FromRawFilelike: FromRawHandle {
    unsafe fn from_raw_filelike(raw: RawFilelike) -> Self;
}

#[cfg(windows)]
impl<T: FromRawHandle> FromRawFilelike for T {
    #[inline]
    unsafe fn from_raw_filelike(raw: RawFilelike) -> Self {
        Self::from_raw_handle(raw)
    }
}

#[cfg(any(unix, target_os = "wasi"))]
pub(crate) trait FromRawSocketlike: FromRawFd {
    unsafe fn from_raw_socketlike(raw: RawSocketlike) -> Self;
}

#[cfg(any(unix, target_os = "wasi"))]
impl<T: FromRawFd> FromRawSocketlike for T {
    #[inline]
    unsafe fn from_raw_socketlike(raw: RawSocketlike) -> Self {
        Self::from_raw_fd(raw)
    }
}

#[cfg(windows)]
pub(crate) trait FromRawSocketlike: FromRawSocket {
    unsafe fn from_raw_socketlike(raw: RawSocketlike) -> Self;
}

#[cfg(windows)]
impl<T: FromRawSocket> FromRawSocketlike for T {
    #[inline]
    unsafe fn from_raw_socketlike(raw: RawSocketlike) -> Self {
        Self::from_raw_socket(raw)
    }
}
