mod ansi;
mod io;
mod symlink;

use miette::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    dbg!(io::parse::parse("symlink.toml")?);
    dbg!(io::parse::parse("some.toml")?);
    Ok(())
}
