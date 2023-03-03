// mod symlink;
mod io;

use miette::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    // dbg!(parse::read("symlink.toml")?);
    io::parse::read("no.toml")?;
    Ok(())
}
