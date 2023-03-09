mod ansi;
mod io;
mod symlink;

use miette::Result;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap()]
struct Opts {
    /// file that define the symlinks
    #[clap(default_value = symlink::DEFAULT_SYMLINK_FILE)]
    file: String,
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let opts: Opts = Opts::parse();
    dbg!(opts);

    // if std::env::args().len() > 2 {
    //     eprintln!("usage: symlink [file]");
    //     std::process::exit(1);
    // }

    dbg!(io::parse::parse("symlink.toml")?);
    dbg!(io::parse::parse("some.toml")?);
    Ok(())
}
