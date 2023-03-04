mod escape;
mod io;
mod symlink;

use miette::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    // dbg!(io::parse::read("symlink.toml")?);
    io::parse::read("some.toml")?;
    Ok(())
}
