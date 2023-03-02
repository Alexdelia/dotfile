mod symlink;

fn main() -> Result<(), &'static str> {
    println!("Hello, world!");
    err()?;
    // symlink::symlink(symlink::Symlink {
    //     path: std::path::Path::new("test"),
    //     target: std::path::Path::new("test"),
    // });
    Ok(())
}

fn err() -> Result<u8, &'static str> {
    Err("this is an error")
}
