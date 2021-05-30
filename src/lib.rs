//! Experimental new types and traits to replace the `Raw` family of types and
//! traits.
//!
//! This API has much conceptual similarity with the `Raw` API, but introduces
//! explicit concepts of ownership and borrowing:
//!
//! | `Raw` API  | This experimental API    |
//! | ---------- | ------------------------ |
//! | `Raw*`     | `Borrowed*` and `Owned*` |
//! | `AsRaw*`   | `AsBorrowed*`            |
//! | `IntoRaw*` | `IntoOwned*`             |
//! | `FromRaw*` | `FromOwned*`             |
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

mod traits;
mod types;

#[cfg(unix)]
pub use traits::{AsBorrowedFd, FromOwnedFd, IntoOwnedFd};
#[cfg(windows)]
pub use traits::{
    AsBorrowedHandle, AsBorrowedSocket, FromOwnedHandle, FromOwnedSocket, IntoOwnedHandle,
    IntoOwnedSocket,
};

#[cfg(unix)]
pub use types::{BorrowedFd, OptionFd, OwnedFd};
#[cfg(windows)]
pub use types::{
    BorrowedHandle, BorrowedSocket, OptionHandle, OptionSocket, OwnedHandle, OwnedSocket,
};
