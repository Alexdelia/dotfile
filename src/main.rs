mod escape;
mod io;
mod symlink;

use miette::Result;

fn main() -> Result<()> {
    println!("Hello, world!");
    dbg!(io::parse::test("symlink.toml")?);
    Ok(())
}
