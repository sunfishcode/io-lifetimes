#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

use io_experiment::{
    AsBorrowedFilelike, AsBorrowedSocketlike, FromOwnedFilelike, FromOwnedSocketlike,
    IntoOwnedFilelike, IntoOwnedSocketlike,
};

struct Tester {}
impl Tester {
    fn use_file<Filelike: io_experiment::AsBorrowedFilelike>(filelike: Filelike) {
        let _ = filelike.as_borrowed_filelike();
    }
    fn use_socket<Socketlike: io_experiment::AsBorrowedSocketlike>(socketlike: Socketlike) {
        let _ = socketlike.as_borrowed_socketlike();
    }

    fn from_file<Filelike: io_experiment::IntoOwnedFilelike>(filelike: Filelike) {
        let _ = std::fs::File::from_owned_filelike(filelike.into_owned_filelike());
    }
    fn from_socket<Socketlike: io_experiment::IntoOwnedSocketlike>(socketlike: Socketlike) {
        let _ = std::net::TcpStream::from_owned_socketlike(socketlike.into_owned_socketlike());
    }

    fn from_into_file<Filelike: io_experiment::IntoOwnedFilelike>(filelike: Filelike) {
        let _ = std::fs::File::from_into_owned_filelike(filelike);
    }
    fn from_into_socket<Socketlike: io_experiment::IntoOwnedSocketlike>(socketlike: Socketlike) {
        let _ = std::net::TcpStream::from_into_owned_socketlike(socketlike);
    }
}

#[test]
fn test_api() {
    let _ = Tester::use_file(
        std::fs::File::open("Cargo.toml")
            .unwrap()
            .as_borrowed_filelike(),
    );
    let _ = Tester::use_socket(
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .as_borrowed_socketlike(),
    );

    let _ = Tester::from_file(
        std::fs::File::open("Cargo.toml")
            .unwrap()
            .into_owned_filelike(),
    );
    let _ = Tester::from_socket(
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .into_owned_socketlike(),
    );

    let _ = Tester::from_into_file(
        std::fs::File::open("Cargo.toml")
            .unwrap()
            .into_owned_filelike(),
    );
    let _ = Tester::from_into_socket(
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .into_owned_socketlike(),
    );
}
