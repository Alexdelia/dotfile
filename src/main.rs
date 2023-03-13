mod ansi;
mod parse;
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

    let p = parse::parse(file)?;
    dbg!(&p);

    Ok(())
}
