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
//!  - More advanced convenience features are possible in the future.

#![deny(missing_docs)]
#![feature(rustc_attrs)]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

mod portability;
mod traits;
mod types;
mod views;

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
pub use views::{AsFilelikeView, AsSocketlikeView, FilelikeView, SocketlikeView};

pub mod example_ffi;
