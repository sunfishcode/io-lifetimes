use std::mem::size_of;

#[cfg(unix)]
#[test]
fn test_assumptions() {
    assert_eq!(size_of::<std::os::unix::io::RawFd>(), size_of::<i32>());
}

#[cfg(windows)]
#[test]
fn test_assumptions() {
    assert_eq!(
        size_of::<std::os::windows::io::RawSocket>(),
        size_of::<winapi::um::winsock2::SOCKET>()
    );
    assert_eq!(winapi::um::winsock2::INVALID_SOCKET, usize::MAX);

    assert_ne!(
        winapi::um::handleapi::INVALID_HANDLE_VALUE,
        std::ptr::null_mut()
    );
}
