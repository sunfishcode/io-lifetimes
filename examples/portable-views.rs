use io_experiment::AsFilelikeView;
use std::fs::File;
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    let stdout = stdout();

    // With `AsFilelikeView`, any type implementing
    // `AsBorrowedFd`/`AsBorrowedHandle` can be viewed as any type
    // supporting `FromOwnedFilelike`, so you can call `File` methods
    // on `Stdout` or other things.
    let metadata = stdout.as_filelike_view::<File>().metadata()?;

    if metadata.is_file() {
        println!("stdout is a file!");
    } else {
        println!("stdout is not a file!");
    }

    Ok(())
}
