mod ansi;
mod io;
mod symlink;

use miette::Result;

fn main() -> Result<()> {
    println!("Hello, world!");

    if std::env::args().len() > 2 {
        eprintln!("usage: symlink [file]");
        std::process::exit(1);
    }

    // if 1 arg
    let file = if let Some(file) = std::env::args().next() {
        file
    } else {
        symlink::DEFAULT_SYMLINK_FILE.to_string()
    };

    dbg!(io::parse::parse("symlink.toml")?);
    dbg!(io::parse::parse("some.toml")?);
    Ok(())
}
