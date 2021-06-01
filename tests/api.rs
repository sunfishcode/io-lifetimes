#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

use io_experiment::{
    AsFilelike, AsSocketlike, FromFilelike, FromSocketlike, IntoFilelike, IntoSocketlike,
};

struct Tester {}
impl Tester {
    fn use_file<Filelike: io_experiment::AsFilelike>(filelike: Filelike) {
        let _ = filelike.as_filelike();
    }
    fn use_socket<Socketlike: io_experiment::AsSocketlike>(socketlike: Socketlike) {
        let _ = socketlike.as_socketlike();
    }

    fn from_file<Filelike: io_experiment::IntoFilelike>(filelike: Filelike) {
        let _ = std::fs::File::from_filelike(filelike.into_filelike());
    }
    fn from_socket<Socketlike: io_experiment::IntoSocketlike>(socketlike: Socketlike) {
        let _ = std::net::TcpStream::from_socketlike(socketlike.into_socketlike());
    }

    fn from_into_file<Filelike: io_experiment::IntoFilelike>(filelike: Filelike) {
        let _ = std::fs::File::from_into_filelike(filelike);
    }
    fn from_into_socket<Socketlike: io_experiment::IntoSocketlike>(socketlike: Socketlike) {
        let _ = std::net::TcpStream::from_into_socketlike(socketlike);
    }
}

#[test]
fn test_api() {
    let _ = Tester::use_file(std::fs::File::open("Cargo.toml").unwrap().as_filelike());
    let _ = Tester::use_socket(
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .as_socketlike(),
    );

    let _ = Tester::from_file(std::fs::File::open("Cargo.toml").unwrap().into_filelike());
    let _ = Tester::from_socket(
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .into_socketlike(),
    );

    let _ = Tester::from_into_file(std::fs::File::open("Cargo.toml").unwrap().into_filelike());
    let _ = Tester::from_into_socket(
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .into_socketlike(),
    );
}
