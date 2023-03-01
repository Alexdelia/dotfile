mod symlink;

fn main() {
    println!("Hello, world!");
    symlink::symlink(symlink::Symlink {
        path: std::path::Path::new("test"),
        target: std::path::Path::new("test"),
    });
}
