# fd-lock
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Advisory cross-platform lock on a file using a file descriptor to it. Adapted
from [mafintosh/fd-lock].

[mafintosh/fd-lock]: https://github.com/mafintosh/fd-lock

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
```rust
// tbi
```

## Installation
```sh
$ cargo add fd-lock
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
- [Windows Dev Center: LockFile function](https://docs.microsoft.com/en-us/windows/desktop/api/fileapi/nf-fileapi-lockfile)
- [flock(2) - Linux Man Page](https://linux.die.net/man/2/flock)
- [`libc::flock`](https://docs.rs/libc/0.2.58/libc/struct.flock.html)
- [`winapi::um::fileapi::LockFile`](https://docs.rs/winapi/0.3.7/x86_64-pc-windows-msvc/winapi/um/fileapi/fn.LockFile.html)

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/fd-lock.svg?style=flat-square
[2]: https://crates.io/crates/fd-lock
[3]: https://img.shields.io/travis/yoshuawuyts/fd-lock/master.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/fd-lock
[5]: https://img.shields.io/crates/d/fd-lock.svg?style=flat-square
[6]: https://crates.io/crates/fd-lock
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/fd-lock

[releases]: https://github.com/yoshuawuyts/fd-lock/releases
[contributing]: https://github.com/yoshuawuyts/fd-lock/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/fd-lock/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/fd-lock/labels/help%20wanted
