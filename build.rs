fn main() {
    // Niche optimizations for `Borrowed*` and `Owned*` depend on `rustc_attrs`
    // which, outside of `std`, are only available on nightly.
    if let rustc_version::Channel::Nightly = rustc_version::version_meta()
        .expect("query rustc release channel")
        .channel
    {
        println!("cargo:rustc-cfg=rustc_attrs");
    }

    // Don't rerun this on changes other than build.rs, as we only depend on
    // the rustc version.
    println!("cargo:rerun-if-changed=build.rs");
}
