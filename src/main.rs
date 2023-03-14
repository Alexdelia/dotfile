mod ansi;
mod env;
mod parse;
mod process;

use miette::{IntoDiagnostic, Result};

use clap::Parser;

const DEFAULT_SYMLINK_FILE: &str = "symlink.toml";

#[derive(Debug, Parser)]
#[clap()]
struct Arg {
    /// file that define the symlinks
    #[clap(default_value = DEFAULT_SYMLINK_FILE)]
    file: String,
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let Arg { file } = Arg::parse();

    process::process(parse::parse(file)?).into_diagnostic()?;

    Ok(())
}
