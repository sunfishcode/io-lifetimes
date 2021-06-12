#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

use io_lifetimes::{
    AsFilelike, AsSocketlike, FromFilelike, FromSocketlike, IntoFilelike, IntoSocketlike,
};

struct Tester {}
impl Tester {
    fn use_file<Filelike: io_lifetimes::AsFilelike>(filelike: Filelike) {
        let filelike = filelike.as_filelike();
        let _ = filelike.as_filelike_view::<std::fs::File>();
        let _ = dbg!(filelike);
    }

    fn use_socket<Socketlike: io_lifetimes::AsSocketlike>(socketlike: Socketlike) {
        let socketlike = socketlike.as_socketlike();
        let _ = socketlike.as_socketlike_view::<std::net::TcpStream>();
        let _ = dbg!(socketlike);
    }

    fn from_file<Filelike: io_lifetimes::IntoFilelike>(filelike: Filelike) {
        let filelike = filelike.into_filelike();
        let _ = filelike.as_filelike_view::<std::fs::File>();
        let _ = dbg!(&filelike);
        let _ = std::fs::File::from_filelike(filelike);
    }

    fn from_socket<Socketlike: io_lifetimes::IntoSocketlike>(socketlike: Socketlike) {
        let socketlike = socketlike.into_socketlike();
        let _ = socketlike.as_socketlike_view::<std::net::TcpStream>();
        let _ = dbg!(&socketlike);
        let _ = std::net::TcpStream::from_socketlike(socketlike);
    }

    fn from_into_file<Filelike: io_lifetimes::IntoFilelike>(filelike: Filelike) {
        let _ = std::fs::File::from_into_filelike(filelike);
    }

    fn from_into_socket<Socketlike: io_lifetimes::IntoSocketlike>(socketlike: Socketlike) {
        let _ = std::net::TcpStream::from_into_socketlike(socketlike);
    }
}

#[test]
fn test_api() {
    Tester::use_file(std::fs::File::open("Cargo.toml").unwrap().as_filelike());
    Tester::use_socket(
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .as_socketlike(),
    );

    Tester::from_file(std::fs::File::open("Cargo.toml").unwrap().into_filelike());
    Tester::from_socket(
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .into_socketlike(),
    );

    Tester::from_into_file(std::fs::File::open("Cargo.toml").unwrap().into_filelike());
    Tester::from_into_socket(
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .into_socketlike(),
    );
}
