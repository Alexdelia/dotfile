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

    /// answer yes to all questions (automatic symlink creation and update)
    /// if yes, will only update the update the symlink with update frequency of 'always' and of the current computer name
    /// will also supress all warnings
    #[clap(short, default_value = "false")]
    yes: bool,
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let Arg { file, yes } = Arg::parse();

    process::process(parse::parse(file, yes)?, !yes).into_diagnostic()?;

    Ok(())
}
