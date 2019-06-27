//! Advisory cross-platform file locks using file descriptors.
//!
//! Note that advisory lock compliance is opt-in, and can freely be ignored by other parties. This
//! means this crate __should not be relied on for security__, but solely used to coordinate file
//! access.
//!
//! ## Example
//! ```rust
//! use fd_lock::FdLock;
//! # use tempfile::tempfile;
//! # use std::io::prelude::*;
//! # use std::fs::File;
//!
//! # fn main() -> Result<(), failure::Error> {
//! // Lock a file and write to it.
//! let mut f = FdLock::new(tempfile()?);
//! f.lock()?.write_all(b"chashu cat")?;
//!
//! // Locks can also be held for extended durations.
//! let mut f = f.lock()?;
//! f.write_all(b"nori cat")?;
//! f.write_all(b"bird!")?;
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
