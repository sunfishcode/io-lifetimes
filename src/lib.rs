//! Advisory cross-platform file locks using file descriptors.
//!
//! Note that advisory lock compliance is opt-in, and can freely be ignored by other
//! parties. This means this crate __should not be used for security purposes__,
//! but solely to coordinate file access.
//!
//! ## Example
//! ```rust
//! use fd_lock::prelude::*;
//! use std::fs;
//! # use tempfile::tempdir;
//! # use std::io::prelude::*;
//!
//! # fn main() -> Result<(), failure::Error> {
//! # let dir = tempdir()?;
//! # let temp_path = dir.path().join("file.db");
//! // Create a new temporary file, and write data to it
//! let mut f = fs::File::create(&temp_path)?.lock_file()?;
//! f.write_all(b"chashu cat")?;
//!
//! // Opening another lock on the file is not allowed.
//! let mut lock = fs::File::open(&temp_path)?.lock_file();
//! assert!(lock.is_err());
//!
//! // But once the previous lock is dropped, we can acquire a new lock
//! drop(f);
//! let mut f = fs::File::open(&temp_path)?.lock_file()?;
//! let mut buf = vec![];
//! f.read_to_end(&mut buf)?;
//! assert_eq!(&buf, b"chashu cat");
//! # Ok(())}
//! ```

#![forbid(future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

mod error;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

pub use error::{Error, ErrorKind};
#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
pub use windows::*;

/// A prelude to lock files.
///
/// This is useful because it pulls in the right extensions for each platform without the chance of
/// naming collisions.
pub mod prelude {
    #[doc(inline)]
    #[cfg(unix)]
    pub use crate::unix::AsRawFdExt as _;
    #[doc(inline)]
    #[cfg(windows)]
    pub use crate::windows::AsRawHandleExt as _;
}
