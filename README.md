This is an experiment, associated with [RFC 3128].

It requires nightly Rust, so that it can use `rustc_attrs` outside of `std`
for now.

For a quick taste, check out the code examples:

 - [hello], a basic demo of this API, doing low-level I/O manually, using the
   [provided example FFI bindings]
 - [easy-conversions], demonstrating the `from_into` convenience feature for
   converting from an `impl Into*` into an `impl From*`.
 - [portable-views], demonstrating the convenience feature which allows one
   to temporarily "view" a file descriptor as any owning type such as `File`

[hello]: https://github.com/sunfishcode/io-experiment/blob/main/examples/hello.rs
[easy-conversions]: https://github.com/sunfishcode/io-experiment/blob/main/examples/easy-conversions.rs
[portable-views]: https://github.com/sunfishcode/io-experiment/blob/main/examples/portable-views.rs
[provided example FFI bindings]: https://github.com/sunfishcode/io-experiment/blob/main/src/example_ffi.rs

The core of the API is very simple, and consists of two main types and three
main traits:

```rust
pub struct BorrowedFd<'owned> { ... }
pub struct OwnedFd { ... }

pub trait AsFd { ... }
pub trait IntoFd { ... }
pub trait FromFd { ... }

impl<'owned> AsRawFd for BorrowedFd<'owned> { ... }
impl AsRawFd for OwnedFd { ... }
impl IntoRawFd for OwnedFd { ... }
impl FromRawFd for OwnedFd { ... }

impl Drop for OwnedFd { ... }

impl<'owned> AsFd for BorrowedFd<'owned> { ... }
impl AsFd for OwnedFd { ... }
impl IntoFd for OwnedFd { ... }
impl FromFd for OwnedFd { ... }
```

On Windows, there are `Handle` and `Socket` versions of every `Fd` thing, and
a special `OptionFileHandle` type to cope with inconsistent error reporting
in the Windows API.

See the [full API documentation here](https://io-experiment.sunfishcode.online/io_experiment/index.html).

## The magic of transparency

Here's the fun part. `BorrowedFd` and `OwnedFd` are `repr(transparent)` and
hold `RawFd` values, and `Option<BorrowedFd>` and `Option<OwnedFd>` are
FFI-safe, so they can all be used in FFI [directly]:

[directly]: https://github.com/sunfishcode/io-experiment/blob/main/src/example_ffi.rs

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

[RFC 3128]: https://github.com/rust-lang/rfcs/pull/3128
