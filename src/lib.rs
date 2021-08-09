//! Experimental new types and traits to replace the `Raw` family of types and
//! traits.
//!
//! This API has much conceptual similarity with the `Raw` API, but introduces
//! explicit concepts of ownership and borrowing:
//!
//! | `Raw` API  | This experimental API    |
//! | ---------- | ------------------------ |
//! | `Raw*`     | `Borrowed*` and `Owned*` |
//! | `AsRaw*`   | `As*`                    |
//! | `IntoRaw*` | `Into*`                  |
//! | `FromRaw*` | `From*`                  |
//!
//! This gives it several advantages:
//!
//!  - Less `unsafe` in user code!
//!
//!  - Easier to understand ownership.
//!
//!  - It avoids the inconsistency where `AsRawFd` and `IntoRawFd` return
//!    `RawFd` values that users ought to be able to trust, but aren't unsafe,
//!    so it's possible to fail to uphold this trust in purely safe Rust.
//!
//!  - It enables a number of safe and portable convenience features, such as
//!    [safe typed views] and [from+into conversions].
//!
//! [safe typed views]: AsFilelike::as_filelike_view
//! [from+into conversions]: FromFilelike::from_into_filelike

#![deny(missing_docs)]
#![cfg_attr(rustc_attrs, feature(rustc_attrs))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

mod portability;
mod traits;
mod types;

#[cfg(any(unix, target_os = "wasi"))]
pub use traits::{AsFd, FromFd, IntoFd};
#[cfg(windows)]
pub use traits::{AsHandle, AsSocket, FromHandle, FromSocket, IntoHandle, IntoSocket};

#[cfg(any(unix, target_os = "wasi"))]
pub use types::{BorrowedFd, OwnedFd};
#[cfg(windows)]
pub use types::{BorrowedHandle, BorrowedSocket, OptionFileHandle, OwnedHandle, OwnedSocket};

pub use portability::{
    AsFilelike, AsSocketlike, BorrowedFilelike, BorrowedSocketlike, FromFilelike, FromSocketlike,
    IntoFilelike, IntoSocketlike, OwnedFilelike, OwnedSocketlike,
};

pub mod example_ffi;
pub mod views;

// Ideally, we'd want crates to implement our traits themselves. But for now,
// while we're prototyping, we provide a few impls on foreign types.
#[cfg(feature = "async-std")]
mod impls_async_std;
#[cfg(feature = "fs_err")]
mod impls_fs_err;
#[cfg(feature = "mio")]
mod impls_mio;
#[cfg(feature = "os_pipe")]
mod impls_os_pipe;
#[cfg(feature = "socket2")]
mod impls_socket2;
#[cfg(feature = "tokio")]
mod impls_tokio;
