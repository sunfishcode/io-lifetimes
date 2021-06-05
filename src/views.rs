//! Typed views using temporary objects.
//!
//! This module defines the return types for [`AsFilelike::as_filelike_view`]
//! and [`AsSocketlike::as_socketlike_view`].
//!
//! [`AsSocketlike::as_socketlike_view`]: crate::AsSocketlike::as_socketlike_view

use crate::portability::{AsRawFilelike, FromRawFilelike};
#[cfg(windows)]
use crate::{
    portability::{AsRawSocketlike, FromRawSocketlike},
    AsSocketlike, FromSocketlike, OwnedSocketlike,
};
use crate::{AsFilelike, FromFilelike, OwnedFilelike};
use std::fmt;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. These are returned by [`AsFilelike::as_filelike_view`].
pub struct FilelikeView<'owned, Target: FromFilelike> {
    /// The value to dereference to. It's wrapped in `ManuallyDrop` because
    /// this is a non-owning view over the underlying resource.
    target: ManuallyDrop<Target>,

    /// This field exists because we don't otherwise explicitly use `'owned`.
    _phantom: PhantomData<&'owned OwnedFilelike>,
}

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. These are returned by [`AsSocketlike::as_socketlike_view`].
///
/// [`AsSocketlike::as_socketlike_view`]: crate::AsSocketlike::as_socketlike_view
#[cfg(any(unix, target_os = "wasi"))]
pub type SocketlikeView<'owned, Target> = FilelikeView<'owned, Target>;

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. These are returned by [`AsSocketlike::as_socketlike_view`].
#[cfg(windows)]
pub struct SocketlikeView<'owned, Target: FromSocketlike> {
    /// The value to dereference to. It's wrapped in `ManuallyDrop` because
    /// this is a non-owning view over the underlying resource.
    target: ManuallyDrop<Target>,

    /// This field exists because we don't otherwise explicitly use `'owned`.
    _phantom: PhantomData<&'owned OwnedSocketlike>,
}

impl<Target: FromFilelike> FilelikeView<'_, Target> {
    /// Construct a temporary `Target` and wrap it in a `FilelikeView` object.
    #[inline]
    pub(crate) fn new<T: AsFilelike>(filelike: &T) -> Self {
        // Safety: The returned `FilelikeView` is scoped to the lifetime of
        // `filelike`, which we've borrowed immutably here, so the raw filelike
        // object will remain valid.
        let owned =
            unsafe { OwnedFilelike::from_raw_filelike(filelike.as_filelike().as_raw_filelike()) };
        Self {
            target: ManuallyDrop::new(Target::from_filelike(owned)),
            _phantom: PhantomData,
        }
    }
}

#[cfg(windows)]
impl<Target: FromSocketlike> SocketlikeView<'_, Target> {
    /// Construct a temporary `Target` and wrap it in a `SocketlikeView`
    /// object.
    #[inline]
    pub(crate) fn new<T: AsSocketlike>(socketlike: &T) -> Self {
        // Safety: The returned `SocketlikeView` is scoped to the lifetime of
        // `socketlike`, which we've borrowed immutably here, so the raw
        // socketlike object will remain valid.
        let owned = unsafe {
            OwnedSocketlike::from_raw_socketlike(socketlike.as_socketlike().as_raw_socketlike())
        };
        Self {
            target: ManuallyDrop::new(Target::from_socketlike(owned)),
            _phantom: PhantomData,
        }
    }
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
