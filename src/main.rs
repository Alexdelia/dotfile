mod ansi;
mod env;
mod parse;
use parse::parse;
mod process;
use process::process;
mod unix;

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
    let Arg { file, yes } = Arg::parse();

    process(parse(file, yes)?, !yes).into_diagnostic()?;

    Ok(())
}
