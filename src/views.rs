use crate::portability::{AsRawFilelike, AsRawSocketlike, FromRawFilelike, FromRawSocketlike};
use crate::{
    AsFilelike, AsSocketlike, FromFilelike, FromSocketlike, OwnedFilelike, OwnedSocketlike,
};
use std::fmt;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. This trait can be used with any type which the platform
/// represents in a manner similar to files, which on Unix is any OS
/// resource, and on Windows is things like files, processes, and pipes.
pub trait AsFilelikeView {
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

impl<T: AsFilelike> AsFilelikeView for T {
    #[inline]
    fn as_filelike_view<Target: FromFilelike>(&self) -> FilelikeView<'_, Target> {
        // Safety: The returned `FilelikeView` is scoped to the lifetime of
        // `self`, which we've borrowed immutably here, so the raw filelike will
        // remain valid.
        let owned =
            unsafe { OwnedFilelike::from_raw_filelike(self.as_filelike().as_raw_filelike()) };
        FilelikeView {
            target: ManuallyDrop::new(Target::from_filelike(owned)),
            _phantom: PhantomData,
        }
    }
}

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. This trait can be used with any type which the platform
/// represents in a manner similar to sockets, which on Unix is any OS
/// resource, and on Windows is just sockets.
pub trait AsSocketlikeView {
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

impl<T: AsSocketlike> AsSocketlikeView for T {
    #[inline]
    fn as_socketlike_view<Target: FromSocketlike>(&self) -> SocketlikeView<'_, Target> {
        // Safety: The returned `SocketlikeView` is scoped to the lifetime of
        // `self`, which we've borrowed immutably here, so the raw socketlike
        // will remain valid.
        let owned = unsafe {
            OwnedSocketlike::from_raw_socketlike(self.as_socketlike().as_raw_socketlike())
        };
        SocketlikeView {
            target: ManuallyDrop::new(Target::from_socketlike(owned)),
            _phantom: PhantomData,
        }
    }
}

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`.
pub struct FilelikeView<'owned, Target: FromFilelike> {
    /// The value to dereference to. It's wrapped in `ManuallyDrop` because
    /// this is a non-owning view over the underlying resource.
    target: ManuallyDrop<Target>,

    /// This field exists because we don't otherwise explicitly use `'owned`.
    _phantom: PhantomData<&'owned OwnedFilelike>,
}

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`.
#[cfg(any(unix, target_os = "wasi"))]
pub type SocketlikeView<'owned, Target> = FilelikeView<'owned, Target>;

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`.
#[cfg(windows)]
pub struct SocketlikeView<'owned, Target: FromSocketlike> {
    /// The value to dereference to. It's wrapped in `ManuallyDrop` because
    /// this is a non-owning view over the underlying resource.
    target: ManuallyDrop<Target>,

    /// This field exists because we don't otherwise explicitly use `'owned`.
    _phantom: PhantomData<&'owned OwnedSocketlike>,
}

impl<Target: FromFilelike> Deref for FilelikeView<'_, Target> {
    type Target = Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

#[cfg(windows)]
impl<Target: FromSocketlike> Deref for SocketlikeView<'_, Target> {
    type Target = Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl<Target: FromFilelike> DerefMut for FilelikeView<'_, Target> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.target
    }
}

#[cfg(windows)]
impl<Target: FromSocketlike> DerefMut for SocketlikeView<'_, Target> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.target
    }
}

impl<Target: FromFilelike + fmt::Debug> fmt::Debug for FilelikeView<'_, Target> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilelikeView")
            .field("target", &*self)
            .finish()
    }
}

#[cfg(windows)]
impl<Target: FromSocketlike + fmt::Debug> fmt::Debug for SocketlikeView<'_, Target> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SocketlikeView")
            .field("target", &*self)
            .finish()
    }
}
