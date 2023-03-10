mod ansi;
mod io;
mod symlink;

use miette::Result;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap()]
struct Arg {
    /// file that define the symlinks
    #[clap(default_value = symlink::DEFAULT_SYMLINK_FILE)]
    file: String,
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let Arg { file } = Arg::parse();

    dbg!(io::parse::parse(file)?);

    Ok(())
}
