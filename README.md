This is an experiment.

Here's an API summary. `OptionFd` is a `RawFd` which may be either valid or -1.
For Windows, there are `Handle` and `Socket` versions of every `Fd` thing.

```rust
pub struct BorrowedFd<'owned> { ... }
pub struct OwnedFd { ... }
pub struct OptionFd { ... }

pub trait AsBorrowedFd { ... }
pub trait IntoOwnedFd { ... }
pub trait FromOwnedFd { ... }

impl TryFrom<OptionFd> for OwnedFd { ... }
impl From<OwnedFd> for OptionFd { ... }

impl<'owned> AsRawFd for BorrowedFd<'owned> { ... }
impl AsRawFd for OwnedFd { ... }
impl IntoRawFd for OwnedFd { ... }
impl FromRawFd for OwnedFd { ... }
impl FromRawFd for OptionFd { ... }

impl Drop for OwnedFd { ... }
impl Drop for OptionFd { ... }

impl<'owned> AsBorrowedFd for BorrowedFd<'owned> { ... }
impl AsBorrowedFd for OwnedFd { ... }
impl IntoOwnedFd for OwnedFd { ... }
impl FromOwnedFd for OwnedFd { ... }
impl FromOwnedFd for OptionFd { ... }
```

Here's the fun part. `BorrowedFd` and `OptionFd` are `repr(transparent)` and
hold `RawFd` values, so they can be used in FFI directly:

```rust
extern "C" {
    pub fn open(pathname: *const u8, flags: c_int, ...) -> OptionFd;
    pub fn write(fd: BorrowedFd, ptr: *const u8, size: size_t) -> isize;
    pub fn close(fd: OptionFd) -> c_int;
}
```

With bindings like this, users never have to touch `RawFd` values. Of course,
not all code will do this, but it is a fun feature for code that can. This
is what motivates having `BorrowedFd` instead of just using `&OwnedFd`.

Note the use of `OptionFd` as the return value of `open`, representing the
fact that it can either succeed or fail.

And as an even more subtle detail, `close` also takes an `OptionFd`, even
though the main use case is to pass it an owned fd. This is because `OwnedFd`
is not `repr(transparent)` so it can do niche optimizations. And there's a
sense in which `OptionFd` is technically accurate, since `close` does have
defined behavior if passed -1.
