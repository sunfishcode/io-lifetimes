This library introduces `OwnedFd`, `BorrowedFd`, and supporting types and
traits, and corresponding features for Windows, which implement safe owning
and borrowing I/O lifetime patterns.

This is associated with [RFC 3128], the I/O Safety RFC, which proposes that
this API be added to `std`, with the goal being to eventually replace `RawFd`
etc. for most use cases.

Some features currently require nightly Rust, as they depend on `rustc_attrs`
to perform niche optimizations needed for FFI use cases.

For a quick taste, check out the code examples:

 - [hello], a basic demo of this API, doing low-level I/O manually, using the
   [provided example FFI bindings]
 - [easy-conversions], demonstrating the `from_into` convenience feature for
   converting from an `impl Into*` into an `impl From*`.
 - [portable-views], demonstrating the convenience feature which allows one
   to temporarily "view" a file descriptor as any owning type such as `File`

[hello]: https://github.com/sunfishcode/io-lifetimes/blob/main/examples/hello.rs
[easy-conversions]: https://github.com/sunfishcode/io-lifetimes/blob/main/examples/easy-conversions.rs
[portable-views]: https://github.com/sunfishcode/io-lifetimes/blob/main/examples/portable-views.rs
[provided example FFI bindings]: https://github.com/sunfishcode/io-lifetimes/blob/main/src/example_ffi.rs

The core of the API is very simple, and consists of two main types and three
main traits:

```rust
pub struct BorrowedFd<'fd> { ... }
pub struct OwnedFd { ... }

pub trait AsFd<'fd> { ... }
pub trait IntoFd { ... }
pub trait FromFd { ... }

impl<'fd> AsRawFd for BorrowedFd<'fd> { ... }
impl AsRawFd for OwnedFd { ... }
impl IntoRawFd for OwnedFd { ... }
impl FromRawFd for OwnedFd { ... }

impl Drop for OwnedFd { ... }

impl<'fd> AsFd<'fd> for BorrowedFd<'fd> { ... }
impl<'fd> AsFd<'fd> for &'fd OwnedFd { ... }
impl IntoFd for OwnedFd { ... }
impl FromFd for OwnedFd { ... }
```

On Windows, there are `Handle` and `Socket` versions of every `Fd` thing, and
a special `OptionFileHandle` type to cope with inconsistent error reporting
in the Windows API.

Full API documentation:
 - [for `x86_64-unknown-linux-gnu`](https://io-experiment.sunfishcode.online/x86_64-unknown-linux-gnu/io_experiment/index.html)
 - [for `x86_64-pc-windows-msvc`](https://io-experiment.sunfishcode.online/x86_64-pc-windows-msvc/io_experiment/index.html)

## The magic of transparency

Here's the fun part. `BorrowedFd` and `OwnedFd` are `repr(transparent)` and
hold `RawFd` values, and `Option<BorrowedFd>` and `Option<OwnedFd>` are
FFI-safe (on nightly Rust), so they can all be used in FFI [directly]:

[directly]: https://github.com/sunfishcode/io-lifetimes/blob/main/src/example_ffi.rs

```rust
extern "C" {
    pub fn open(pathname: *const c_char, flags: c_int, ...) -> Option<OwnedFd>;
    pub fn read(fd: BorrowedFd<'_>, ptr: *mut c_void, size: size_t) -> ssize_t;
    pub fn write(fd: BorrowedFd<'_>, ptr: *const c_void, size: size_t) -> ssize_t;
    pub fn close(fd: OwnedFd) -> c_int;
}
```

With bindings like this, users never have to touch `RawFd` values. Of course,
not all code will do this, but it is a fun feature for code that can. This
is what motivates having `BorrowedFd` instead of just using `&OwnedFd`.

Note the use of `Option<OwnedFd>` as the return value of `open`, representing
the fact that it can either succeed or fail.

## Prior Art

There are several similar crates: [fd](https://crates.io/crates/fd),
[filedesc](https://crates.io/crates/filedesc),
[filedescriptor](https://crates.io/crates/filedescriptor),
[owned-fd](https://crates.io/crates/owned-fd), and
[unsafe-io](https://crates.io/crates/unsafe-io).

Some of these provide additional features such as the ability to create pipes
or sockets, to get and set flags, and to do read and write operations.
io-lifetimes omits these features, leaving them to to be provided as separate
layers on top.

Most of these crates provide ways to duplicate a file descriptor. io-lifetimes
currently treats this as another feature than can be provided by a layer on
top, though if there are use cases where this is a common operation, it could
be added.

io-lifetimes's distinguishing features are its use of `repr(transparent)`
to support direct FFI usage, niche optimizations so `Option` can support direct
FFI usafe as well (on nightly Rust), lifetime-aware `As*`/`Into*`/`From*`
traits which leverage Rust's lifetime system and allow safe and checked
`from_*` and `as_*`/`into_*` functions, and powerful convenience features
enabled by its underlying safety.

io-lifetimes also has full Windows support, as well as Unix/Windows
portability abstractions, covering both file-like and socket-like types.

io-lifetimes's [`OwnedFd`] type is similar to
[fd](https://crates.io/crates/fd)'s
[`FileDesc`](https://docs.rs/fd/0.2.3/fd/struct.FileDesc.html). io-lifetimes
doesn't have a `close_on_drop` parameter, and instead uses [`OwnedFd`] and
[`BorrowedFd`] to represent dropping and non-dropping handles, respectively, in
a way that is checked at compile time rather than runtime.

io-lifetimes's [`OwnedFd`] type is also similar to
[filedesc](https://crates.io/crates/filedesc)'s
[`FileDesc`](https://docs.rs/filedesc/0.3.0/filedesc/struct.FileDesc.html)
io-lifetimes's `OwnedFd` reserves the value -1, so it doesn't need to test for
`-1` in its `Drop`, and `Option<OwnedFd>` is the same size as `OwnedFd` (on
nightly Rust).

io-lifetimes's [`OwnedFd`] type is also similar to
[owned-fd](https://crates.io/crates/owned-fd)'s
[`OwnedFd`](https://docs.rs/owned-fd/0.1.0/owned_fd/struct.OwnedFd.html).
io-lifetimes doesn't implement `Clone`, because duplicating a file descriptor
can fail due to OS process limits, while `Clone` is an infallible interface.

io-lifetimes's [`BorrowedFd`] is similar to
[owned-fd](https://crates.io/crates/owned-fd)'s
[`FdRef`](https://docs.rs/owned-fd/0.1.0/owned_fd/struct.FdRef.html), except it
uses a lifetime parameter and `PhantomData` rather than transmuting a raw file
descriptor value into a reference value.

io-lifetimes's convenience features are similar to those of
[unsafe-io](https://crates.io/crates/unsafe-io), but io-lifetimes is built on
its own `As*`/`Into*`/`From*` traits, rather than extending
`AsRaw*`/`IntoRaw*`/`FromRaw*` with
[`OwnsRaw`](https://docs.rs/unsafe-io/0.6.9/unsafe_io/trait.OwnsRaw.html), so
they're simpler and safer to use. io-lifetimes doesn't include unsafe-io's
`*ReadWrite*` or `*HandleOrSocket*` abstractions, and leaves these as features
to be provided by separate layers on top.

[`OwnedFd`]: https://io-experiment.sunfishcode.online/x86_64-unknown-linux-gnu/io_experiment/struct.OwnedFd.html
[`BorrowedFd`]: https://io-experiment.sunfishcode.online/x86_64-unknown-linux-gnu/io_experiment/struct.BorrowedFd.html
[RFC 3128]: https://github.com/rust-lang/rfcs/pull/3128
